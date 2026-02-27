#![allow(dead_code)]
/// This module provides the [`AnkiConnectClient`].
pub mod anki_connect_client;
pub mod card;
pub mod cards_client;
mod client;
pub mod deck;
pub mod decks_client;
pub mod error;
pub mod model;
pub mod models_client;
pub mod note;
pub mod notes_client;
mod params;
pub mod response;
mod util;

pub use anki_connect_client::AnkiConnectClient;
