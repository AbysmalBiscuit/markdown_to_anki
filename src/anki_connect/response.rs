use std::fmt::{Debug, Display};

use serde::Deserialize;

use super::error::APIError;

#[derive(Debug, Deserialize)]
pub struct Response<R: Debug> {
    pub result: Option<R>,
    pub error: Option<String>,
}

impl<R> From<Result<R, APIError>> for Response<R>
where
    R: Debug,
{
    fn from(value: Result<R, APIError>) -> Self {
        match value {
            Ok(v) => Self {
                result: Some(v),
                error: None,
            },
            Err(err) => Self {
                result: None,
                error: Some(err.to_string()),
            },
        }
    }
}

impl<R: Debug> Display for Response<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
