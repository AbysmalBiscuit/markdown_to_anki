use std::fmt::Debug;

use enum_dispatch::enum_dispatch;
use serde::Serialize;

use crate::anki_connect_client::AnkiConnectClient;
use crate::anki_connect_client::error::APIError;
use crate::anki_connect_client::model::Model;
use crate::anki_connect_client::note::Note;
use crate::anki_connect_client::notes::params::AddNote;
use crate::callout::Callout;

#[enum_dispatch]
pub trait InternalModelMethods: Debug + Default + Serialize {
    fn from_callout(&self, callout: &Callout, header_lang: Option<&str>) -> Self;
    fn create_model(&self, client: &AnkiConnectClient, css: &str) -> Result<Model, APIError>;
    fn to_note(self, model: Model) -> Result<Note, APIError>;
    fn to_add_note<'a>(&'a self, deck_name: &'a str, model_name: &'a str) -> AddNote<'a>;
}
