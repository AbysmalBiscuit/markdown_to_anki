use std::error::Error;

use ankiconnect_rs::Model;
use ankiconnect_rs::NoteError;
use basic::Basic;
use enum_dispatch::enum_dispatch;
use rule::Rule;
use strum::{Display, EnumString};
use traits::InternalModel;
use word::Word;

use crate::Callout;
use crate::error::GenericError;
use ankiconnect_rs::AnkiClient;
use ankiconnect_rs::models::ModelId;
use ankiconnect_rs::{AnkiError, Note};

pub(crate) mod basic;
pub(crate) mod rule;
pub(crate) mod traits;
pub(crate) mod word;

#[derive(Debug, Display, EnumString)]
#[strum(serialize_all = "PascalCase")]
// #[derive(Debug)]
#[enum_dispatch(InternalModel)]
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
