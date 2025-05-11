use super::ClientBehavior;
use std::time::Duration;

use crate::anki_connect::{error::APIError, params::Params, response::Response};
use serde::{Serialize, de::DeserializeOwned};
use ureq::Agent;

#[derive(Debug, Clone)]
pub struct UreqClient {
    agent: Agent,
    url: String,
}

impl UreqClient {
    pub fn new(url: Option<&str>, port: Option<u32>) -> Self {
        let config = Agent::config_builder()
            .timeout_global(Some(Duration::from_secs(5)))
            .build();
        UreqClient {
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
    ) -> Result<Response<R>, APIError>
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
                dbg!(&response);
                if response.error.is_some() {
                    Err(APIError::AnkiConnectError(response.error.unwrap()))
                } else {
                    Ok(response)
                }
            }
            Err(err) => {
                dbg!(&err);
                Err(APIError::UnknownError(err.to_string()))
            }
        }
    }

    fn request<R, P>(&self, action: &str, params: Option<P>) -> Result<Response<R>, APIError>
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
                if response.error.is_some() {
                    Err(APIError::AnkiConnectError(response.error.unwrap()))
                } else {
                    Ok(response)
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
        UreqClient {
            agent: config.into(),
            url: format!("{}:{}", "http://localhost", 8765),
        }
    }
}
