use ankiconnect_rs::{AnkiClient, AnkiConnectError, Deck, DeckId, Model};
use rayon::prelude::*;

use super::error::CombinedAnkiError;

pub trait Find {
    fn find_model(&self, model_name: &str) -> Result<Model, CombinedAnkiError>;
    fn find_deck(&self, deck_name: &str) -> Result<Deck, CombinedAnkiError>;
    fn find_deck_by_id(&self, id: DeckId) -> Result<Deck, CombinedAnkiError>;
    fn find_or_create_deck(&self, deck_name: &str) -> Deck;
}

impl Find for AnkiClient {
    fn find_model(&self, model_name: &str) -> Result<Model, CombinedAnkiError> {
        self.models()
            .get_all()?
            .into_par_iter()
            .find_any(|model| model.name().eq(model_name))
            .ok_or(CombinedAnkiError::AnkiConnectError(
                AnkiConnectError::ModelNotFound(model_name.to_string()),
            ))
    }
    fn find_deck(&self, deck_name: &str) -> Result<Deck, CombinedAnkiError> {
        self.decks()
            .get_all()?
            .into_par_iter()
            .find_any(|deck| deck.name().eq(deck_name))
            .ok_or(CombinedAnkiError::AnkiConnectError(
                AnkiConnectError::DeckNotFound(deck_name.to_string()),
            ))
    }

    fn find_deck_by_id(&self, id: DeckId) -> Result<Deck, CombinedAnkiError> {
        self.decks()
            .get_all()?
            .into_par_iter()
            .find_any(|deck| deck.id() == id)
            .ok_or(CombinedAnkiError::AnkiConnectError(
                AnkiConnectError::DeckNotFound(id.0.to_string()),
            ))
    }

    fn find_or_create_deck(&self, deck_name: &str) -> Deck {
        let found_deck: Result<Deck, CombinedAnkiError> = self.find_deck(deck_name);
        match found_deck {
            Ok(deck) => deck,
            Err(_) => {
                let deck_id = self.decks().create(deck_name).unwrap();
                self.find_deck_by_id(deck_id).unwrap()
            }
        }
    }
}
