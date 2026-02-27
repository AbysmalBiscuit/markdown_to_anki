use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
};

use strum::Display;
use thiserror::Error;

use crate::callout::{Callout, error::CalloutError};

#[derive(Error, Display, Debug)]
pub enum DeckError {
    Io(#[from] std::io::Error),
    WrongMarkdownFileExtension(PathBuf),
    InvalidUtf8Path(PathBuf),
}

#[derive(Debug)]
pub struct Deck {
    pub source_file: PathBuf,
    pub qualified_name: String,
    pub callouts: Vec<Callout>,
    pub failed: Vec<(String, CalloutError)>,
}

impl Deck {
    pub fn get_qualified_name(
        &self,
        to_remove_prefix: Option<&Path>,
        to_add_prefix: Option<&str>,
    ) -> Result<String, DeckError> {
        let to_remove_prefix = to_remove_prefix.map_or("", |path| path.to_str().unwrap_or(""));
        let to_add_prefix = to_add_prefix.unwrap_or("");
        let source_file = self
            .source_file
            .to_str()
            .ok_or_else(|| DeckError::InvalidUtf8Path(self.source_file.clone()))?;

        let clean_name = source_file
            .strip_prefix(to_remove_prefix)
            .unwrap_or(source_file)
            .strip_suffix(".md")
            .ok_or_else(|| DeckError::WrongMarkdownFileExtension(self.source_file.clone()))?
            .split('/')
            .filter(|part| !part.is_empty())
            .collect::<Vec<_>>()
            .join("::");

        if to_add_prefix.is_empty() {
            Ok(clean_name)
        } else {
            Ok(format!("{to_add_prefix}::{clean_name}"))
        }
    }
}

impl TryFrom<&PathBuf> for Deck {
    type Error = DeckError;

    fn try_from(value: &PathBuf) -> Result<Self, Self::Error> {
        let callouts_results = Callout::extract_callouts(value);
        if !callouts_results.failed.is_empty() {
            // TODO: extract line numbers to report more accurate errors about why making some
            // caloluts failed.
            let content: String = read_to_string(value).map_or(String::new(), |text| text);
            if !content.is_empty() {}
        }
        Ok(Self {
            source_file: value.clone(),
            qualified_name: String::new(),
            callouts: callouts_results.callouts,
            failed: callouts_results.failed,
        })
    }
}
