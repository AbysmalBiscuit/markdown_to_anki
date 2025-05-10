use std::fmt::{Debug, Display};

use serde::Deserialize;

use super::error::APIError;

#[derive(Debug, Deserialize)]
pub struct Response<R: Debug> {
    pub result: Option<R>,
    pub error: Option<String>,
}

impl<R: Debug> From<Result<R, APIError>> for Response<R> {
    fn from(value: Result<R, APIError>) -> Self {
        if value.is_ok() {
            Self {
                result: Some(value.unwrap()),
                error: None,
            }
        } else {
            Self {
                result: None,
                error: Some(value.unwrap_err().to_string()),
            }
        }
    }
}

impl<R: Debug> Display for Response<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self)
    }
}
