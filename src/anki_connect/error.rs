use strum::Display;
use thiserror::Error;

#[derive(Debug, Display, Error)]

pub enum APIError {
    AnkiConnectError(String),
    FailedConnection(String),
    UnknownError(String),
    UreqError(ureq::Error),
    DeckNotFound,
    ModelNotFound(String),
}
