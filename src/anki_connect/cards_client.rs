use super::{
    AnkiConnectClient, card::CardId, client::ClientBehavior, error::APIError, response::Response,
};

/// Card actions.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CardsClient<'a>(pub &'a AnkiConnectClient);

impl CardsClient<'_> {
    /// Returns an array of card IDs for a given query. Functionally identical to guiBrowse but doesn't use the GUI for better performance.
    pub fn find_cards(&self, query: &str) -> Result<Vec<CardId>, APIError> {
        let response: Response<Vec<CardId>> = self
            .0
            .request("findCards", Some(params::FindCards::new(query)))?;
        Ok(response.result.unwrap_or_default())
    }
}

pub mod params {
    use derive_new::new;
    use serde::Serialize;

    #[derive(Debug, Serialize, new)]
    #[serde(rename_all = "camelCase")]
    pub struct FindCards<'a> {
        query: &'a str,
    }
}
