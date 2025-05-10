// From Anki
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_aux::field_attributes::deserialize_number_from_string;
use serde_json::Value;
use serde_tuple::Serialize_tuple;

use super::util::{default_on_invalid, is_default};
use crate::new_id_type;

#[derive(Debug, Serialize, Deserialize)]
pub struct Deck {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub(crate) id: DeckId,
    #[serde(
        rename = "mod",
        deserialize_with = "deserialize_number_from_string",
        default
    )]
    pub(crate) mtime: i64,
    pub(crate) name: String,
    pub(crate) usn: i32,
    #[serde(flatten)]
    pub(crate) today: DeckTodaySchema11,
    #[serde(rename = "collapsed")]
    study_collapsed: bool,
    #[serde(default, rename = "browserCollapsed")]
    browser_collapsed: bool,
    #[serde(default)]
    desc: String,
    #[serde(default, rename = "md", skip_serializing_if = "is_false")]
    markdown_description: bool,
    #[serde(rename = "dyn")]
    dynamic: u8,
    #[serde(flatten)]
    other: HashMap<String, Value>,
}

fn is_false(b: &bool) -> bool {
    !b
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default, Clone)]
pub struct DeckTodaySchema11 {
    #[serde(rename = "lrnToday")]
    pub(crate) lrn: TodayAmountSchema11,
    #[serde(rename = "revToday")]
    pub(crate) rev: TodayAmountSchema11,
    #[serde(rename = "newToday")]
    pub(crate) new: TodayAmountSchema11,
    #[serde(rename = "timeToday")]
    pub(crate) time: TodayAmountSchema11,
}

#[derive(Serialize_tuple, Deserialize, Debug, PartialEq, Eq, Default, Clone)]
#[serde(from = "Vec<Value>")]
pub struct TodayAmountSchema11 {
    day: i32,
    amount: i32,
}

impl From<Vec<Value>> for TodayAmountSchema11 {
    fn from(mut v: Vec<Value>) -> Self {
        let amt = v.pop().and_then(|v| v.as_i64()).unwrap_or(0);
        let day = v.pop().and_then(|v| v.as_i64()).unwrap_or(0);
        TodayAmountSchema11 {
            amount: amt as i32,
            day: day as i32,
        }
    }
}

new_id_type!(DeckId, i64);
