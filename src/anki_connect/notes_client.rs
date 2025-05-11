use crate::model::{InternalModelMethods, ModelType};

use super::{
    AnkiConnectClient, client::ClientBehavior, error::APIError, note::NoteId, response::Response,
};

use rayon::prelude::*;

#[derive(Debug, Clone)]
pub struct NotesClient<'a>(pub &'a AnkiConnectClient);

impl NotesClient<'_> {
    /// Returns an array of note IDs for a given query.
    pub fn find_notes(&self, query: &str) -> Result<Vec<NoteId>, APIError> {
        let response: Response<Vec<NoteId>> = self
            .0
            .request("findNotes", Some(params::FindNotes::new(query)))?;
        Ok(response.result.unwrap())
    }

    /// Gets all notes stored in a deck based on deck_name.
    pub fn find_notes_by_deck_name(&self, deck_name: &str) -> Result<Vec<NoteId>, APIError> {
        let notes = self.find_notes(&format!("deck:{}", deck_name))?;

        Ok(notes)
    }

    /// Creates a note using the given deck and model, with the provided field values and tags.
    /// Returns the identifier of the created note created on success, and null on failure.
    ///
    /// Anki-Connect can download audio, video, and picture files and embed them in newly created
    /// notes. The corresponding audio, video, and picture note members are optional and can be
    /// omitted. If you choose to include any of them, they should contain a single object or an
    /// array of objects with the mandatory filename field and one of data, path or url. Refer to
    /// the documentation of storeMediaFile for an explanation of these fields. The skipHash field
    /// can be optionally provided to skip the inclusion of files with an MD5 hash that matches the
    /// provided value. This is useful for avoiding the saving of error pages and stub files. The
    /// fields member is a list of fields that should play audio or video, or show a picture when
    /// the card is displayed in Anki. The allowDuplicate member inside options group can be set to
    /// true to enable adding duplicate cards. Normally duplicate cards can not be added and
    /// trigger exception.
    ///
    /// The duplicateScope member inside options can be used to specify the scope for which
    /// duplicates are checked. A value of "deck" will only check for duplicates in the target
    /// deck; any other value will check the entire collection.
    ///
    /// - duplicateScopeOptions.deckName will specify which deck to use for checking duplicates in.
    ///   If undefined or null, the target deck will be used.
    /// - duplicateScopeOptions.checkChildren will change whether or not duplicate cards are checked
    ///   in child decks. The default value is false.
    /// - duplicateScopeOptions.checkAllModels specifies whether duplicate checks are performed
    ///   across all note types. The default value is false.
    pub fn add_note(&self, add_note: params::AddNoteNote) -> Result<NoteId, APIError> {
        self.0
            .request("addNote", Some(params::AddNote::new(add_note)))
            .map(|result| result.result.unwrap())
    }

    // pub fn add_note_from_model_type(&self, note: &ModelType, ) -> Result<NoteId, APIError> {
    //     self.add_note(note.to_add_note(deck_name, model_name))
    // }

    /// Creates multiple notes using the given deck and model, with the provided field values and
    /// tags. Returns an array of identifiers of the created notes.
    /// If errors occur, then no notes are added and instead a string representing a Python list
    /// describing all errors is returned.
    pub fn add_notes(&self, notes: params::AddNotes) -> Result<Vec<NoteId>, APIError> {
        self.0
            .request("addNotes", Some(notes))
            .map(|result| result.result.unwrap())
    }

    // pub fn update_note_fields(&self, note: &ModelType) -> Result<bool, APIError> {
    //     self.0.http_client
    // }

    #[inline]
    pub fn notes_to_add_notes<'a>(
        notes: &'a Vec<ModelType>,
        deck_name: &'a str,
        model_name: &'a str,
    ) -> Vec<params::AddNoteNote<'a>> {
        notes
            .par_iter()
            .map(|note| note.to_add_note(deck_name, model_name))
            .collect()
    }
}

pub mod params {
    use std::collections::HashMap;

    use derive_new::new;
    use serde::Serialize;

    #[derive(Debug, Serialize, new)]
    #[serde(rename_all = "camelCase")]
    pub struct DeleteDecks {
        decks: Vec<String>,
        cards_too: bool,
    }

    #[derive(Debug, Serialize, new)]
    #[serde(rename_all = "camelCase")]
    pub struct AddNote<'a> {
        note: AddNoteNote<'a>,
    }

    #[derive(Debug, Serialize, new)]
    #[serde(rename_all = "camelCase")]
    pub struct AddNotes<'a> {
        notes: Vec<AddNoteNote<'a>>,
    }

    #[derive(Debug, Serialize, new)]
    #[serde(rename_all = "camelCase")]
    pub struct AddNoteNote<'a> {
        deck_name: &'a str,
        model_name: &'a str,
        fields: HashMap<&'a str, &'a str>,
        options: AddNoteOptions<'a>,
        #[serde(default, deserialize_with = "default_on_invalid")]
        tags: Vec<&'a str>,
        #[serde(skip_serializing, skip_deserializing)]
        audio: Option<()>,
        #[serde(skip_serializing, skip_deserializing)]
        video: Option<()>,
        #[serde(skip_serializing, skip_deserializing)]
        picture: Option<()>,
    }

    #[derive(Debug, Serialize, new)]
    #[serde(rename_all = "camelCase")]
    pub struct AddNoteOptions<'a> {
        allow_duplicate: bool,
        duplicate_scope: &'a str,
        options: DuplicateScopeOptions<'a>,
    }

    #[derive(Debug, Serialize, new)]
    #[serde(rename_all = "camelCase")]
    pub struct DuplicateScopeOptions<'a> {
        #[serde(default = "default_duplicate_scope_options_deck_name")]
        deck_name: &'a str,
        #[serde(default)]
        check_children: bool,
        #[serde(default)]
        check_all_models: bool,
    }

    fn default_duplicate_scope_options_deck_name<'a>() -> &'a str {
        "Default"
    }

    #[derive(Debug, Serialize, new)]
    #[serde(rename_all = "camelCase")]
    pub struct FindNotes<'a> {
        query: &'a str,
    }
}
