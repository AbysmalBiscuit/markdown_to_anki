use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_aux::field_attributes::deserialize_number_from_string;
use serde_json::Value;
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_tuple::Serialize_tuple;

use super::util::{Usn, default_on_invalid, is_default};
use crate::new_id_type;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub id: ModelId,
    pub name: String,
    #[serde(rename = "type")]
    kind: NotetypeKind,
    #[serde(rename = "mod")]
    mtime: usize,
    usn: Usn,
    sortf: u16,
    #[serde(deserialize_with = "default_on_invalid")]
    did: Option<usize>,
    tmpls: Vec<Template>,
    flds: Vec<Field>,
    #[serde(deserialize_with = "default_on_invalid")]
    css: String,
    #[serde(default)]
    latex_pre: String,
    #[serde(default)]
    latex_post: String,
    #[serde(default, deserialize_with = "default_on_invalid")]
    latex_svg: String,
    #[serde(default, deserialize_with = "default_on_invalid")]
    req: CardRequirementsSchema11,
    #[serde(default, skip_serializing_if = "is_default")]
    original_stock_kind: i32,
    #[serde(default, skip_serializing_if = "is_default")]
    original_id: Option<i64>,
    #[serde(flatten)]
    other: HashMap<String, Value>,
}

new_id_type!(ModelId, i64);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Field {
    name: String,
    ord: usize,
    sticky: bool,
    rtl: bool,
    font: String,
    size: usize,
    media: Option<Vec<Media>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Media {
    source: String,
    filename: String,
    media_type: MediaType,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum MediaType {
    Audio,
    Image,
    Video,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Template {
    name: String,
    ord: usize,
    qfmt: String,
    afmt: String,
    did: Option<String>,
    bqfmt: String,
    bafmt: String,
}

// From https://github.com/ankitects/anki/rslib/src/notetype/schema11.rs
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Eq, Debug, Clone, Copy)]
#[repr(u8)]
pub enum NotetypeKind {
    Standard = 0,
    Cloze = 1,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub(crate) struct CardRequirementsSchema11(pub(crate) Vec<CardRequirementSchema11>);

#[derive(Serialize_tuple, Deserialize, Debug, Clone)]
pub(crate) struct CardRequirementSchema11 {
    pub(crate) card_ord: u16,
    pub(crate) kind: FieldRequirementKindSchema11,
    pub(crate) field_ords: Vec<u16>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum FieldRequirementKindSchema11 {
    Any,
    All,
    None,
}
