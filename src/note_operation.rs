use serde::Serialize;

#[derive(Debug, Default, Clone, Copy, Serialize)]
pub enum NoteOperation {
    Add,
    Update,
    Move,
    // Delete,
    #[default]
    Nop,
}
