// From Anki repo

use serde::{Deserialize, Serialize};

use crate::new_id_type;

#[derive(Debug, Serialize, Deserialize)]
pub struct Card {}

new_id_type!(CardId, i64);
