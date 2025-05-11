mod basic;
// mod rule;
// mod word;

use crate::anki_connect::models_client::params::CreateModel;
use crate::anki_connect::{APIError, model::Model, note::Note, notes_client::params::AddNoteNote};
use crate::callout::Callout;

use basic::Basic;
// use rule::Rule;
// use word::Word;

use enum_dispatch::enum_dispatch;
use serde::Serialize;
use std::collections::HashMap;
use std::fmt::Debug;
use strum::{Display, EnumString};

#[derive(Debug, Display, EnumString, Serialize)]
#[strum(serialize_all = "PascalCase")]
#[enum_dispatch(InternalModelMethods)]
pub enum ModelType {
    Basic(Basic),
    // Rule(Rule),
    // Word(Word),
}

impl Default for ModelType {
    fn default() -> Self {
        ModelType::Basic(Basic::default())
    }
}

#[enum_dispatch]
pub trait InternalModelMethods: Debug + Default + Serialize {
    fn from_callout(&self, callout: &Callout, header_lang: Option<&str>) -> Self;
    fn to_create_model<'a>(&self, model_name: &'a str, css: Option<&'a str>) -> CreateModel<'a>;
    fn get_fields<'a>(&'a self) -> HashMap<&'a str, &'a str>;
    fn to_note(self, model: Model) -> Result<Note, APIError>;
    fn to_add_note<'a>(&'a self, deck_name: &'a str, model_name: &'a str) -> AddNoteNote<'a>;
}
