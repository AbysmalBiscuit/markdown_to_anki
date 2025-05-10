use basic::Basic;
use enum_dispatch::enum_dispatch;
use rule::Rule;
use strum::{Display, EnumString};
use traits::InternalModelMethods;
use word::Word;

use crate::Callout;
use serde::Serialize;

use crate::anki_connect_client::AnkiConnectClient;
use crate::anki_connect_client::error::APIError;
use crate::anki_connect_client::model::Model;
use crate::anki_connect_client::note::Note;
use crate::anki_connect_client::notes::params::AddNote;

pub(crate) mod basic;
pub(crate) mod rule;
pub(crate) mod traits;
pub(crate) mod word;

#[derive(Debug, Display, EnumString, Serialize)]
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
