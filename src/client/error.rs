use strum::Display;
use thiserror::Error;

#[derive(Debug, Display, Error)]

pub enum APIError {
    AnkiConnectError(String),
    FailedConnection,
    UnknownError(String),
    UreqError(ureq::Error),
    ModelNotFound(String),
}
