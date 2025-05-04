use std::collections::{HashMap, HashSet};

use serde::Serialize;

use crate::client::note::Note;

use super::internal_model::InternalModel;

#[derive(Debug, Clone, Serialize)]
pub struct InternalNote {
    id: Option<u64>,
    model: InternalModel,
    field_values: HashMap<String, String>,
    tags: HashSet<String>,
    // media: Vec<FieldMedia>,
}

impl InternalNote {
    pub fn new(
        model: InternalModel,
        field_values: HashMap<String, String>,
        tags: HashSet<String>,
    ) -> Self {
        InternalNote {
            id: None,
            model,
            field_values,
            tags,
        }
    }
}

impl From<Note> for InternalNote {
    fn from(value: Note) -> Self {
        InternalNote {
            id: value.id().map(|id| id.0),
            model: value.model().to_owned().into(),
            field_values: value.field_values().to_owned(),
            tags: value.tags().to_owned(),
        }
    }
}

impl From<InternalNote> for Note {
    fn from(value: InternalNote) -> Self {
        Note::new(
            value.model.into(),
            value.field_values,
            value.tags,
            Vec::new(),
        )
        .unwrap()
    }
}
