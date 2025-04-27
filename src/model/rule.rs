#![allow(unused)]
use ankiconnect_rs::NoteError;

use crate::anki::{internal_model::InternalModel, internal_note::InternalNote};

use super::traits::InternalModelMethods;

#[derive(Debug, Default)]
pub struct Rule {
    front: String,
    back: String,
    audio: String,
    notation: String,
    quick_notes: String,
    alternate_phrasing: String,
    rules: String,
    rule_alternate_meanings: String,
    other_rules_with_similar_meanings: String,
    rule_used_but_unrelated_to_primary: String,
    irregular_rules: String,
    phonetics: String,
    references: String,
}

impl InternalModelMethods for Rule {
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
