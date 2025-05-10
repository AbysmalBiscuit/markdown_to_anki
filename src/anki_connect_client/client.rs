use std::sync::Arc;

use serde::{Deserialize, Serialize};

use super::{
    cards::CardClient, decks::DeckClient, error::APIError, http_client::HttpClient,
    models::ModelClient, notes::NotesClient, response::Response,
};

#[derive(Debug, Clone)]
pub struct AnkiConnectClient {
    http_client: Arc<HttpClient>,
    pub cards: CardClient,
    pub decks: DeckClient,
    pub models: ModelClient,
    pub notes: NotesClient,
}

impl AnkiConnectClient {
    pub fn new(url: Option<&str>, port: Option<usize>) -> Self {
        let http_client = Arc::new(HttpClient::new(url, port));
        AnkiConnectClient {
            http_client: http_client.clone(),
            cards: CardClient::new(http_client.clone()),
            decks: DeckClient::new(http_client.clone()),
            models: ModelClient::new(http_client.clone()),
            notes: NotesClient::new(http_client),
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
                Some(1),
            ) {
            Ok(_) => Ok(true),
            Err(err) => match err {
                APIError::UreqError(_) => Ok(false),
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
