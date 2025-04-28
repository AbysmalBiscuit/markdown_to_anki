use std::{
    error::Error,
    fs::read_to_string,
    path::{Path, PathBuf},
};

use strum::Display;

use crate::callout::{Callout, error::CalloutError};

#[derive(Display, Debug)]
pub enum DeckError {
    Io(std::io::Error),
    WrongMarkdownFileExtension(PathBuf),
}

impl Error for DeckError {}

#[derive(Debug)]
pub struct Deck {
    pub source_file: PathBuf,
    pub callouts: Vec<Callout>,
    pub failed: Vec<(String, CalloutError)>,
}

impl Deck {
    pub fn get_qualified_name(
        &self,
        to_remove_prefix: Option<&Path>,
        to_add_prefix: Option<&str>,
    ) -> Result<String, DeckError> {
        let to_remove_prefix = match to_remove_prefix {
            Some(path) => path.to_str().unwrap_or(""),
            None => "",
        };
        let to_add_prefix = to_add_prefix.unwrap_or("");
        let source_file = self.source_file.to_str().unwrap();

        let clean_name = source_file
            .strip_prefix(to_remove_prefix)
            .unwrap_or(source_file)
            .strip_suffix(".md")
            .ok_or_else(|| DeckError::WrongMarkdownFileExtension(self.source_file.clone()))?
            .split('/')
            .filter(|part| !part.is_empty())
            .collect::<Vec<_>>()
            .join("::");

        if !to_add_prefix.is_empty() {
            Ok(format!("{}::{}", to_add_prefix, clean_name))
        } else {
            Ok(clean_name)
        }
    }
}

impl TryFrom<&PathBuf> for Deck {
    type Error = DeckError;

    fn try_from(value: &PathBuf) -> Result<Self, Self::Error> {
        let callouts_results = Callout::extract_callouts(value);
        if !callouts_results.failed.is_empty() {
            let content: String = match read_to_string(value) {
                Ok(text) => text,
                Err(err) => "".to_string(),
            };
            if !content.is_empty() {}
        }
        Ok(Deck {
            source_file: value.clone(),
            callouts: callouts_results.callouts,
            failed: callouts_results.failed,
        })
    }
}
