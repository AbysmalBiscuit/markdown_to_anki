use std::fmt::Debug;

use ankiconnect_rs::models::ModelId;
use ankiconnect_rs::{AnkiClient, AnkiError, NoteError};
use enum_dispatch::enum_dispatch;

use crate::anki::internal_model::InternalModel;
use crate::anki::internal_note::InternalNote;
use crate::callout::Callout;

#[enum_dispatch]
pub trait InternalModelMethods: Debug + Default {
    fn from_callout(&self, callout: &Callout, header_lang: Option<&str>) -> Self;
    fn create_model(&self, client: &AnkiClient, css: &str) -> Result<ModelId, AnkiError>;
    fn to_note(self, model: ankiconnect_rs::Model) -> Result<ankiconnect_rs::Note, NoteError>;
}
