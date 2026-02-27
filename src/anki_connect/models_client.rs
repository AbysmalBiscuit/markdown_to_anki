use super::{
    AnkiConnectClient, client::ClientBehavior, error::APIError, model::Model, response::Response,
};

/// Performs Model actions.
#[derive(Debug, Clone)]
pub struct ModelsClient<'a>(pub &'a AnkiConnectClient);

impl ModelsClient<'_> {
    /// Modify the CSS styling of an existing model by name.
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

    /// Gets the complete list of model names for the current user.
    pub fn get_all_names(&self) -> Result<Vec<String>, APIError> {
        self.0.request("modelNames", None::<()>)
    }

    /// Gets a list of models for the provided model names from the current user.
    pub fn find_by_name(&self, model_names: Vec<&str>) -> Result<Vec<Model>, APIError> {
        self.0.request::<Vec<Model>, _>(
            "findModelsByName",
            Some(params::FindModelsByNameParams::new(model_names)),
        )
    }

    /// Creates a new model to be used in Anki.
    /// User must provide the modelName, inOrderFields and cardTemplates to be used in the model.
    /// There are optional fields css and isCloze.
    /// If not specified, css will use the default Anki css and isCloze will be equal to false.
    /// If isCloze is true then model will be created as Cloze.
    pub fn create_model(&self, model: params::CreateModel) -> Result<Model, APIError> {
        self.0.request("createModel", Some(model))
    }
}

/// This module declares the request parameters for [`ModelsClient`] actions.
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
