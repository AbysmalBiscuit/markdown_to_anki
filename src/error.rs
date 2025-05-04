use std::error::Error;
use std::io::Error as IOError;

use crate::client::error::APIError;

pub type GenericError = Box<dyn Error + Send>;
pub type GenericSyncError = Box<dyn Error + Send + Sync>;
pub type GenericSendStatic = Box<dyn Error + Send + 'static>;

#[derive(Debug)]
pub enum M2AnkiError {
    GenericError(GenericError),
    GenericSyncError(GenericSyncError),
    GenericSendStatic(GenericSendStatic),
    IOError(IOError),
    APIError(APIError),
    ThreadPanic,
    ModelParseError(strum::ParseError),
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
