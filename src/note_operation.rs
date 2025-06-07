use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize)]
pub enum NoteOperation {
    Add,
    Update,
    Move,
    // Delete,
    Nop,
}

impl Default for NoteOperation {
    fn default() -> Self {
        NoteOperation::Nop
    }
}
