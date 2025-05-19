use crate::anki_connect::card::CardId;
use crate::anki_connect::deck::DeckId;
use crate::anki_connect::{
    AnkiConnectClient, ClientBehavior, error::APIError, model::Model, note::NoteId,
    notes_client::NoteOperation,
};
use crate::callout::Callout;
use crate::cli::SyncArgs;
use crate::deck::Deck;
use crate::find_markdown_files::find_markdown_files;
use crate::model::InternalModelMethods;
use crate::model::ModelType;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
use std::collections::HashMap;
use std::fs::{File, read_to_string};
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;
use std::thread;
use tracing::{debug, error, info, warn};

use rayon::prelude::*;

use crate::error::M2AnkiError;
use crate::progress::{LOOKING_GLASS, SPARKLE, print_step};

pub fn sync(args: SyncArgs) -> Result<(), M2AnkiError> {
    // Extract args into variables
    let parent_deck = args.deck.unwrap().to_string();
    let model_type_name = args.model_type_name.unwrap().to_string();
    let model_name = args
        .model_name
        .unwrap_or_else(|| format!("md2anki {}", &model_type_name));
    let header_lang: Option<String> = Some(args.header_lang.clone().unwrap().to_string());
    let input_dir = &args.input_dir;

    print_step(1, 10, Some("Connecting to Anki"), Some(LOOKING_GLASS));

    // Create a client with default connection (localhost:8765)
    let client = AnkiConnectClient::new(None, None);

    // Prepare channel for async initial processing
    let (tx, rx) = std::sync::mpsc::channel();

    // Test client connection
    let client_clone = client.clone();
    let tx_client = tx.clone();
    let client_handle = thread::spawn(move || {
        let res: bool = client_clone.test_connection().unwrap_or(false);
        tx_client.send(("client", res));
    });

    let input_dir_clone = args.input_dir.clone();
    let tx_files = tx.clone();
    let markdown_files_hadle = thread::spawn(move || {
        let markdown_files = find_markdown_files(&input_dir_clone).unwrap_or_else(|_| Vec::new());
        let found_files = !markdown_files.is_empty();
        tx_files.send(("md_files", found_files));

        info!("Found {} markdown files", &markdown_files.len());

        print_step(2, 10, Some("Extracting decks"), Some(LOOKING_GLASS));

        let decks: Vec<Deck> = markdown_files
            .par_iter()
            .map(Deck::try_from)
            .filter(Result::is_ok)
            .map(Result::unwrap)
            .filter(|deck| !deck.callouts.is_empty())
            .collect();

        let total_callouts: usize = decks.par_iter().map(|deck| deck.callouts.len()).sum();
        tx_files.send(("num_callouts", total_callouts > 0));

        let num_found_decks: usize = decks.len();
        let num_total_callouts: usize = decks.par_iter().map(|d| d.callouts.len()).sum();

        // Display errors for callouts that couldn't be parsed
        let failed_decks: Vec<&Deck> = decks
            .par_iter()
            .filter(|deck| !deck.failed.is_empty())
            .collect();
        if !failed_decks.is_empty() {
            for deck in failed_decks {
                let mut err_msg = Vec::with_capacity(deck.failed.len() + 1);
                err_msg.push(format!(
                    "Callout parsing errors in deck: '{}'\n",
                    &deck.source_file.to_str().unwrap_or_default()
                ));
                for (callout_string, err) in &deck.failed {
                    err_msg.push(format!(
                        "{:?}:\n{}\n{}\n",
                        err,
                        callout_string,
                        "=".repeat(80),
                    ));
                }
                warn!("{}", err_msg.join("\n"));
            }
        }
        info!(
            "Found {} decks with a total of {} callouts",
            num_found_decks, num_total_callouts
        );

        let model_type = ModelType::from_str(&model_type_name);

        // Load css file if it exists
        let css_file = args.css_file.clone().unwrap_or_default();
        let css = if css_file.is_file() {
            read_to_string(css_file)
        } else {
            Ok("".to_string())
        };

        (decks, total_callouts, model_type, css)
    });

    for _ in 0..3 {
        match rx.recv() {
            Ok(("client", false)) => {
                error!("Cannot connect to Anki. Make sure it is running.");
                return Err(M2AnkiError::APIError(APIError::FailedConnection(
                    "Cannot connect to Anki. Make sure it is running.".to_string(),
                )));
            }
            Ok(("md_files", false)) => {
                warn!(
                    "Failed to find any markdown files in: '{}'",
                    input_dir.to_str().unwrap()
                );
                return Ok(());
            }
            Ok(("num_callouts", false)) => {
                warn!(
                    "No callouts found in any of the markdown files in: '{}'",
                    input_dir.to_str().unwrap()
                );
                return Ok(());
            }
            _ => continue,
        }
    }

    client_handle.join().map_err(|_| M2AnkiError::ThreadPanic)?;
    let (decks, total_callouts, model_type, css) = markdown_files_hadle
        .join()
        .map_err(|_| M2AnkiError::ThreadPanic)?;

    let model_type = model_type?;
    let css = css?;

    let mut created_model = false;

    let note_type: Model = match client.models().find_by_name(vec![&model_name]) {
        Ok(models) => {
            if models.is_empty() {
                let new_model = client
                    .models()
                    .create_model(model_type.to_create_model(&model_name, Some(&css)))?;
                created_model = true;
                new_model
            } else {
                models.first().unwrap().to_owned()
            }
        }
        Err(_) => {
            let new_model = client
                .models()
                .create_model(model_type.to_create_model(&model_name, Some(&css)))?;
            created_model = true;
            new_model
        }
    };

    if !css.is_empty() && !created_model {
        let _ = client
            .models()
            .update_model_styling(&note_type.name, css.as_str());
        info!("Updated model CSS.");
    }

    let mut failed_notes: Vec<(PathBuf, Vec<(String, ModelType)>)> = Vec::new();
    let mut num_added_total = 0usize;

    // Delete the deck and re-create it for testing purposes
    if args.delete_existing {
        let _ = client.decks().delete(&parent_deck);
    } else if client.decks().find_deck_id_by_name(&parent_deck).is_ok() {
        let mut notes_in_deck = client.notes().get_notes_by_deck_name(&parent_deck)?;
        let deck_names_and_ids: HashMap<String, DeckId> = client
            .decks()
            .deck_names_and_ids()?
            .into_par_iter()
            .filter(|(name, id)| name.starts_with(&parent_deck))
            .collect();
        let all_cards: Vec<&CardId> = notes_in_deck
            .par_iter()
            .map(|note| &note.cards)
            .flatten()
            .collect();
        let anki_decks = client.decks().get_decks(&all_cards)?;
        let card_ids_to_deck: HashMap<&CardId, &str> = anki_decks
            .par_iter()
            .map(|(name, cards)| cards.par_iter().map(|card| (card, name.as_str())))
            .flatten()
            .collect();

        let card_to_note: HashMap<&CardId, &String> = notes_in_deck
            .par_iter()
            .map(|note| (note.cards.first().unwrap(), &note.markdown_id))
            .collect();

        let note_to_deck: HashMap<&str, &str> = anki_decks
            .par_iter()
            .map(|(name, cards)| {
                (
                    card_to_note.get(cards.first().unwrap()).unwrap().as_str(),
                    name.as_str(),
                )
            })
            .collect();
        dbg!(&note_to_deck);
        // dbg!(&anki_decks);
        // dbg!(&deck_names_and_ids);
        // dbg!(&cards_in_deck);
        // Prepare hashmap for faster card lookup
        let callouts_map: HashMap<&String, &Callout> = decks
            .par_iter()
            .flat_map(|deck| {
                deck.callouts
                    .par_iter()
                    .map(|callout| (&callout.markdown_id, callout))
            })
            .collect();

        notes_in_deck.par_iter_mut().for_each(|note| {
            if !callouts_map.contains_key(&note.markdown_id) {
                note.operation = NoteOperation::Add;
            }
            // else if
        });

        dbg!(&notes_in_deck);

        // TODO: identify how notes have changed:
        //      - new note -> note should be added
        //      - updated note -> note should be updated
        //      - note exists, but in different file -> update deck
        //      - note doesn't exist anymore -> delete note
    }

    // Prepare progress bars
    let m = MultiProgress::new();
    let sty = ProgressStyle::with_template(
        "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
    )
    .unwrap()
    .progress_chars("##-");
    let global_pbar = m.add(ProgressBar::new(
        total_callouts
            .try_into()
            .map_err(|_| M2AnkiError::ProgressBarError)?,
    ));
    global_pbar.set_style(sty.clone());
    global_pbar.set_message("Overall");
    let decks_pbar = m.add(ProgressBar::new(
        decks
            .len()
            .try_into()
            .map_err(|_| M2AnkiError::ProgressBarError)?,
    ));
    decks_pbar.set_style(sty.clone());
    decks_pbar.set_message("Decks");

    // Start main upload loop
    print_step(4, 10, Some("Adding notes to Anki"), None);
    for deck in decks {
        let new_notes: Vec<ModelType> = deck
            .callouts
            .par_iter()
            .map(|callout| model_type.from_callout(callout, header_lang.as_deref()))
            .collect();

        let deck_name = deck.get_qualified_name(Some(input_dir), Some(&parent_deck))?;
        let _ = client.decks().find_or_create_deck(deck_name.as_str())?;

        // Add the notes to the deck
        let current_deck_pb = m.add(ProgressBar::new(
            new_notes
                .len()
                .try_into()
                .map_err(|_| M2AnkiError::ProgressBarError)?,
        ));
        current_deck_pb.set_style(sty.clone());
        current_deck_pb.set_message(deck_name.clone());

        let mut note_id: NoteId = NoteId(0);
        let mut num_added = 0usize;
        let mut failed_in_deck: Vec<(String, ModelType)> =
            Vec::with_capacity(deck.callouts.len() / 2);

        for note in new_notes {
            match client
                .notes()
                .add_note(note.to_add_note(&deck_name, &model_name))
            {
                Ok(id) => {
                    note_id = id;
                    num_added += 1;
                    debug!("Added note with ID: {}", note_id.0)
                }
                Err(err) => {
                    failed_in_deck.push((err.to_string(), note));
                    debug!("Error: {:?}; for note: {:?}", err, &failed_in_deck.last());
                }
            };
            global_pbar.inc(1);
            current_deck_pb.inc(1);
        }
        num_added_total += num_added;
        if !failed_in_deck.is_empty() {
            failed_notes.push((deck.source_file, failed_in_deck));
        }
        decks_pbar.inc(1);
    }

    let _ = m.clear();

    info!("Added {} notes.", num_added_total);
    if !failed_notes.is_empty() {
        warn!(
            "Failed in {} decks, with a total of {} failed notes.",
            failed_notes.len(),
            failed_notes
                .par_iter()
                .map(|(_, item)| { item.len() })
                .sum::<usize>(),
        );

        let mut f = File::create(input_dir.join("failed_notes.json"))?;
        let failed_hash_map: HashMap<PathBuf, Vec<(String, ModelType)>> = failed_notes
            .into_par_iter()
            .map(|(source, failed)| (source, failed))
            .collect();
        f.write_all(serde_json::to_string_pretty(&failed_hash_map)?.as_bytes())?;
    }

    print_step(5, 10, Some("Done"), Some(SPARKLE));
    Ok(())
}
