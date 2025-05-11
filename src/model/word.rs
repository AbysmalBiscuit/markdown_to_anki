#![allow(unused)]

use serde::Serialize;

use super::InternalModelMethods;
use crate::anki_connect::error::APIError;
use crate::anki_connect::model::Model;
use crate::anki_connect::models_client::params::CreateModel;
use crate::anki_connect::note::Note;
use crate::callout::Callout;

#[derive(Debug, Default, Serialize)]
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
    fn from_callout(&self, callout: &Callout, header_lang: Option<&str>) -> Self {
        todo!()
    }
    fn to_create_model(&self, model_name: &str, css: Option<&str>) -> CreateModel {
        todo!()
    }
    fn to_add_note<'a>(
        &'a self,
        deck_name: &'a str,
        model_name: &'a str,
    ) -> crate::anki_connect::notes_client::params::AddNoteNote<'a> {
        todo!()
    }
}
