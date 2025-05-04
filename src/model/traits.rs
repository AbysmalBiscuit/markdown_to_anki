use std::fmt::Debug;

use enum_dispatch::enum_dispatch;

use crate::callout::Callout;
use crate::client::AnkiConnectClient;
use crate::client::error::APIError;
use crate::client::model::model_response::Model;
use crate::client::note::Note;

#[enum_dispatch]
pub trait InternalModelMethods: Debug + Default {
    fn from_callout(&self, callout: &Callout, header_lang: Option<&str>) -> Self;
    fn create_model(&self, client: &AnkiConnectClient, css: &str) -> Result<Model, APIError>;
    fn to_note(self, model: Model) -> Result<Note, APIError>;
}
