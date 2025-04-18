use crate::callout::callout::Callout;
use crate::deck::Deck;
use crate::note::Basic;
use anki_bridge::prelude::*;
use jwalk::WalkDir;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Error, Write};
use std::path::PathBuf;

use crate::error::GenericError;

fn find_markdown_files(input_dir: &PathBuf) -> Result<Vec<PathBuf>, Error> {
    Ok(WalkDir::new(input_dir)
        .into_iter()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "md"))
        .collect::<Vec<_>>())
}

// fn request(action, )
pub fn sync(path: &PathBuf, parent_deck: String) -> Result<(), GenericError> {
    let client = AnkiClient::default();

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

    let decks_names: Vec<String> = client.request(DeckNamesRequest {}).unwrap();
    let deck_stats: HashMap<usize, GetDeckStatsResponse> = client
        .request(GetDeckStatsRequest { decks: decks_names })
        .unwrap();

    dbg!(&decks[0].callouts[0]);
    // let mut f = File::create(path.join("out.html"))?;
    // f.write_all(&decks[0].callouts[0].to_html().as_bytes())?;

    let basics: Vec<Basic> = callouts.par_iter().map(Basic::from).collect();
    // dbg!(&basics);
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
