use crate::anki_connect::anki_connect_client::params::Action;
use crate::anki_connect::anki_connect_client::response::BasicResponse;
use crate::anki_connect::card::CardId;
use crate::anki_connect::decks_client::params::ChangeDeck;
use crate::anki_connect::notes_client::params::{AddNote, UpdateNoteFields};
use crate::anki_connect::notes_client::responses::NoteInfo;
use crate::anki_connect::{AnkiConnectClient, error::APIError, model::Model, note::NoteId};
use crate::callout::Callout;
use crate::cli::SyncArgs;
use crate::deck::Deck;
use crate::find_markdown_files::find_markdown_files;
use crate::model::InternalModelMethods;
use crate::model::ModelType;
use crate::note_operation::NoteOperation;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
use std::cmp::min;
use std::collections::HashMap;
use std::fmt::Display;
use std::fs::{File, read_to_string};
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;
use std::thread;
use tracing::{debug, error, info, warn};

use rayon::prelude::*;

use crate::error::M2AnkiError;
use crate::progress::{
    BAR_CHART, CROSS, LOOKING_GLASS, PLUS, REPEAT, SHUFFLE, SPARKLE, SYNC, Step,
};

#[derive(Debug)]
struct SyncStats {
    num_added: u64,
    num_added_errors: u64,
    num_updated: u64,
    num_updated_errors: u64,
    num_moved: u64,
    num_moved_errors: u64,
    num_deleted: u64,
    num_deleted_errors: u64,
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
        let max_value = *[
            self.num_added_errors,
            self.num_updated_errors,
            self.num_moved_errors,
            self.num_deleted_errors,
        ]
        .iter()
        .max()
        .unwrap();
        let width2 = (max_value + 10).to_string().len();
        write!(
            f,
            "{:<8}{:>width$}\n{:<8}{:>width$}\n{:<8}{:>width$}\n{:<8}{:>width$}\n{:<15}{:>width2$}\n{:<15}{:>width2$}\n{:<15}{:>width2$}\n{:<15}{:>width2$}",
            "Added:",
            self.num_added,
            "Updated:",
            self.num_updated,
            "Moved:",
            self.num_moved,
            "Deleted:",
            self.num_deleted,
            "Added Errors:",
            self.num_added_errors,
            "Updated Errors:",
            self.num_updated_errors,
            "Moved Errors:",
            self.num_moved_errors,
            "Deleted Errors:",
            self.num_deleted_errors,
            width = width,
            width2 = width2,
        )
    }
}

#[derive(Debug)]
struct OperationParams<'a> {
    add: Vec<AddNote<'a>>,
    update: Vec<UpdateNoteFields<'a>>,
    move_: Vec<ChangeDeck<'a>>,
    delete: Vec<&'a NoteId>,
    notes: Vec<ModelType<'a>>,
    notes_errors: Vec<(M2AnkiError, &'a ModelType<'a>)>,
}

