use std::{collections::HashMap, sync::Arc};

use super::{AnkiConnectClient, error::APIError, http_client::HttpClient, response::Response};

#[derive(Debug, Clone)]
pub struct CardsClient<'a>(pub &'a AnkiConnectClient);

impl CardsClient<'_> {}

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
