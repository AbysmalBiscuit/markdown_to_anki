use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use rayon::prelude::*;
use serde::Serialize;

/// Represents a field within a model
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct InternalField {
    name: String,
    ord: usize, // Field ordinal/position in the model
}

impl From<&ankiconnect_rs::Field> for InternalField {
    fn from(value: &ankiconnect_rs::Field) -> Self {
        InternalField {
            name: value.name().to_string(),
            ord: value.ord(),
        }
    }
}

impl Into<ankiconnect_rs::Field> for &InternalField {
    fn into(self) -> ankiconnect_rs::Field {
        ankiconnect_rs::Field::new(self.name.clone(), self.ord)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct InternalModel {
    id: u64,
    name: String,
    fields: Vec<InternalField>,
}

impl From<ankiconnect_rs::Model> for InternalModel {
    fn from(value: ankiconnect_rs::Model) -> Self {
        InternalModel {
            id: value.id().0,
            name: value.name().to_string(),
            fields: value
                .fields()
                .into_par_iter()
                .map(|field| field.into())
                .collect(),
        }
    }
}

impl From<InternalModel> for ankiconnect_rs::Model {
    fn from(value: InternalModel) -> Self {
        ankiconnect_rs::Model::new(
            value.id,
            value.name,
            value.fields.par_iter().map(|field| field.into()).collect(),
        )
        .unwrap()
    }
}
