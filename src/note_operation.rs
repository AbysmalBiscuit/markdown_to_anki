#[derive(Debug)]
pub enum NoteOperation {
    Add,
    Update,
    Move,
    MoveUpdate,
    Delete,
    Nop,
}

impl Default for NoteOperation {
    fn default() -> Self {
        NoteOperation::Nop
    }
}
