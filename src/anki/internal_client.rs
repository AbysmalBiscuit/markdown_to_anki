use std::{collections::HashMap, error::Error};

use strum::Display;

use crate::http::CreateModelParams;

#[derive(Debug, Display)]
pub enum APIError {}
impl Error for APIError {}

#[derive(Debug)]
struct CustomClient {
    url: String,
}

impl CustomClient {
    fn create_model(action: String, params: Option<CreateModelParams>) -> Result<(), APIError> {
        todo!()
    }
}
