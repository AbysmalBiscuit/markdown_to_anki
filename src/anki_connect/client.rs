use serde::{Deserialize, Serialize};

use super::{
    cards_client::CardsClient, decks_client::DecksClient, error::APIError, http_client::HttpClient,
    models_client::ModelsClient, notes_client::NotesClient,
};

#[derive(Debug, Clone)]
pub struct AnkiConnectClient {
    pub http_client: HttpClient,
}

impl AnkiConnectClient {
    pub fn new(url: Option<&str>, port: Option<usize>) -> Self {
        let http_client = HttpClient::new(url, port);
        AnkiConnectClient { http_client }
    }

    pub fn cards(&self) -> CardsClient<'_> {
        CardsClient(self)
    }

    pub fn decks(&self) -> DecksClient<'_> {
        DecksClient(self)
    }

    pub fn models(&self) -> ModelsClient<'_> {
        ModelsClient(self)
    }

    pub fn notes(&self) -> NotesClient<'_> {
        NotesClient(self)
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
