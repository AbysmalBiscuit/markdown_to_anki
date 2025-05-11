// From Anki
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_aux::field_attributes::deserialize_number_from_string;
use serde_json::Value;
use serde_tuple::Serialize_tuple;

use super::util::{Usn, default_on_invalid, is_default};
use crate::new_id_type;

#[derive(Debug, Serialize, Deserialize)]
pub struct Card {}

new_id_type!(CardId, i64);
