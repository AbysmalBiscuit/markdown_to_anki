#![allow(unused)]

// use crate::anki::{internal_model::InternalModel, internal_note::InternalNote};

use serde::Serialize;

use super::traits::InternalModelMethods;
use crate::anki_connect_client::AnkiConnectClient;
use crate::anki_connect_client::error::APIError;
use crate::anki_connect_client::model::Model;
use crate::anki_connect_client::note::Note;
use crate::callout::Callout;

#[derive(Debug, Default, Serialize)]
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
    fn to_note(self, model: Model) -> Result<Note, APIError> {
        todo!()
    }
    fn create_model(&self, client: &AnkiConnectClient, css: &str) -> Result<Model, APIError> {
        todo!()
    }
    fn from_callout(&self, callout: &Callout, header_lang: Option<&str>) -> Self {
        todo!()
    }
    fn to_add_note<'a>(
        &'a self,
        deck_name: &'a str,
        model_name: &'a str,
    ) -> crate::anki_connect_client::notes::params::AddNote<'a> {
        todo!()
    }
}
