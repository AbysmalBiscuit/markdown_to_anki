#![allow(unused)]
use crate::anki::{internal_model::InternalModel, internal_note::InternalNote};

use super::traits::InternalModelMethods;
use crate::callout::Callout;
use crate::client::AnkiConnectClient;
use crate::client::error::APIError;
use crate::client::model::model_response::Model;
use crate::client::note::Note;

#[derive(Debug, Default)]
pub struct Word {
    front: String,
    back: String,
    audio: String,
    notation: String,
    quick_notes: String,
    rules: String,
    examples: String,
    related_words_rules: String,
    select_conjugations: String,
    irregular_rules: String,
    additinoal_rules: String,
    phonetics: String,
    references: String,
}

impl InternalModelMethods for Word {
    fn to_note(self, model: Model) -> Result<Note, APIError> {
        todo!()
    }
    fn create_model(&self, client: &AnkiConnectClient, css: &str) -> Result<Model, APIError> {
        todo!()
    }
    fn from_callout(&self, callout: &Callout, header_lang: Option<&str>) -> Self {
        todo!()
    }
}
