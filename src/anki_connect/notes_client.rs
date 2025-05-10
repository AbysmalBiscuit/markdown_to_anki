use super::{AnkiConnectClient, error::APIError, note::NoteId, response::Response};

#[derive(Debug, Clone)]
pub struct NotesClient<'a>(pub &'a AnkiConnectClient);

impl NotesClient<'_> {
    /// Returns an array of note IDs for a given query.
    pub fn find_notes(&self, query: &str) -> Result<Vec<NoteId>, APIError> {
        let response: Response<Vec<NoteId>> = self
            .0
            .http_client
            .request("findNotes", Some(params::FindNotes::new(query)))?;
        Ok(response.result.unwrap())
    }

    /// Gets all notes stored in a deck based on deck_name.
    pub fn find_notes_by_deck_name(&self, deck_name: &str) -> Result<Vec<NoteId>, APIError> {
        let notes = self.find_notes(&format!("deck:{}", deck_name))?;

        Ok(notes)
    }

    pub fn add_note(&self, add_note: params::AddNote) -> Result<NoteId, APIError> {
        self.0
            .http_client
            .request("addNote", Some(params::AddNoteNote::new(add_note)))
            .map(|result| result.result.unwrap())
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
    pub struct AddNoteNote<'a> {
        note: AddNote<'a>,
    }

    #[derive(Debug, Serialize, new)]
    #[serde(rename_all = "camelCase")]
    pub struct AddNote<'a> {
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
