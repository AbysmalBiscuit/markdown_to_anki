use std::any::Any;
use std::error::Error as StdError;
use std::io::Error as IOError;

use crate::anki_connect::error::APIError;
use crate::deck::DeckError;
use serde_json::Error as SerdeJsonError;
use thiserror::Error;

pub type GenericError = Box<dyn StdError + Send + Sync>;

/// Main error enum of `md2anki`.
#[derive(Error, Debug)]
pub enum M2AnkiError {
    #[error("error from AnkiConnect API: '{0}'")]
    APIError(#[from] APIError),
    #[error("Deck error: {0}")]
    DeckError(#[from] DeckError),
    #[error("cannot find deck with name: '{0}'")]
    DeckNameNotFound(String),
    #[error("error: {0}")]
    GenericError(#[from] GenericError),
    #[error("error: {0}")]
    InvalidOperation(String),
    #[error("invalid utf8 path")]
    InvalidUtf8Path,
    #[error("Deck error: {0}")]
    IOError(#[from] IOError),
    #[error("error fetching model from Anki: {0}")]
    ModelFetchError(String),
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
        Self::GenericError(value.into())
    }
}

impl From<jwalk::Error> for M2AnkiError {
    fn from(value: jwalk::Error) -> Self {
        Self::IOError(value.into())
    }
}
