use std::path::{Path, PathBuf};

use strum::Display;

use crate::callout::Callout;

#[derive(Display, Debug)]
pub enum DeckError {
    MissingFileName,
}

#[derive(Debug)]
pub struct Deck {
    pub name: String,
    pub source_file: PathBuf,
    pub callouts: Vec<Callout>,
}

impl Deck {
    pub fn get_qualified_name(&self, path_prefix: &Path) -> String {
        let prefix = path_prefix
            .as_os_str()
            .to_str()
            .expect("path should be the source directory");

        let clean_name = self
            .source_file
            .to_str()
            .expect("the source_file should exist")
            .strip_prefix(&format!("{}/", prefix))
            .expect("this should be a &str");

        clean_name
            .strip_suffix(".md")
            .expect("file should have a '.md' extension")
            .split('/')
            .collect::<Vec<_>>()
            .join("::")
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
