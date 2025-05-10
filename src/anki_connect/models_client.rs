use std::{collections::HashMap, sync::Arc};

use rayon::prelude::*;

use super::{client::AnkiConnectClient, error::APIError, model::Model, response::Response};

#[derive(Debug, Clone)]
pub struct ModelsClient<'a>(pub &'a AnkiConnectClient);

impl<'a> ModelsClient<'a> {
    pub fn update_model_styling(
        &self,
        model_name: &str,
        css: &str,
    ) -> Result<Response<Option<()>>, APIError> {
        self.0.http_client.request(
            "updateModelStyling",
            Some(params::UpdateModelStyling::new(
                params::UpdateModelStylingModel::new(model_name, css),
            )),
        )
    }

    pub fn get_all_names(&self) -> Result<Vec<String>, APIError> {
        let response: Response<Vec<String>> =
            self.0.http_client.request("modelNames", None::<()>)?;
        Ok(response.result.unwrap())
    }

    pub fn find_by_name(&self, model_names: Vec<&str>) -> Result<Vec<Model>, APIError> {
        let models = self.0.http_client.request::<Vec<Model>, _>(
            "findModelsByName",
            Some(params::FindModelsByNameParams::new(model_names)),
        )?;
        Ok(models.result.unwrap())
    }

    pub fn create_model(
        &self,
        model_name: &str,
        in_order_fields: Vec<&str>,
        css: Option<&str>,
        is_cloze: bool,
        card_templates: Vec<HashMap<String, String>>,
    ) -> Result<Model, APIError> {
        todo!()
    }
}

pub mod params {
    use std::collections::HashMap;

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
        css: &'a str,
        is_cloze: bool,
        card_templates: Vec<HashMap<&'a str, &'a str>>,
    }
}
