use std::fmt::Debug;

use ankiconnect_rs::models::ModelId;
use ankiconnect_rs::{AnkiClient, AnkiError, Model, Note, NoteError};
use enum_dispatch::enum_dispatch;

use crate::callout::Callout;

#[enum_dispatch]
pub trait InternalModel: Debug + Default {
    fn from_callout(&self, callout: &Callout, header_lang: Option<&str>) -> Self;
    fn create_model(&self, client: &AnkiClient, css: &str) -> Result<ModelId, AnkiError>;
    fn to_note(self, model: Model) -> Result<Note, NoteError>;
}
