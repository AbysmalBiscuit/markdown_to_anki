use std::any::Any;
use std::error::Error as StdError;
use std::io::Error as IOError;

use crate::anki_connect::error::APIError;
use crate::deck::DeckError;
use serde_json::Error as SerdeJsonError;
use thiserror::Error;

pub type GenericError = Box<dyn StdError + Send>;
pub type GenericSyncError = Box<dyn StdError + Send + Sync>;
pub type GenericSendStatic = Box<dyn StdError + Send + 'static>;

#[derive(Error, Debug)]
pub enum M2AnkiError {
    #[error("error from AnkiConnect API: '{0}'")]
    APIError(#[from] APIError),
    // #[error("Anki note not found with ID: '{0}'")]
    // AnkiNoteNotFound(String),
    // #[error("Card ID not found: '{0}'")]
    // CardIdNotFound(String),
    #[error("Deck error: {0}")]
    DeckError(#[from] DeckError),
    #[error("cannot find deck with name: '{0}'")]
    DeckNameNotFound(String),
    #[error("error: {0}")]
    GenericError(#[from] GenericError),
    #[error("error: {0}")]
    GenericSendStatic(GenericSendStatic),
    #[error("error: {0}")]
    GenericSyncError(#[from] GenericSyncError),
    #[error("Deck error: {0}")]
    IOError(#[from] IOError),
    #[error("error parsing model: {0}")]
    ModelParseError(#[from] strum::ParseError),
    #[error("error")]
    NoteHasNoCards,
    #[error("Deck error: {0}")]
    NoteIdNotFound(String),
    #[error("error")]
    ProgressBarError,
    #[error("JSON parsing error: '{0}'")]
    SerdeJsonError(#[from] SerdeJsonError),
    #[error("thread panicked: '{0:?}'")]
    ThreadPanic(Box<dyn Any + Send>),
}

impl From<&str> for M2AnkiError {
    fn from(value: &str) -> Self {
        M2AnkiError::GenericSyncError(value.into())
    }
}
