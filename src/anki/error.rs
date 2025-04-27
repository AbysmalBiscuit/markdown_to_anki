use std::{error::Error, fmt::Display};
use strum::Display;

use ankiconnect_rs::{AnkiConnectError, AnkiError};
#[derive(Debug, Display)]
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

impl Error for CombinedAnkiError {}

#[derive(Debug, Display)]
pub enum APIError {}
impl Error for APIError {}
