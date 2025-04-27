use super::client_traits::Find;
use crate::deck::Deck;
use crate::find_markdown_files::find_markdown_files;
use crate::model::ModelType;
use crate::model::traits::InternalModel;
use ankiconnect_rs::{AnkiClient, Model, Note, NoteId};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
use std::fs::{File, read_to_string};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use tracing::{debug, info, warn};

use crate::error::GenericError;
use crate::progress::{LOOKING_GLASS, SPARKLE, print_step};

#[allow(unused)]
fn print_models_info(models: &[Model]) {
    for (i, model) in models.iter().enumerate() {
        println!("\n{}. Model: {} (ID: {})", i, model.name(), model.id().0);

        // Print field information
        println!("   Fields ({}):", model.fields().len());
        for field in model.fields() {
            println!("   - {} (position: {})", field.name(), field.ord());

            // Add some helpful info about likely roles
            if field.is_front() {
                println!("     Likely role: Question/Front field");
            } else if field.is_back() {
                println!("     Likely role: Answer/Back field");
            }
        }
    }
}

pub fn sync(
    input_dir: &PathBuf,
    parent_deck: String,
    model_type: String,
    model_name: String,
    css_file: &Path,
    header_lang: Option<&str>,
) -> Result<(), GenericError> {
    print_step(1, 10, Some("Connecting to Anki"), Some(LOOKING_GLASS));
    // Create a client with default connection (localhost:8765)
    let client = AnkiClient::new();

    let markdown_files = find_markdown_files(input_dir)?;

    if markdown_files.is_empty() {
        warn!(
            "Failed to find any markdown files in: '{}'",
            input_dir.to_str().unwrap()
        );
        return Ok(());
    }
    info!("Found {} markdown files", &markdown_files.len());

    print_step(2, 10, Some("Extracting decks"), Some(LOOKING_GLASS));

    let decks = markdown_files
        .par_iter()
        .map(|path| Deck::try_from(path).unwrap())
        .filter(|deck| !deck.callouts.is_empty())
        .collect::<Vec<_>>();

    let num_found_decks: usize = decks.len();
    let num_total_callouts: usize = decks.par_iter().map(|d| d.callouts.len()).sum();

    info!(
        "Found {} decks with a total of {} callouts",
        num_found_decks, num_total_callouts
    );

    let internal_model = ModelType::from_str(&model_type)?;

    // Load css file if it exists
    let css = if css_file.is_file() {
        read_to_string(css_file)?
    } else {
        "".into()
    };

    let mut created_model = false;

    let note_type = match client.find_model(&model_name) {
        Ok(model) => model,
        Err(_) => {
            let model_id = internal_model.create_model(&client, &css)?;
            created_model = true;
            match client.models().get_by_id(model_id)? {
                Some(model) => Ok(model),
                None => Err("failed to create model or to get new model id"),
            }?
        }
    };

    if !css.is_empty() && !created_model {
        let _ = client.models().update_styling(&note_type, css.as_str());
        info!("Updated model CSS.");
        // dbg!(&css);
    }

    let mut failed_notes = Vec::new();
    let mut num_added_total = 0usize;

    // Delete the deck and re-create it for testing purposes
    let _ = client.decks().delete(&parent_deck, true);

    print_step(4, 10, Some("Adding notes to Anki"), None);
    let total_callouts: usize = decks.par_iter().map(|deck| deck.callouts.len()).sum();
    let m = MultiProgress::new();
    let sty = ProgressStyle::with_template(
        "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
    )
    .unwrap()
    .progress_chars("##-");
    let global_pbar = m.add(ProgressBar::new(total_callouts.try_into()?));
    global_pbar.set_style(sty.clone());
    global_pbar.set_message("Overall");
    let decks_pbar = m.add(ProgressBar::new(decks.len().try_into()?));
    decks_pbar.set_style(sty.clone());
    decks_pbar.set_message("Decks");
    for deck in decks {
        let internal_models: Vec<ModelType> = deck
            .callouts
            .par_iter()
            .map(|callout| internal_model.from_callout(callout, header_lang))
            .collect();

        let notes: Vec<_> = internal_models
            .into_par_iter()
            .map(|internal| internal.to_note(note_type.clone()))
            .filter(Result::is_ok)
            .map(Result::unwrap)
            .collect();

        let deck_name = deck.get_qualified_name(Some(input_dir), Some(&parent_deck))?;
        let selected_deck = client.find_or_create_deck(format!("Current: {}", deck_name).as_str());

        // Add the notes to the deck
        let current_deck_pb = m.add(ProgressBar::new(notes.len().try_into()?));
        current_deck_pb.set_style(sty.clone());
        current_deck_pb.set_message(deck_name.clone());

        let mut note_id: NoteId = NoteId(0);
        let mut num_added = 0usize;
        let mut failed_in_deck: Vec<Note> = Vec::with_capacity(deck.callouts.len() / 2);

        for note in notes {
            match client
                .cards()
                .add_note(&selected_deck, note.clone(), false, None)
            {
                Ok(id) => {
                    note_id = id;
                    num_added += 1;
                    debug!("Added note with ID: {}", note_id.value())
                }
                Err(err) => {
                    failed_in_deck.push(note);
                    debug!("AnkiError: {:?}", err);
                    debug!("Failed to create note for: {:?}", &failed_in_deck.last())
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
        )
    }

    let mut f = File::create(input_dir.join("failed_notes.txt"))?;
    f.write_all(
        failed_notes
            .par_iter()
            .map(|note| format!("{:?}", note))
            .collect::<Vec<_>>()
            .join("\n")
            .as_bytes(),
    )?;

    print_step(5, 10, Some("Done"), Some(SPARKLE));
    Ok(())
}
