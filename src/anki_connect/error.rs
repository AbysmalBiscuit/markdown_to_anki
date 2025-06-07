use strum::Display;
use thiserror::Error;

#[derive(Debug, Display, Error)]
pub enum APIError {
    AnkiConnectError(String),
    FailedConnection(String),
    UnknownError(String),
    UreqError(#[from] ureq::Error),
    DeckNotFound,
    // ModelNotFound(String),
}
