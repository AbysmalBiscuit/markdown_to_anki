use std::{collections::HashMap, sync::Arc};

use super::{
    deck::{Deck, DeckId},
    error::APIError,
    http_client::HttpClient,
    response::Response,
};

use rayon::prelude::*;

#[derive(Debug, Clone)]
pub struct CardClient {
    http_client: Arc<HttpClient>,
}

impl CardClient {
    pub fn new(client: Arc<HttpClient>) -> Self {
        CardClient {
            http_client: client,
        }
    }
}

pub mod params {
    use derive_new::new;
    use serde::Serialize;

    #[derive(Debug, Serialize, new)]
    #[serde(rename_all = "camelCase")]
    pub struct DeleteDecks {
        decks: Vec<String>,
        cards_too: bool,
    }
}
