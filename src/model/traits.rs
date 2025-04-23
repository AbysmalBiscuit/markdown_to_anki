use std::fmt::Debug;

use ankiconnect_rs::error::Result;
use ankiconnect_rs::models::ModelId;
use ankiconnect_rs::{AnkiClient, Deck, DuplicateScope, Field, Note, NoteId};

use crate::callout::Callout;

pub trait AddNote {
    fn add_note(
        &self,
        deck_name: &ankiconnect_rs::Deck,
        model: &ankiconnect_rs::Model,
        note: ankiconnect_rs::Note,
        allow_duplicate: bool,
        duplicate_scope: Option<ankiconnect_rs::DuplicateScope>,
    ) -> Result<NoteId>;
}

pub trait CreateModel {
    fn create_model(&self, client: &AnkiClient, css: &str) -> Result<ModelId>;
}

pub trait FromCallout {
    fn from_callout(callout: &Callout, header_lang: Option<&str>) -> Self;
}

// pub trait GenerateFields {
//     fn generate_fields(&self) -> Vec<Field>;
// }

pub trait InternalModel: AddNote + CreateModel + FromCallout + Debug {}
