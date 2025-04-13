use crate::deck::Deck;
use anki_bridge::prelude::*;
use jwalk::WalkDir;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::collections::HashMap;
use std::io::Error;
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
    let markdown_files = find_markdown_files(path)?;

    let decks = markdown_files
        .par_iter()
        .map(|path| Deck::try_from(path).unwrap())
        .collect::<Vec<_>>();

    let client = AnkiClient::default();
    let decks: Vec<String> = client.request(DeckNamesRequest {}).unwrap();
    let deck_stats: HashMap<usize, GetDeckStatsResponse> =
        client.request(GetDeckStatsRequest { decks }).unwrap();
    dbg!(deck_stats);
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
