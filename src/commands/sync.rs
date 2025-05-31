use crate::anki_connect;
use crate::anki_connect::anki_connect_client::params as client_params;
use crate::anki_connect::anki_connect_client::response::BasicResponse;
use crate::anki_connect::card::CardId;
use crate::anki_connect::deck::DeckId;
use crate::anki_connect::decks_client::params::ChangeDeck;
use crate::anki_connect::notes_client::params::{self as notes_params, UpdateNoteFields};
use crate::anki_connect::notes_client::responses::NoteInfo;
use crate::anki_connect::response::Response;
use crate::anki_connect::{
    AnkiConnectClient, ClientBehavior, error::APIError, model::Model, note::NoteId,
};
use crate::callout::Callout;
use crate::cli::SyncArgs;
use crate::deck::Deck;
use crate::find_markdown_files::find_markdown_files;
use crate::model::ModelType;
use crate::model::{InternalModelMethods, MediaFile};
use crate::note_operation::NoteOperation;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use rayon::iter::{Either, IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
use std::cmp::max;
use std::collections::HashMap;
use std::fmt::Display;
use std::fs::{File, read_to_string};
use std::io::Write;
use std::ops::Deref;
use std::path::PathBuf;
use std::str::FromStr;
use std::thread;
use tracing::{debug, error, info, warn};
use tracing_subscriber::field::debug;

use rayon::prelude::*;

use crate::error::M2AnkiError;
use crate::progress::{LOOKING_GLASS, SPARKLE, print_step};

#[derive(Debug)]
struct SyncStats {
    num_added: u64,
    num_updated: u64,
    num_moved: u64,
    num_deleted: u64,
}

impl Display for SyncStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_value = *[
            self.num_added,
            self.num_updated,
            self.num_moved,
            self.num_deleted,
        ]
        .iter()
        .max()
        .unwrap();
        let width = (max_value + 10).to_string().len();
        write!(
            f,
            "{:<8}{:>width$}\n{:<8}{:>width$}\n{:<8}{:>width$}\n{:<8}{:>width$}",
            "Added:",
            self.num_added,
            "Updated:",
            self.num_updated,
            "Moved:",
            self.num_moved,
            "Deleted:",
            self.num_deleted,
            width = width,
        )
    }
}

