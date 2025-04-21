use ankiconnect_rs::error::Result;
use ankiconnect_rs::{Deck, DuplicateScope, Note, NoteId};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::callout::{Callout, callout_type::CalloutType, content::CalloutContent};

pub trait AddNote {
    fn add_note(
        &self,
        deck: &Deck,
        note: Note,
        allow_duplicate: bool,
        duplicate_scope: Option<DuplicateScope>,
    ) -> Result<NoteId>;
}
