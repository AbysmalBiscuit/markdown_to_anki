use ankiconnect_rs::NoteError;
use basic::Basic;
use enum_dispatch::enum_dispatch;
use rule::Rule;
use strum::{Display, EnumString};
use traits::InternalModelMethods;
use word::Word;

use crate::Callout;
use ankiconnect_rs::AnkiClient;
use ankiconnect_rs::AnkiError;
use ankiconnect_rs::models::ModelId;

pub(crate) mod basic;
pub(crate) mod rule;
pub(crate) mod traits;
pub(crate) mod word;

#[derive(Debug, Display, EnumString)]
#[strum(serialize_all = "PascalCase")]
#[enum_dispatch(InternalModelMethods)]
pub enum ModelType {
    Basic(Basic),
    Rule(Rule),
    Word(Word),
}

impl Default for ModelType {
    fn default() -> Self {
        ModelType::Basic(Basic::default())
    }
}
