use std::fmt::Debug;

use ankiconnect_rs::error::Result;
use ankiconnect_rs::models::ModelId;
use ankiconnect_rs::{AnkiClient, Deck, DuplicateScope, Field, Note, NoteId};

use crate::callout::Callout;

pub trait CreateModel {
    fn create_model(&self, client: &AnkiClient, css: &str) -> Result<ModelId>;
}
pub trait InternalModel: CreateModel + Debug + Default {
    fn from_callout(callout: &Callout, header_lang: Option<&str>) -> Self;
}
