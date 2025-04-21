use ankiconnect_rs::error::Result;
use ankiconnect_rs::{Deck, DuplicateScope, Note, NoteId};

pub trait AddNote {
    fn add_note(
        &self,
        deck_name: &ankiconnect_rs::Deck,
        model: &ankiconnect_rs::Model,
        note: ankiconnect_rs::Note,
        allow_duplicate: bool,
        duplicate_scope: Option<ankiconnect_rs::DuplicateScope>,
    ) -> Result<NoteId>;
}
