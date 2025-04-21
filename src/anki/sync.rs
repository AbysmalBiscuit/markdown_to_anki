use crate::callout::Callout;
use crate::deck::Deck;
use crate::find_markdown_files::find_markdown_files;
use crate::note::basic::Basic;
use ankiconnect_rs::{AnkiClient, DuplicateScope, NoteBuilder};
use jwalk::WalkDir;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{Error as IOError, Write};
use std::path::PathBuf;

use crate::error::GenericError;

// fn request(action, )
pub fn sync(
    path: &PathBuf,
    parent_deck: String,
    model_name: String,
    header_lang: Option<&str>,
) -> Result<(), GenericError> {
    // Create a client with default connection (localhost:8765)
    let client = AnkiClient::new();

    let markdown_files = find_markdown_files(path)?;

    if markdown_files.is_empty() {
        println!(
            "Failed to find any markdown files in: '{}'",
            path.to_str().unwrap()
        );
        return Ok(());
    }

    println!("Found {} markdown files", &markdown_files.len());

    let decks = markdown_files
        .par_iter()
        .map(|path| Deck::try_from(path).unwrap())
        .filter(|deck| !deck.callouts.is_empty())
        .collect::<Vec<_>>();

    let num_found_decks: usize = decks.len();
    let num_total_callouts: usize = decks.par_iter().map(|d| d.callouts.len()).sum();

    println!(
        "Found {} decks with a total of {} callouts",
        num_found_decks, num_total_callouts
    );

    let callouts: Vec<Callout> = markdown_files
        .par_iter()
        .map(|path| Callout::extract_callouts(path).unwrap())
        .flatten()
        .collect::<Vec<_>>();

    // Get available decks and models
    let decks = client.decks().get_all()?;
    let models = client.models().get_all()?;
    let selected_model = models
        .par_iter()
        .find_any(|model| model.name().eq(&model_name));
    dbg!(selected_model);
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

    // Build a note with the selected model
    let selected_model = &models[0];
    dbg!(selected_model);
    // let front_field = selected_model.field_ref("Front").unwrap();
    // let back_field = selected_model.field_ref("Back").unwrap();
    //
    // let note = NoteBuilder::new(selected_model.clone())
    //     .with_field(front_field, "¿Dónde está la biblioteca?")
    //     .with_field(back_field, "Where is the library?")
    //     .with_tag("spanish-vocab")
    //     .build()?;

    // dbg!(markdown::to_html("foo\n\nbar"));
    // dbg!(&decks[0].callouts[0]);
    // dbg!(&decks[0].callouts[0].to_html_only_content(None));
    // let mut f = File::create(path.join("out.html"))?;
    // f.write_all(&decks[0].callouts[0].to_html(None).as_bytes())?;

    let basics: Vec<Basic> = callouts
        .par_iter()
        .map(|callout| Basic::from_callout(callout, header_lang))
        .collect();
    let mut f = File::create(path.join("out.html"))?;
    f.write_all(&basics[0].back.as_bytes())?;
    dbg!(&basics);

    // Add the note to the first deck
    // let note_id = client.cards().add_note(&decks[0], note, false, None)?;
    // println!("Added note with ID: {}", note_id.value());

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
