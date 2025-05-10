use serde::Deserialize as DeTrait;
use serde::Deserializer;
use serde_json::Value;

use crate::new_id_type;

// From https://github.com/ankitects/anki/rslib/src/serde.rs
pub fn is_default<T: Default + PartialEq>(value: &T) -> bool {
    value == &T::default()
}

pub fn default_on_invalid<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Default + DeTrait<'de>,
    D: Deserializer<'de>,
{
    let v: Value = DeTrait::deserialize(deserializer)?;
    Ok(T::deserialize(v).unwrap_or_default())
}

new_id_type!(Usn, i32);