pub fn sync(args: SyncArgs) -> Result<(), M2AnkiError> {
    // Extract args into variables
    // it's better to use expect instead of unwrap
    // prefer to unwrap with special people
    // if let Some(...) = ... { ... } else { panic!() }
    let parent_deck = args.deck.unwrap().to_string();
    let model_type_name = args.model_type_name.unwrap().to_string();
    let model_name = args
        .model_name
        .unwrap_or_else(|| format!("md2anki {}", &model_type_name));
    let header_lang = args.header_lang.map(|item| item.to_string());
    // let header_lang: Option<String> = Some(args.header_lang.clone().unwrap().to_string());
    let input_dir = &args.input_dir;

    let mut step = Step::new(1, 10);
    step.print_step(Some("Connecting to Anki"), Some(LOOKING_GLASS));

    // Create a client with default connection (localhost:8765)
    let client = AnkiConnectClient::new(None, None);

    // Prepare channel for async initial processing
    let (tx, rx) = std::sync::mpsc::channel();

    // Test client connection
    let client_clone = client.clone();
    let tx_client = tx.clone();
    // put testing stuff in a function
    // take the client by reference and do the clone in the function and do the testing there
    let client_handle = thread::spawn(move || {
        let res: bool = client_clone.test_connection().unwrap_or(false);
        let _ = tx_client.send(("client", res));
    });

    let input_dir_clone = args.input_dir.clone();
    let parent_deck_clone = parent_deck.clone();
    let tx_files = tx.clone();
    step.print_step(Some("Extracting decks"), Some(LOOKING_GLASS));
    // pass in the channel handle as one of the function parameter
    // think of it as a unit of work that stands alone
    // functions serve 2 purposes:
    // 1) code reuse
    // 2) organziation, managinging complexity and congnitive overhead.
    //
    // In Python this costs something, but in rust this will get inlined.
    //
    // if you're not using a profiler to find hotspots, don't worry about inlining
    // if your cpu spends a lot of time in jump (in a tight loop), then inlining may be worth it
    // more functions may be better when profiling as it makes it easier to find hotspots
    //
    // in rust a single line function that hides a long clone change can be valid to improve
    // readability
    //
    // once your closure is more than 1/3 of your screen you need a function, or like 5 lines
    // (Igor's rule of thumb)
    let markdown_files_hadle = thread::spawn(move || {
        let markdown_files = find_markdown_files(&input_dir_clone).unwrap_or_else(|_| Vec::new());
        let found_files = !markdown_files.is_empty();
        let _ = tx_files.send(("md_files", found_files));

        info!("Found {} markdown files", &markdown_files.len());

        let decks: Vec<Deck> = markdown_files
            .par_iter()
            .map(Deck::try_from)
            .filter(Result::is_ok)
            .map(Result::unwrap)
            .filter(|deck| !deck.callouts.is_empty())
            .map(|mut deck| {
                // deck.with_name(input_dir, parent_deck) -> Deck
                // this returns the same deck but modified
                // this would fit more nicely in the builder pattern
                //
                // the rustic thing is to take things by value, consuming them and then returning
                // something new
                // mut references are fine if they are more ergonomic
                deck.qualified_name = deck
                    .get_qualified_name(Some(&input_dir_clone), Some(&parent_deck_clone))
                    .unwrap_or_default();
                deck
            })
            .collect();

        // doing a par_iter may not really improve the speed here.
        // using the previous par_iter to create the vec and total count as a tuple, then using a reducer to
        // create the final vector and int.
        let total_callouts: usize = decks.iter().map(|deck| deck.callouts.len()).sum();
        let _ = tx_files.send(("num_callouts", total_callouts > 0));

        // a more optimized thing is to use a semaphore that sends information in a multithreaded
        // context by incrementing and decrementing.
        // it's like the with() as ...: pattern in Python.
        // the guard inside is increments whenever it reaches a block.
        // it's used to do blocking in a parallel context.
        // You implement it using Atomic or Mutex.
        let num_found_decks: usize = decks.len();
        let num_total_callouts: usize = decks.iter().map(|d| d.callouts.len()).sum();

        // Display errors for callouts that couldn't be parsed
        let failed_decks: Vec<&Deck> = decks
            .par_iter()
            .filter(|deck| !deck.failed.is_empty())
            .collect();
        if !failed_decks.is_empty() {
            // this can be rewritten as a .iter() or .into_iter()
            // have the Deck handle it's formatting error message
            // using the result type would be the rustic way
            // using anyhow gets you close.
            // you can use a match to display on error and otherwise add it to the vec
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

    // using constants: to say why 0..3
    // this can be done as a funciton that can return an error, and on the first error you return
    // an error.
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

    client_handle
        .join()
        .map_err(|err| M2AnkiError::ThreadPanic(err))?;
    let (mut decks, total_callouts, model_type, css) = markdown_files_hadle
        .join()
        .map_err(|err| M2AnkiError::ThreadPanic(err))?;

    let model_type = model_type?;
    let css = css?;

    let mut created_model = false;

    // another way of doing this is:
    // let created_model = { ... } else { ... };
    // or:
    // client.models().find_by_name().map()
    // then closure goes inside
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
    // this would make more sense in a separate function
    // this would be truncation
    if args.delete_existing {
        let _ = client.decks().delete(&parent_deck);
    }

    // dbg!(&decks);
    // return Ok(());

    // Get existing notes
    let anki_notes_in_deck = if client.decks().find_deck_id_by_name(&parent_deck).is_ok() {
        client.notes().get_notes_by_deck_name(&parent_deck)?
    } else {
        Vec::new()
    };

    let mut operation_params = OperationParams {
        add: vec![],
        update: vec![],
        move_: vec![],
        delete: vec![],
        notes: vec![],
        notes_errors: vec![],
    };

    // add comments
    if !anki_notes_in_deck.is_empty() {
        let markdown_id_to_anki_note: HashMap<&String, &NoteInfo> = anki_notes_in_deck
            .par_iter()
            .map(|note| (&note.markdown_id, note))
            .collect();

        let markdown_id_to_anki_note_id: HashMap<&String, &NoteId> = anki_notes_in_deck
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

        // replace par_iter with just iter
        // use par_iter when there's enough work to benefit going into a thread
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
        // comments to explain if this condition then we do this, else this, we handle the errors
        // xyz, in human language.
        decks.par_iter_mut().for_each(|deck| {
            let _ = deck.callouts.par_iter_mut().try_for_each(|callout| {
                // Callout is new
                if !markdown_id_to_anki_note_id.contains_key(&callout.markdown_id) {
                    callout.operation = NoteOperation::Add;
                    Ok::<(), M2AnkiError>(())
                } else {
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

        operation_params.notes = decks
            .par_iter()
            .map(|deck| {
                deck.callouts.par_iter().map(|callout| {
                    model_type.from_callout(&callout, header_lang.as_deref(), &deck.qualified_name)
                })
            })
            .flatten()
            .collect();

        operation_params
            .notes
            .iter()
            .for_each(|note| match note.get_operation() {
                NoteOperation::Add => operation_params.add.push(AddNote::new(
                    note.to_add_note(note.get_deck_name(), &model_name),
                )),
                NoteOperation::Update => {
                    match markdown_id_to_anki_note_id.get(note.get_markdown_id()) {
                        Some(note_id) => operation_params.update.push(note.to_update_note(note_id)),
                        None => operation_params.notes_errors.push((
                            M2AnkiError::NoteIdNotFound(note.get_markdown_id().to_string()),
                            &note,
                        )),
                    }
                }
                NoteOperation::Move => {
                    let anki_note = markdown_id_to_anki_note
                        .get(&note.get_markdown_id())
                        .unwrap();
                    let cards: Vec<&CardId> = anki_note.cards.iter().collect();
                    operation_params
                        .move_
                        .push(ChangeDeck::new(cards, &note.get_deck_name()));
                }
                _ => (),
            });

        // Check if notes need to be deleted
        let callouts_map: HashMap<&String, &Callout> = decks
            .par_iter()
            .flat_map(|deck| {
                deck.callouts
                    .par_iter()
                    .map(|callout| (&callout.markdown_id, callout))
            })
            .collect();
        operation_params.delete = anki_notes_in_deck
            .par_iter()
            .filter_map(|note| {
                if !callouts_map.contains_key(&note.markdown_id) {
                    Some(&note.note_id)
                } else {
                    None
                }
            })
            .collect::<Vec<&NoteId>>()
    }

    // it may be better to have the deck, note, etc. to have the sync function

    debug!(
        "OperationParams {{ add: {:?}, update: {:?}, move_: {:?}, delete: {:?} }}",
        &operation_params.add.len(),
        &operation_params.update.len(),
        &operation_params.move_.len(),
        &operation_params.delete.len()
    );

    // dbg!(&operation_params);

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

    // let decks_pbar = m.add(ProgressBar::new(
    //     decks
    //         .len()
    //         .try_into()
    //         .map_err(|_| M2AnkiError::ProgressBarError)?,
    // ));
    // decks_pbar.set_style(sty.clone());
    // decks_pbar.set_message("Decks");

    // Prepare stats and error tracking
    let mut failed_notes: Vec<(PathBuf, Vec<(String, ModelType)>)> = Vec::new();
    let mut sync_stats = SyncStats {
        num_added: 0,
        num_added_errors: 0,
        num_updated: 0,
        num_updated_errors: 0,
        num_moved: 0,
        num_moved_errors: 0,
        num_deleted: 0,
        num_deleted_errors: 0,
    };

    // Start main upload loop
    // Add new notes
    m.suspend(|| {
        step.print_step(Some("Syncing notes to Anki"), Some(SYNC));
        step.print_step(Some("Adding new notes"), Some(PLUS));
    });
    if !operation_params.add.is_empty() {
        let action = "addNote";
        let add_actions: Vec<Action<AddNote>> = operation_params
            .add
            .par_iter()
            .map(|add_note| Action::new(action, 6, add_note))
            .collect();

        let add_actions_refs: Vec<&Action<AddNote>> = add_actions.par_iter().collect();

        let chunk_size = min(add_actions_refs.len(), 500);

        for chunk in add_actions_refs.chunks(chunk_size) {
            let _ = match client.multi::<AddNote, BasicResponse>(chunk.to_vec()) {
                // TODO: parse response to collect detailed errors for individual actions
                Ok(response) => {
                    global_pbar.inc(response.len().try_into().unwrap());
                    let (success, fail): (Vec<&BasicResponse>, Vec<&BasicResponse>) =
                        response.par_iter().partition(|resp| resp.error.is_none());
                    sync_stats.num_added += success.len() as u64;
                    sync_stats.num_added_errors += fail.len() as u64;
                    Ok(())
                }
                Err(err) => Err(M2AnkiError::APIError(err)),
            };
        }
    } else {
        m.suspend(|| info!("No new notes."));
    }

    // Update notes
    m.suspend(|| step.print_step(Some("Updating notes"), Some(REPEAT)));
    if !operation_params.update.is_empty() {
        let action = "updateNote";
        let update_actions: Vec<Action<UpdateNoteFields>> = operation_params
            .update
            .par_iter()
            .map(|update_note| Action::new(action, 6, update_note))
            .collect();

        let update_actions_refs: Vec<&Action<UpdateNoteFields>> =
            update_actions.par_iter().collect();

        let chunk_size = min(update_actions_refs.len(), 500);

        for chunk in update_actions_refs.chunks(chunk_size) {
            let _ = match client.multi::<UpdateNoteFields, BasicResponse>(chunk.to_vec()) {
                // TODO: parse response to collect detailed errors for individual actions
                Ok(response) => {
                    global_pbar.inc(response.len().try_into().unwrap());
                    let (success, fail): (Vec<&BasicResponse>, Vec<&BasicResponse>) =
                        response.par_iter().partition(|resp| resp.error.is_none());
                    sync_stats.num_updated += success.len() as u64;
                    sync_stats.num_updated_errors += fail.len() as u64;
                    Ok(())
                }
                Err(err) => Err(M2AnkiError::APIError(err)),
            };
        }
    } else {
        m.suspend(|| info!("No notes to update."));
    }

    m.suspend(|| step.print_step(Some("Moving notes"), Some(SHUFFLE)));
    if !operation_params.move_.is_empty() {
        let action = "changeDeck";
        let move_actions: Vec<Action<ChangeDeck>> = operation_params
            .move_
            .par_iter()
            .map(|change_deck| Action::new(action, 6, change_deck))
            .collect();

        let move_actions_refs: Vec<&Action<ChangeDeck>> = move_actions.par_iter().collect();

        let chunk_size = min(move_actions_refs.len(), 500);

        for chunk in move_actions_refs.chunks(chunk_size) {
            let _ = match client.multi::<ChangeDeck, BasicResponse>(chunk.to_vec()) {
                // TODO: parse response to collect detailed errors for individual actions
                Ok(response) => {
                    global_pbar.inc(response.len().try_into().unwrap());
                    let (success, fail): (Vec<&BasicResponse>, Vec<&BasicResponse>) =
                        response.par_iter().partition(|resp| resp.error.is_none());
                    sync_stats.num_moved += success.len() as u64;
                    sync_stats.num_moved_errors += fail.len() as u64;
                    Ok(())
                }
                Err(err) => Err(M2AnkiError::APIError(err)),
            };
        }
    } else {
        m.suspend(|| info!("No notes to move."));
    }

    // Delete removed notes
    m.suspend(|| step.print_step(Some("Deleting notes"), Some(CROSS)));
    if !operation_params.delete.is_empty() {
        let _ = client.notes().delete_notes(&operation_params.delete);
        sync_stats.num_deleted += operation_params.delete.len() as u64;
        global_pbar.inc(operation_params.delete.len().try_into().unwrap());
    }

    // TODO: find a way to delete empty decks

    // Delete empty decks
    m.suspend(|| step.print_step(Some("Deleting empty decks"), Some(CROSS)));
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
    m.suspend(|| step.print_step(Some("Displaying stats and results:"), Some(BAR_CHART)));
    global_pbar.finish();
    println!("\nSync Stats:\n{}", sync_stats);

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

    step.print_step(Some("Done"), Some(SPARKLE));
    Ok(())
}