pub fn sync(args: SyncArgs) -> Result<(), M2AnkiError> {
    // Extract args into variables
    let parent_deck = args.deck.unwrap().to_string();
    let model_type_name = args.model_type_name.unwrap().to_string();
    let model_name = args
        .model_name
        .unwrap_or_else(|| format!("md2anki {}", &model_type_name));
    let header_lang: Option<String> = Some(args.header_lang.clone().unwrap().to_string());
    let input_dir = &args.input_dir;

    let max_steps = 10;

    print_step(
        1,
        max_steps,
        Some("Connecting to Anki"),
        Some(LOOKING_GLASS),
    );

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
    let parent_deck_clone = parent_deck.clone();
    let tx_files = tx.clone();
    let markdown_files_hadle = thread::spawn(move || {
        let markdown_files = find_markdown_files(&input_dir_clone).unwrap_or_else(|_| Vec::new());
        let found_files = !markdown_files.is_empty();
        tx_files.send(("md_files", found_files));

        info!("Found {} markdown files", &markdown_files.len());

        print_step(2, max_steps, Some("Extracting decks"), Some(LOOKING_GLASS));

        let mut decks: Vec<Deck> = markdown_files
            .par_iter()
            .map(Deck::try_from)
            .filter(Result::is_ok)
            .map(Result::unwrap)
            .filter(|deck| !deck.callouts.is_empty())
            .map(|mut deck| {
                deck.qualified_name = deck
                    .get_qualified_name(Some(&input_dir_clone), Some(&parent_deck_clone))
                    .unwrap_or_default();
                deck
            })
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
    let (mut decks, total_callouts, model_type, css) = markdown_files_hadle
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

    // Delete the deck
    if args.delete_existing {
        let _ = client.decks().delete(&parent_deck);
    }

    // dbg!(&decks);
    // return Ok(());

    // Get existing notes
    let mut anki_notes_in_deck = if client.decks().find_deck_id_by_name(&parent_deck).is_ok() {
        client.notes().get_notes_by_deck_name(&parent_deck)?
    } else {
        Vec::new()
    };

    let mut markdown_id_to_anki_note: HashMap<&String, &NoteInfo> = HashMap::new();
    let mut markdown_id_to_anki_note_id: HashMap<&String, &NoteId> = HashMap::new();
    let mut to_delete_anki_cards: Vec<&NoteId> = vec![];

    if !anki_notes_in_deck.is_empty() {
        let anki_all_card_ids: Vec<&CardId> = anki_notes_in_deck
            .par_iter()
            .map(|note| &note.cards)
            .flatten()
            .collect();

        markdown_id_to_anki_note = anki_notes_in_deck
            .par_iter()
            .map(|note| (&note.markdown_id, note))
            .collect();

        markdown_id_to_anki_note_id = anki_notes_in_deck
            .par_iter()
            .map(|note| (&note.markdown_id, &note.note_id))
            .collect();

        let anki_all_card_ids: Vec<&CardId> = anki_notes_in_deck
            .par_iter()
            .map(|note| &note.cards)
            .flatten()
            .collect();

        let anki_decks: HashMap<String, Vec<CardId>> =
            client.decks().get_decks(&anki_all_card_ids)?;

        let anki_card_ids_to_deck: HashMap<&CardId, &str> = anki_decks
            .par_iter()
            .map(|(name, cards)| cards.par_iter().map(|card| (card, name.as_str())))
            .flatten()
            .collect();

        let markdown_id_to_anki_deck: Result<HashMap<&String, &str>, M2AnkiError> =
            anki_notes_in_deck
                .par_iter()
                .map(|note| {
                    let card_id = note.cards.first().ok_or(M2AnkiError::NoteHasNoCards)?;
                    let deck =
                        anki_card_ids_to_deck
                            .get(card_id)
                            .ok_or(M2AnkiError::DeckNameNotFound(format!(
                                "Searching for {:?}",
                                card_id
                            )))?;
                    Ok((&note.markdown_id, *deck))
                })
                .collect();
        let markdown_id_to_anki_deck = markdown_id_to_anki_deck?;

        // Set operations for each Callout
        decks.par_iter_mut().for_each(|deck| {
            deck.callouts.par_iter_mut().try_for_each(|callout| {
                // Callout is new
                if !markdown_id_to_anki_note_id.contains_key(&callout.markdown_id) {
                    callout.operation = NoteOperation::Add;
                    Ok::<(), M2AnkiError>(())
                } else {
                    let anki_note_id = markdown_id_to_anki_note_id
                        .get(&callout.markdown_id)
                        .unwrap();

                    if markdown_id_to_anki_deck
                        .get(&callout.markdown_id)
                        .ok_or(M2AnkiError::DeckNameNotFound(format!(
                            "Searching for {:?}",
                            &callout.markdown_id
                        )))?
                        .eq(&deck.qualified_name)
                    {
                        callout.operation = NoteOperation::Update;
                    } else {
                        callout.operation = NoteOperation::Move;
                    }
                    Ok::<(), M2AnkiError>(())
                }
            });
        });

        if anki_notes_in_deck.len() != total_callouts {
            let callouts_map: HashMap<&String, &Callout> = decks
                .par_iter()
                .flat_map(|deck| {
                    deck.callouts
                        .par_iter()
                        .map(|callout| (&callout.markdown_id, callout))
                })
                .collect();
            to_delete_anki_cards = anki_notes_in_deck
                .par_iter()
                .filter_map(|note| {
                    if !callouts_map.contains_key(&note.markdown_id) {
                        Some(&note.note_id)
                    } else {
                        None
                    }
                })
                .collect::<Vec<&NoteId>>()
        };
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

    // Prepare stats and error tracking
    let mut failed_notes: Vec<(PathBuf, Vec<(String, ModelType)>)> = Vec::new();
    let mut sync_stats = SyncStats {
        num_added: 0,
        num_updated: 0,
        num_moved: 0,
        num_deleted: 0,
    };

    // Start main upload loop
    print_step(4, max_steps, Some("Syncing notes to Anki"), None);
    for deck in decks {
        let deck_name = deck.qualified_name;

        // Set up deck pbar
        let current_deck_pb = m.add(ProgressBar::new(
            deck.callouts
                .len()
                .try_into()
                .map_err(|_| M2AnkiError::ProgressBarError)?,
        ));
        current_deck_pb.set_style(sty.clone());
        current_deck_pb.set_message(deck_name.clone());

        let _ = client.decks().find_or_create_deck(deck_name.as_str())?;

        let mut note_id: NoteId = NoteId(0);
        let mut num_added = 0;
        let mut failed_in_deck: Vec<(String, ModelType)> =
            Vec::with_capacity(deck.callouts.len() / 2);

        // Prepare all notes
        let notes: Vec<ModelType> = deck
            .callouts
            .par_iter()
            .map(|callout| model_type.from_callout(callout, header_lang.as_deref()))
            .collect();

        // Add
        let to_add_notes: Vec<&ModelType> = notes
            .par_iter()
            .filter(|note| matches!(note.get_operation(), NoteOperation::Add))
            .collect();

        client
            .notes()
            .add_notes_convenience(&deck_name, &model_name, to_add_notes);

        // for note in to_add_notes {
        //     match client
        //         .notes()
        //         .add_note(note.to_add_note(&deck_name, &model_name))
        //     {
        //         Ok(id) => {
        //             note_id = id;
        //             num_added += 1;
        //             // debug!("Added note with ID: {}", note_id.0)
        //         }
        //         Err(err) => {
        //             failed_in_deck.push((err.to_string(), note.clone()));
        //             // debug!("Error: {:?}; for note: {:?}", err, &failed_in_deck.last());
        //         }
        //     };
        //     global_pbar.inc(1);
        //     current_deck_pb.inc(1);
        // }

        // Update (all notes that are Move, need to be updated first)
        let to_update_notes: Vec<&ModelType> = notes
            .par_iter()
            .filter(|note| {
                matches!(
                    note.get_operation(),
                    NoteOperation::Update | NoteOperation::Move
                )
            })
            .collect();

        let (to_update_params, to_update_errors): (
            Vec<Result<notes_params::UpdateNoteFields, M2AnkiError>>,
            Vec<Result<notes_params::UpdateNoteFields, M2AnkiError>>,
        ) = to_update_notes
            .par_iter()
            .map(|note| {
                match markdown_id_to_anki_note_id
                    .get(note.get_markdown_id())
                    .copied()
                {
                    Some(note_id) => Ok(note.to_update_note(note_id)),
                    None => Err(M2AnkiError::CardIdNotFound(
                        note.get_markdown_id().to_string(),
                    )),
                }
            })
            .partition(Result::is_ok);
        let to_update_params: Vec<notes_params::UpdateNoteFields> = to_update_params
            .into_par_iter()
            .map(|action| action.unwrap())
            .collect();
        let to_update_actions: Vec<client_params::Action<notes_params::UpdateNoteFields<'_>>> =
            to_update_params
                .par_iter()
                .map(|params| client_params::Action::new("updateNote", 6, params))
                .collect();
        let to_update_actions_refs = to_update_actions.par_iter().collect();

        match client.multi::<UpdateNoteFields<'_>, BasicResponse>(to_update_actions_refs) {
            Ok(result) => {
                // for (idx, res) in result.iter().enumerate() {
                //     if !res.error.is_some() {
                //         let markdown_id = to_update_params[idx]
                //             .note
                //             .fields
                //             .get("MarkdownID")
                //             .unwrap()
                //             .clone();
                //         failed_in_deck.push((
                //             "Failed to update".to_string(),
                //             to_update_notes
                //                 .par_iter()
                //                 .map(|note| *note)
                //                 .find_any(|note| note.get_markdown_id() == markdown_id)
                //                 .unwrap()
                //                 .clone(),
                //         ));
                //     }
                // }
            }
            Err(err) => match err {
                APIError::UnknownError(ref msg) => {
                    error!(
                        "{}({}) error for all notes in {}",
                        err,
                        msg,
                        deck.source_file.to_str().unwrap()
                    )
                }
                _ => error!(
                    "{} error for all notes in {}",
                    err,
                    deck.source_file.to_str().unwrap()
                ),
            },
        };

        to_update_errors
            .iter()
            // .filter(|err| Result::is_err(err))
            .for_each(|item| match item {
                Ok(_) => (),
                Err(err) => match &err {
                    M2AnkiError::CardIdNotFound(markdown_id) => failed_in_deck.push((
                        err.to_string(),
                        to_update_notes
                            .par_iter()
                            .map(|note| *note)
                            .find_any(|note| note.get_markdown_id() == markdown_id)
                            .unwrap()
                            .clone(),
                    )),
                    _ => (),
                },
            });

        global_pbar.inc(to_update_params.len() as u64);
        current_deck_pb.inc(to_update_params.len() as u64);

        // Move
        let to_move_update_callouts: Vec<&ModelType> = notes
            .par_iter()
            .filter(|callout| matches!(callout.get_operation(), NoteOperation::Move))
            .collect();
        let change_deck_params: Vec<ChangeDeck> = to_move_update_callouts
            .par_iter()
            .map(|note| {
                let anki_note = markdown_id_to_anki_note
                    .get(&note.get_markdown_id())
                    .unwrap();
                let cards: Vec<&CardId> = anki_note.cards.iter().collect();
                ChangeDeck::new(cards, &deck_name)
            })
            .collect();
        let move_actions: Vec<client_params::Action<ChangeDeck>> = change_deck_params
            .par_iter()
            .map(|params| client_params::Action::new("changeDeck", 6, params))
            .collect();
        let move_actions_refs = move_actions.par_iter().collect();
        client.multi::<_, BasicResponse>(move_actions_refs);

        // Update overall stats
        sync_stats.num_added += num_added;
        sync_stats.num_updated += to_update_params.len() as u64;
        sync_stats.num_moved += move_actions.len() as u64;
        if !failed_in_deck.is_empty() {
            failed_notes.push((deck.source_file, failed_in_deck));
        }
        decks_pbar.inc(1);
    }

    let _ = m.clear();

    // Delete removed notes
    if !to_delete_anki_cards.is_empty() {
        client.notes().delete_notes(&to_delete_anki_cards);
        sync_stats.num_deleted += to_delete_anki_cards.len() as u64;
    }

    // TODO: find a way to delete empty decks

    // Delete empty decks
    // if sync_stats.num_moved > 0 || sync_stats.num_deleted > 0 {
    //     let anki_notes_in_deck = client.notes().get_notes_by_deck_name(&parent_deck)?;
    //     let all_card_ids = anki_notes_in_deck
    //         .par_iter()
    //         .flat_map(|note| &note.cards)
    //         .collect();
    //     let anki_decks: HashMap<String, Vec<CardId>> = client.decks().get_decks(&all_card_ids)?;
    //     let empty_decks: Vec<String> = anki_decks
    //         .into_par_iter()
    //         .filter_map(
    //             |(name, cards)| {
    //                 if cards.is_empty() { Some(name) } else { None }
    //             },
    //         )
    //         .collect();
    //     dbg!(&empty_decks);
    //     client
    //         .decks()
    //         .delete_decks(empty_decks.iter().map(|name| name.as_str()).collect());
    // }

    // Report stats
    info!("\nSync Stats:\n{}", sync_stats);

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

    print_step(5, max_steps, Some("Done"), Some(SPARKLE));
    Ok(())
}
