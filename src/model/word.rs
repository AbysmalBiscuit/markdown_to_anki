#![allow(unused)]
use super::traits::InternalModel;

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

impl InternalModel for Word {
    fn to_note(
        self,
        model: ankiconnect_rs::Model,
    ) -> Result<ankiconnect_rs::Note, ankiconnect_rs::NoteError> {
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
