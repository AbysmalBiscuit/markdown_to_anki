use std::collections::HashMap;

use params::CreateModel;

use crate::model::ModelType;

use super::{
    AnkiConnectClient, client::ClientBehavior, error::APIError, model::Model, response::Response,
};

#[derive(Debug, Clone)]
pub struct ModelsClient<'a>(pub &'a AnkiConnectClient);

impl ModelsClient<'_> {
    pub fn update_model_styling(
        &self,
        model_name: &str,
        css: &str,
    ) -> Result<Response<Option<()>>, APIError> {
        self.0.request(
            "updateModelStyling",
            Some(params::UpdateModelStyling::new(
                params::UpdateModelStylingModel::new(model_name, css),
            )),
        )
    }

    pub fn get_all_names(&self) -> Result<Vec<String>, APIError> {
        let response: Response<Vec<String>> = self.0.request("modelNames", None::<()>)?;
        Ok(response.result.unwrap())
    }

    pub fn find_by_name(&self, model_names: Vec<&str>) -> Result<Vec<Model>, APIError> {
        let models = self.0.request::<Vec<Model>, _>(
            "findModelsByName",
            Some(params::FindModelsByNameParams::new(model_names)),
        )?;
        Ok(models.result.unwrap())
    }

    pub fn create_model(&self, model: params::CreateModel) -> Result<Model, APIError> {
        self.0
            .request("createModel", Some(model))
            .map(|result| result.result.unwrap())
    }
}

pub mod params {
    use std::{borrow::Cow, collections::HashMap};

    use derive_new::new;
    use serde::Serialize;

    #[derive(Debug, Serialize, new)]
    pub struct UpdateModelStyling<'a> {
        model: UpdateModelStylingModel<'a>,
    }

    #[derive(Debug, Serialize, new)]
    pub struct UpdateModelStylingModel<'a> {
        name: &'a str,
        css: &'a str,
    }

    #[derive(Debug, Serialize, new)]
    #[serde(rename_all = "camelCase")]
    pub struct FindModelsByNameParams<'a> {
        model_names: Vec<&'a str>,
    }

    #[derive(Debug, Serialize, new)]
    #[serde(rename_all = "camelCase")]
    pub struct CreateModel<'a> {
        model_name: &'a str,
        in_order_fields: Vec<&'a str>,
        css: Option<&'a str>,
        is_cloze: Option<bool>,
        card_templates: Vec<HashMap<Cow<'a, str>, Cow<'a, str>>>,
    }
}
