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

    /// Gets ids of all notes stored in a deck based on deck_name.
    pub fn find_notes_ids_by_deck_name(&self, deck_name: &str) -> Result<Vec<NoteId>, APIError> {
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

    /// Modify the fields of an existing note. You can also include audio, video, or picture files
    /// which will be added to the note with an optional audio, video, or picture property. Please
    /// see the documentation for addNote for an explanation of objects in the audio, video, or
    /// picture array.
    pub fn update_note_fields(&self, note: params::UpdateNoteFields) -> Result<bool, APIError> {
        self.0
            .request::<(), _>("updateNoteFields", Some(note))
            .map(|_| true)
    }

    pub fn update_note(&self, id: NoteId, note: &ModelType) -> Result<bool, APIError> {
        self.update_note_fields(params::UpdateNoteFields::new(
            params::UpdateNoteFieldsNote::new(id, note.get_fields(), None, None, None),
        ))
    }

    pub fn notes_info_by_id(
        &self,
        ids: &Vec<NoteId>,
    ) -> Result<Vec<responses::NoteInfo>, APIError> {
        self.0
            .request("notesInfo", Some(params::NotesInfoIds::new(ids)))
            .map(|response| response.result.unwrap())
    }

    pub fn notes_info_by_query(&self, query: &str) -> Result<Vec<responses::NoteInfo>, APIError> {
        self.0
            .request("notesInfo", Some(params::NotesInfoQuery::new(query)))
            .map(|response| response.result.unwrap())
    }

    /// Gets ids of all notes stored in a deck based on deck_name.
    pub fn get_notes_by_deck_name(
        &self,
        deck_name: &str,
    ) -> Result<Vec<responses::NoteInfo>, APIError> {
        let query = format!("deck:{}", deck_name);
        self.notes_info_by_query(&format!("deck:{}", deck_name))
    }

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

    use crate::anki_connect::note::NoteId;

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
        #[serde(skip_serializing)]
        audio: Option<()>,
        #[serde(skip_serializing)]
        video: Option<()>,
        #[serde(skip_serializing)]
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

    // notesInfo
    #[derive(Debug, Serialize, new)]
    #[serde(rename_all = "camelCase")]
    pub struct NotesInfoIds<'a> {
        notes: &'a Vec<NoteId>,
    }

    #[derive(Debug, Serialize, new)]
    #[serde(rename_all = "camelCase")]
    pub struct NotesInfoQuery<'a> {
        query: &'a str,
    }

    // updateNoteFields
    #[derive(Debug, Serialize, new)]
    #[serde(rename_all = "camelCase")]
    pub struct UpdateNoteFields<'a> {
        note: UpdateNoteFieldsNote<'a>,
    }
    #[derive(Debug, Serialize, new)]
    #[serde(rename_all = "camelCase")]
    pub struct UpdateNoteFieldsNote<'a> {
        id: NoteId,
        fields: HashMap<&'a str, &'a str>,
        #[serde(skip_serializing)]
        audio: Option<()>,
        #[serde(skip_serializing)]
        video: Option<()>,
        #[serde(skip_serializing)]
        picture: Option<()>,
    }
}

mod responses {
    use std::collections::HashMap;

    use rayon::prelude::*;
    use serde::Deserialize;
    use serde::Deserializer;
    use serde_json::Value;

    use crate::anki_connect::{card::CardId, note::NoteId};

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct NoteInfo {
        note_id: NoteId,
        profile: String,
        model_name: String,
        tags: Vec<String>,
        #[serde(deserialize_with = "deserialize_note_field")]
        fields: Vec<(String, String)>,
        #[serde(rename = "mod")]
        mtime: u64,
        cards: Vec<CardId>,
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct NoteField {
        value: String,
        order: u32,
    }

    #[derive(Debug, Deserialize)]
    struct NoteFieldResponse {
        value: String,
        order: u32,
    }

    fn deserialize_note_field<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Vec<(String, String)>, D::Error> {
        // value: HashMap<String, NoteFieldResponse>
        let value: HashMap<String, NoteFieldResponse> = Deserialize::deserialize(deserializer)?;

        let mut data = value
            .into_par_iter()
            .map(|(k, note_field_response)| {
                (note_field_response.order, k, note_field_response.value)
            })
            .collect::<Vec<_>>();

        data.par_sort_unstable_by_key(|(key, _, _)| *key);

        Ok(data
            .into_par_iter()
            .map(|(_, name, value)| (name, value))
            .collect())
    }
}
