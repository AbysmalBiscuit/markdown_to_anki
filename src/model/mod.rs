use basic::Basic;
use enum_dispatch::enum_dispatch;
use rule::Rule;
use strum::{Display, EnumString};
use traits::InternalModelMethods;
use word::Word;

use crate::client::error::APIError;                                                                              ▐
use crate::Callout;
use crate::client::AnkiConnectClient;
use crate::client::model::model_response::Model;
use crate::client::note::Note;

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
