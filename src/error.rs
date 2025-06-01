use std::error::Error;
use std::fmt::Display;
use std::io::Error as IOError;

use crate::anki_connect::error::APIError;
use crate::deck::DeckError;
use serde_json::Error as SerdeJsonError;

pub type GenericError = Box<dyn Error + Send>;
pub type GenericSyncError = Box<dyn Error + Send + Sync>;
pub type GenericSendStatic = Box<dyn Error + Send + 'static>;

#[derive(Debug)]
pub enum M2AnkiError {
    APIError(APIError),
    DeckError(DeckError),
    GenericError(GenericError),
    GenericSyncError(GenericSyncError),
    GenericSendStatic(GenericSendStatic),
    IOError(IOError),
    ThreadPanic,
    ModelParseError(strum::ParseError),
    ProgressBarError,
    SerdeJsonError(SerdeJsonError),
    CardIdNotFound(String),
    NoteIdNotFound(String),
    NoteHasNoCards,
    DeckNameNotFound(String),
    AnkiNoteNotFound(String),
}

impl Display for M2AnkiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self,)
    }
}

impl From<std::io::Error> for M2AnkiError {
    fn from(value: std::io::Error) -> Self {
        M2AnkiError::IOError(value)
    }
}

impl From<APIError> for M2AnkiError {
    fn from(value: APIError) -> Self {
        M2AnkiError::APIError(value)
    }
}

impl From<DeckError> for M2AnkiError {
    fn from(value: DeckError) -> Self {
        M2AnkiError::DeckError(value)
    }
}

impl From<GenericSendStatic> for M2AnkiError {
    fn from(value: GenericSendStatic) -> Self {
        M2AnkiError::GenericSendStatic(value)
    }
}

impl From<strum::ParseError> for M2AnkiError {
    fn from(value: strum::ParseError) -> Self {
        M2AnkiError::ModelParseError(value)
    }
}

impl From<&str> for M2AnkiError {
    fn from(value: &str) -> Self {
        M2AnkiError::GenericSyncError(value.into())
    }
}

impl From<SerdeJsonError> for M2AnkiError {
    fn from(value: SerdeJsonError) -> Self {
        M2AnkiError::SerdeJsonError(value)
    }
}
