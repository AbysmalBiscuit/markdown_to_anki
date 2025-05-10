use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use rayon::prelude::*;
use serde::Serialize;

use crate::anki_connect_client::model::Field;
use crate::anki_connect_client::model::Model;

/// Represents a field within a model
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct InternalField {
    name: String,
    ord: usize, // Field ordinal/position in the model
}

impl From<&Field> for InternalField {
    fn from(value: &Field) -> Self {
        InternalField {
            name: value.name().to_string(),
            ord: value.ord(),
        }
    }
}

impl Into<Field> for &InternalField {
    fn into(self) -> Field {
        Field::new(self.name.clone(), self.ord)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct InternalModel {
    id: u64,
    name: String,
    fields: Vec<InternalField>,
}

impl From<Model> for InternalModel {
    fn from(value: Model) -> Self {
        InternalModel {
            id: value.id,
            name: value.name().to_string(),
            fields: value
                .fields()
                .into_par_iter()
                .map(|field| field.into())
                .collect(),
        }
    }
}

impl From<InternalModel> for Model {
    fn from(value: InternalModel) -> Self {
        Model::new(
            value.id,
            value.name,
            value.fields.par_iter().map(|field| field.into()).collect(),
        )
        .unwrap()
    }
}
