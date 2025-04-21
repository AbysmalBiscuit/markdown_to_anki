use crate::callout::Callout;
use crate::deck::Deck;
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

pub fn find_markdown_files(input_dir: &PathBuf) -> Result<Vec<PathBuf>, IOError> {
    Ok(WalkDir::new(input_dir)
        .into_iter()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "md"))
        .collect::<Vec<_>>())
}
