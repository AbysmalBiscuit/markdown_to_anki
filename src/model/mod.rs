use std::error::Error;

use basic::Basic;
use rule::Rule;
use strum::{Display, EnumString};
use traits::InternalModel;
use word::Word;

use crate::error::GenericError;

pub(crate) mod basic;
pub(crate) mod rule;
pub(crate) mod traits;
pub(crate) mod word;

#[derive(Debug, Display, EnumString)]
#[strum(serialize_all = "PascalCase")]
pub enum ModelType {
    Basic(Basic),
    // Rule(Rule),
    // Word(Word),
}
