use super::ClientBehavior;
use std::time::Duration;

use crate::anki_connect::{error::APIError, params::Params, response::Response};
use serde::{Serialize, de::DeserializeOwned};
use ureq::Agent;

/// Ureq implementation of [`ClientBehavior`].
#[derive(Debug, Clone)]
pub struct UreqClient {
    /// [`ureq`] state [`Agent`].
    agent: Agent,
    /// `AnkiConnect` URL to which requsets will be made.
    url: String,
}

impl UreqClient {
    /// Creates a new [`UreqClient`].
    pub fn new(url: Option<&str>, port: Option<u32>) -> Self {
        let config = Agent::config_builder()
            .timeout_global(Some(Duration::from_secs(5)))
            .build();
        Self {
            agent: config.into(),
            url: format!(
                "{}:{}",
                url.unwrap_or("http://localhost"),
                port.unwrap_or(8765)
            ),
        }
    }
}

impl ClientBehavior for UreqClient {
    fn request_with_timeout<R, P>(
        &self,
        action: &str,
        params: Option<P>,
        timeout: Option<u8>,
    ) -> Result<R, APIError>
    where
        R: DeserializeOwned + std::fmt::Debug,
        P: Serialize + std::fmt::Debug,
    {
        match self
            .agent
            .post(&self.url)
            .config()
            .timeout_global(Some(Duration::from_secs(timeout.unwrap_or(1).into())))
            .build()
            .send_json(Params::new(action, params))
            .map_err(APIError::UreqError)?
            .body_mut()
            .read_json::<Response<R>>()
        {
            Ok(response) => {
                // dbg!(&response);
                if let Some(err) = response.error {
                    Err(APIError::AnkiConnectError(err))
                } else {
                    response
                        .result
                        .ok_or(APIError::AnkiConnectError("empty result".into()))
                }
            }
            Err(err) => {
                // dbg!(&err);
                Err(APIError::UnknownError(err.to_string()))
            }
        }
    }

    fn request<R, P>(&self, action: &str, params: Option<P>) -> Result<R, APIError>
    where
        R: DeserializeOwned + std::fmt::Debug,
        P: Serialize + std::fmt::Debug,
    {
        match self
            .agent
            .post(&self.url)
            .send_json(Params::new(action, params))
            .map_err(APIError::UreqError)?
            .body_mut()
            .read_json::<Response<R>>()
        {
            Ok(response) => {
                // trace!("{}", &response);
                // dbg!(&response);
                if let Some(err) = response.error {
                    Err(APIError::AnkiConnectError(err))
                } else {
                    response
                        .result
                        .ok_or(APIError::AnkiConnectError("empty result".into()))
                }
            }
            Err(err) => {
                // trace!("{}", &err);
                Err(APIError::UnknownError(err.to_string()))
            }
        }
    }
}

impl Default for UreqClient {
    fn default() -> Self {
        let config = Agent::config_builder()
            .timeout_global(Some(Duration::from_secs(5)))
            .build();
        Self {
            agent: config.into(),
            url: format!("{}:{}", "http://localhost", 8765),
        }
    }
}
