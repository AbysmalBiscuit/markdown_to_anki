use std::{
    error::Error,
    path::{Path, PathBuf},
};

use strum::Display;

use crate::callout::Callout;

#[derive(Display, Debug)]
pub enum DeckError {
    MissingFileName,
    WrongMarkdownFileExtension(PathBuf),
}

impl Error for DeckError {}

#[derive(Debug)]
pub struct Deck {
    pub name: String,
    pub source_file: PathBuf,
    pub callouts: Vec<Callout>,
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
        Ok(Deck {
            name: value
                .file_name()
                .ok_or(DeckError::MissingFileName)?
                .to_string_lossy()
                .into_owned(),
            source_file: value.clone(),
            callouts: Callout::extract_callouts(value).unwrap(),
        })
    }
}
