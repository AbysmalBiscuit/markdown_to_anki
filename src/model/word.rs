#![allow(unused)]
use ankiconnect_rs::NoteError;

use crate::anki::{internal_model::InternalModel, internal_note::InternalNote};

use super::traits::InternalModelMethods;

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
    fn to_note(self, model: ankiconnect_rs::Model) -> Result<ankiconnect_rs::Note, NoteError> {
        todo!()
    }
    fn create_model(
        &self,
        client: &ankiconnect_rs::AnkiClient,
        css: &str,
    ) -> Result<ankiconnect_rs::models::ModelId, ankiconnect_rs::AnkiError> {
        todo!()
    }
    fn from_callout(&self, callout: &crate::callout::Callout, header_lang: Option<&str>) -> Self {
        todo!()
    }
}
