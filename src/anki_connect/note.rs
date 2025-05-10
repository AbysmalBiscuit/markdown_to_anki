use serde::{Deserialize, Serialize};

use crate::new_id_type;

use super::{model::ModelId, util::Usn};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Note {
    pub id: NoteId,
    pub guid: String,
    pub notetype_id: ModelId,
    pub mtime: i64,
    pub usn: Usn,
    pub tags: Vec<String>,
    fields: Vec<String>,
    pub(crate) sort_field: Option<String>,
    pub(crate) checksum: Option<u32>,
}

new_id_type!(NoteId, i64);
