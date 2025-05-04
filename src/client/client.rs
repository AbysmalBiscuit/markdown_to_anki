use std::sync::Arc;

use serde::{Deserialize, Serialize};

use super::{error::APIError, http_client::HttpClient, model::ModelClient, response::Response};

#[derive(Debug, Clone)]
pub struct AnkiConnectClient {
    http_client: Arc<HttpClient>,
    pub model: ModelClient,
}

impl AnkiConnectClient {
    pub fn new(url: Option<&str>, port: Option<usize>) -> Self {
        let http_client = Arc::new(HttpClient::new(url, port));
        AnkiConnectClient {
            http_client: http_client.clone(),
            model: ModelClient::new(http_client),
        }
    }

    pub fn test_connection(&self) -> Result<bool, APIError> {
        match self
            .http_client
            .request_with_timeout::<TestConnectionParams, TestConnectionParams>(
                "apiReflect",
                Some(TestConnectionParams {
                    scopes: vec!["actions".into()],
                    actions: vec!["apiReflect".into()],
                }),
                1,
            ) {
            Ok(response) => Ok(true),
            Err(err) => match err {
                APIError::UreqError(err) => Ok(false),
                _ => {
                    dbg!(&err);
                    Err(APIError::UnknownError(err.to_string()))
                }
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct TestConnectionParams {
    scopes: Vec<String>,
    actions: Vec<String>,
}
