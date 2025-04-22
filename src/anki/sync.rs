use super::client_traits::Find;
use crate::callout::Callout;
use crate::deck::Deck;
use crate::find_markdown_files::find_markdown_files;
use crate::model::basic::Basic;
use ankiconnect_rs::{AnkiClient, AnkiConnectError, AnkiError, Model, Note, NoteBuilder, NoteId};
use indicatif::ProgressBar;
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
use std::fmt::format;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use tracing::{debug, error, info, warn};

use crate::error::GenericError;
use crate::progress::{LOOKING_GLASS, print_step};

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
    path: &PathBuf,
    parent_deck: String,
    model_name: String,
    header_lang: Option<&str>,
) -> Result<(), GenericError> {
    print_step(1, 10, Some("Connecting to Anki"), Some(LOOKING_GLASS));
    // Create a client with default connection (localhost:8765)
    let client = AnkiClient::new();

    let markdown_files = find_markdown_files(path)?;

    if markdown_files.is_empty() {
        warn!(
            "Failed to find any markdown files in: '{}'",
            path.to_str().unwrap()
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

    // let callouts: Vec<Callout> = markdown_files
    //     .par_iter()
    //     .map(|path| Callout::extract_callouts(path).unwrap())
    //     .flatten()
    //     .collect::<Vec<_>>();
    // dbg!(&callouts);

    // let basics: Vec<Basic> = callouts
    //     .par_iter()
    //     .map(|callout| Basic::from_callout(callout, header_lang))
    //     .collect();

    // let selected_model = client.models().get_by_name(&model_name)?.ok_or();
    let selected_model = client.find_model(&model_name)?;
    let front_field = selected_model
        .field_ref("Front")
        .ok_or(AnkiError::InvalidField {
            field_name: "Front".to_string(),
            model_name: model_name.clone(),
        })?;
    let back_field = selected_model
        .field_ref("Back")
        .ok_or(AnkiError::InvalidField {
            field_name: "Back".to_string(),
            model_name: model_name.clone(),
        })?;

    // let mut f = File::create(path.join("out.html"))?;
    // f.write_all(&basics[0].back.as_bytes())?;
    // dbg!(&basics);

    let mut failed_notes = Vec::new();
    let mut num_added_total = 0usize;

    // Delete the deck and re-create it for testing purposes
    client.decks().delete(&parent_deck, true);

    let total_callouts: usize = decks.par_iter().map(|deck| deck.callouts.len()).sum();
    let global_pbar = ProgressBar::new(total_callouts.try_into()?);
    let decks_pbar = ProgressBar::new(decks.len().try_into()?);
    for deck in decks {
        let basics: Vec<Basic> = deck
            .callouts
            .par_iter()
            .map(|callout| Basic::from_callout(callout, header_lang))
            .collect();

        let notes: Vec<_> = basics
            .into_par_iter()
            .map(|basic| {
                NoteBuilder::new(selected_model.clone())
                    .with_field_raw(front_field, &basic.front)
                    .with_field_raw(back_field, &basic.back)
                    .with_tag("md2anki")
                    .build()
            })
            .filter(Result::is_ok)
            .map(Result::unwrap)
            .collect();
        // dbg!(&notes);
        let deck_name = deck.get_qualified_name(Some(path), Some(&parent_deck))?;
        let selected_deck = client.find_or_create_deck(&deck_name);
        // dbg!(&selected_deck);

        // Add the notes to the deck
        let pb = ProgressBar::new(notes.len().try_into()?);
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
                    debug!("Failed to create note for: {:?}", &failed_in_deck.last())
                }
            };
            global_pbar.inc(1);
            pb.inc(1);
        }
        num_added_total += num_added;
        failed_notes.push((deck.source_file, failed_in_deck));
        decks_pbar.inc(1);
    }

    info!(
        "Added {} notes. Failed to add {} notes.",
        num_added_total,
        failed_notes.len()
    );
    // dbg!(markdown::to_html("foo\n\nbar"));
    // dbg!(&decks[0].callouts[0]);
    // dbg!(&decks[0].callouts[0].to_html_only_content(None));
    // let mut f = File::create(path.join("out.html"))?;
    // f.write_all(&decks[0].callouts[0].to_html(None).as_bytes())?;

    let mut f = File::create(path.join("failed_notes.txt"))?;
    f.write_all(
        failed_notes
            .par_iter()
            .map(|note| format!("{:?}", note))
            .collect::<Vec<_>>()
            .join("\n")
            .as_bytes(),
    )?;

    Ok(())
}

// This modules is heavily based on:
// https://github.com/ObsidianToAnki/Obsidian_to_Anki/blob/master/obsidian_to_anki.py#L220
//

// Requests used by ObsidianToAnki
// changeDeck
// deleteNotes
// findNotes
// getTags
// modelFieldNames
// modelNames
// multi -> addNote
// multi -> addTags
// multi -> notesInfo
// multi -> storeMediaFile
// multi -> updateNoteFields
// notesInfo
// removeTags
