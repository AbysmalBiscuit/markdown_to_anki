use std::collections::HashMap;

use super::{
    AnkiConnectClient, client::ClientBehavior, deck::DeckId, error::APIError, response::Response,
};

use rayon::prelude::*;

#[derive(Debug, Clone)]
pub struct DecksClient<'a>(pub &'a AnkiConnectClient);

#[allow(unused)]
impl DecksClient<'_> {
    /// Gets the complete list of deck names for the current user.
    pub fn deck_names(&self) -> Result<Vec<String>, APIError> {
        let response: Response<Vec<String>> = self.0.request("deckNames", None::<()>)?;
        Ok(response.result.unwrap())
    }

    /// Gets the complete list of deck names and their respective IDs for the current user.
    pub fn deck_names_and_ids(&self) -> Result<HashMap<String, DeckId>, APIError> {
        let response: Response<HashMap<String, DeckId>> =
            self.0.request("deckNamesAndIds", None::<()>)?;
        Ok(response.result.unwrap())
    }

    pub fn find_deck_id_by_name(&self, name: &str) -> Result<DeckId, APIError> {
        let decks = self.deck_names_and_ids()?;
        match decks.get(name) {
            Some(id) => Ok(id.to_owned()),
            None => Err(APIError::DeckNotFound),
        }
    }

    /// Create a new empty deck. Will not overwrite a deck that exists with the same name.
    pub fn create_deck(&self, deck_name: &str) -> Result<DeckId, APIError> {
        self.0
            .request("createDeck", Some(params::CreateDeck::new(deck_name)))
            .map(|response| response.result.unwrap())
    }

    pub fn find_or_create_deck(&self, deck_name: &str) -> Result<DeckId, APIError> {
        let deck = self.find_deck_id_by_name(deck_name);
        if deck.is_err() {
            self.create_deck(deck_name)
        } else {
            deck
        }
    }

    /// Deletes decks with the given names.
    pub fn delete_decks(&self, decks: Vec<&str>) -> Result<bool, APIError> {
        self.0
            .request::<Option<()>, _>(
                "deleteDecks",
                Some(params::DeleteDecks::new(
                    decks.into_par_iter().map(|name| name.to_string()).collect(),
                    true,
                )),
            )
            .map(|_| true)
    }

    /// Deletes the deck with the given name.
    pub fn delete(&self, deck_name: &str) -> Result<bool, APIError> {
        self.delete_decks(vec![deck_name])
    }
}

pub mod params {
    use derive_new::new;
    use serde::Serialize;

    #[derive(Debug, Serialize, new)]
    #[serde(rename_all = "camelCase")]
    pub struct CreateDeck<'a> {
        deck: &'a str,
    }

    #[derive(Debug, Serialize, new)]
    #[serde(rename_all = "camelCase")]
    pub struct DeleteDecks {
        decks: Vec<String>,
        cards_too: bool,
    }
}
