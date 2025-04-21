use crate::callout::Callout;

use super::traits::AddNote;

#[derive(Debug)]
pub struct Basic {
    pub front: String,
    pub back: String,
}

impl Basic {
    const FIELDS_MAP: [[&str; 2]; 2] = [["front", "Front"], ["back", "Back"]];

    pub fn from_callout(callout: &Callout, header_lang: Option<&str>) -> Self {
        Basic {
            front: callout.header.clone(),
            back: callout.content_to_html(header_lang),
        }
    }
}

impl AddNote for Basic {
    fn add_note(
        &self,
        deck: &ankiconnect_rs::Deck,
        note: ankiconnect_rs::Note,
        allow_duplicate: bool,
        duplicate_scope: Option<ankiconnect_rs::DuplicateScope>,
    ) -> ankiconnect_rs::Result<ankiconnect_rs::NoteId> {
        todo!()
    }
}
