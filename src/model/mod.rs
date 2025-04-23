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
pub enum ModelType {
    Basic(Basic),
    // Rule(Rule),
    // Word(Word),
}

pub fn get_internal_model(model_type: &str) -> Result<impl InternalModel, GenericError> {
    match model_type {
        "Basic" => Ok(Basic {
            front: "".into(),
            back: "".into(),
        }),
        _ => Err("Unknown model type".into()),
    }
}
