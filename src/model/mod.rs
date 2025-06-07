mod basic;
// mod rule;
// mod word;

use crate::anki_connect::models_client::params::CreateModel;
use crate::anki_connect::note::NoteId;
use crate::anki_connect::notes_client::params as notes_params;
use crate::anki_connect::notes_client::params::AddNoteNote;
use crate::callout::Callout;
use crate::note_operation::NoteOperation;

use basic::Basic;
// use rule::Rule;
// use word::Word;

use derive_new::new;
use enum_dispatch::enum_dispatch;
use serde::Serialize;
use std::collections::HashMap;
use std::fmt::Debug;
use strum::{Display, EnumString};

#[derive(Debug, Display, Clone, EnumString, Serialize)]
#[strum(serialize_all = "PascalCase")]
#[enum_dispatch(InternalModelMethods)]
pub enum ModelType<'a> {
    Basic(Basic<'a>),
    // Rule(Rule),
    // Word(Word),
}

impl<'a> Default for ModelType<'a> {
    fn default() -> Self {
        ModelType::Basic(Basic::default())
    }
}

#[enum_dispatch]
pub trait InternalModelMethods<'a>: Debug + Default {
    fn from_callout(
        &self,
        callout: &Callout,
        header_lang: Option<&str>,
        deck_name: &'a str,
    ) -> Self;
    fn to_create_model(&self, model_name: &'a str, css: Option<&'a str>) -> CreateModel<'a>;
    fn get_fields(&'a self) -> HashMap<&'a str, &'a str>;
    fn to_add_note(&'a self, deck_name: &'a str, model_name: &'a str) -> AddNoteNote<'a>;
    fn to_update_note(&'a self, note_id: &'a NoteId) -> notes_params::UpdateNoteFields<'a>;
    fn get_deck_name(&'a self) -> &'a str;
    fn get_operation(&'a self) -> NoteOperation;
    fn get_markdown_id(&'a self) -> &'a String;
    fn get_audio(&'a self) -> Option<&'a Vec<MediaFile<'a>>>;
    fn get_video(&'a self) -> Option<&'a Vec<MediaFile<'a>>>;
    fn get_picture(&'a self) -> Option<&'a Vec<MediaFile<'a>>>;
}

#[derive(Debug, Serialize, Clone, new)]
#[serde(rename_all = "camelCase")]
pub struct MediaFile<'a> {
    filename: &'a str,
    /// The skipHash field can be optionally provided to skip the inclusion of files with an MD5 hash that matches the provided value. This is useful for avoiding the saving of error pages and stub files.
    #[serde(skip_serializing_if = "Option::is_none")]
    skip_hash: Option<&'a str>,
    /// The fields member is a list of fields that should play audio or video, or show a picture when the card is displayed in Anki.
    fields: &'a Vec<&'a str>,
    /// Base64 encoded data that will be saved as a media file.
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<&'a str>,
    /// Relative or absolute path to the file that should be uploaded.
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<&'a str>,
    /// URL for downloading the file.
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<&'a str>,
}
