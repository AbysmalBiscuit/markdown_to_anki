use std::{collections::HashMap, sync::Arc};

use rayon::prelude::*;

use super::{error::APIError, http_client::HttpClient, model::Model, response::Response};

pub mod create_model_params {
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Params {
        model_name: String,
        in_order_fields: Vec<String>,
        css: String,
        is_cloze: bool,
        card_templates: Vec<HashMap<String, String>>,
    }
}

pub mod params {
    use derive_new::new;
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    #[derive(Debug, Serialize, Deserialize, new)]
    #[serde(rename_all = "camelCase")]
    pub struct FindModelsByNameParams {
        // #[new(into_iter = "String")]
        model_names: Vec<String>,
    }
}

#[derive(Debug, Clone)]
pub struct ModelClient {
    http_client: Arc<HttpClient>,
}

impl ModelClient {
    pub fn new(client: Arc<HttpClient>) -> Self {
        ModelClient {
            http_client: client,
        }
    }

    pub fn update_model_styling(
        &self,
        model_name: &str,
        css: &str,
    ) -> Result<Response<Option<()>>, APIError> {
        self.http_client.request(
            "updateModelStyling",
            Some(update_model_styling_params::UpdateModelStylingParams::new(
                model_name, css,
            )),
        )
    }

    pub fn get_all_names(&self) -> Result<Vec<String>, APIError> {
        let response: Response<Vec<String>> = self.http_client.request("modelNames", None::<()>)?;
        Ok(response.result.unwrap())
    }

    pub fn find_by_name(&self, model_names: Vec<&str>) -> Result<Vec<Model>, APIError> {
        let models = self.http_client.request::<Vec<Model>, _>(
            "findModelsByName",
            Some(params::FindModelsByNameParams::new(
                model_names
                    .into_par_iter()
                    .map(|name| name.to_string())
                    .collect(),
            )),
        )?;
        // models
        //     .into_par_iter()
        //     .find_first(|name| name.eq(model_names))
        //     .ok_or_else(|| APIError::ModelNotFound(model_names.into()))
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

pub mod update_model_styling_params {
    use serde::Serialize;

    #[derive(Debug, Serialize)]
    pub struct UpdateModelStylingParams {
        model: InnerParams,
    }

    #[derive(Debug, Serialize)]
    struct InnerParams {
        name: String,
        css: String,
    }

    impl UpdateModelStylingParams {
        pub fn new(name: &str, css: &str) -> Self {
            UpdateModelStylingParams {
                model: InnerParams {
                    name: name.into(),
                    css: css.into(),
                },
            }
        }
    }
}
