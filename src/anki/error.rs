use std::{error::Error, fmt::Display};

use ankiconnect_rs::{AnkiClient, AnkiConnectError, AnkiError, Deck, Model};
#[derive(Debug)]
pub enum CombinedAnkiError {
    AnkiError(AnkiError),
    AnkiConnectError(AnkiConnectError),
}
impl From<AnkiError> for CombinedAnkiError {
    fn from(value: AnkiError) -> Self {
        CombinedAnkiError::AnkiError(value)
    }
}

impl From<AnkiConnectError> for CombinedAnkiError {
    fn from(value: AnkiConnectError) -> Self {
        CombinedAnkiError::AnkiConnectError(value)
    }
}

impl Display for CombinedAnkiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self,)
    }
}

impl Error for CombinedAnkiError {}
