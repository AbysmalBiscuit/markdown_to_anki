#[derive(Debug)]
pub enum NoteOperation {
    Add,
    Update,
    Move,
    Delete,
    Nop,
}

impl Default for NoteOperation {
    fn default() -> Self {
        NoteOperation::Nop
    }
}
