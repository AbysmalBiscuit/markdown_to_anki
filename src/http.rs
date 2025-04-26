//! Taken from: https://github.com/btrkeks/ankiconnect-rs/blob/master/src/http.rs
//! HTTP client implementation for communicating with AnkiConnect

use ankiconnect_rs::error::{AnkiConnectError, AnkiError};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;

/// Trait for sending requests to AnkiConnect
///
/// This abstraction allows for dependency injection and easier testing.
pub trait RequestSender: Send + Sync {
    /// Sends a request to AnkiConnect
    ///
    /// # Arguments
    ///
    /// * `action` - The action to perform
    /// * `params` - The parameters for the action
    ///
    /// # Returns
    ///
    /// The response from AnkiConnect
    fn send<P, R>(&self, action: &str, params: Option<P>) -> Result<R, AnkiError>
    where
        P: Serialize + Debug,
        R: DeserializeOwned + 'static;
}

/// HTTP implementation of the RequestSender trait
pub struct HttpRequestSender {
    url: String,
    api_version: u8,
}

impl HttpRequestSender {
    /// Creates a new HttpRequestSender with the given host and port
    pub fn new(host: &str, port: u16) -> Self {
        Self {
            url: format!("http://{}:{}", host, port),
            api_version: 6, // AnkiConnect API version
        }
    }
}

#[derive(Serialize)]
struct AnkiConnectRequest<T> {
    action: String,
    version: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    params: Option<T>,
}

#[derive(Deserialize)]
struct AnkiConnectResponse<T> {
    result: Option<T>,
    error: Option<String>,
}

impl RequestSender for HttpRequestSender {
    fn send<P, R>(&self, action: &str, params: Option<P>) -> Result<R, AnkiError>
    where
        P: Serialize + Debug,
        R: DeserializeOwned + 'static,
    {
        let request = AnkiConnectRequest {
            action: action.to_string(),
            version: self.api_version,
            params,
        };

        // Send the request to AnkiConnect
        let mut response = ureq::post(&self.url)
            .send_json(&request)
            .map_err(AnkiError::HttpError)?;

        // Parse the response
        let anki_response: AnkiConnectResponse<R> = response
            .body_mut()
            .read_json()
            .map_err(|e| AnkiError::JsonError(e.to_string()))?;

        // Handle the response
        if let Some(error) = anki_response.error {
            Err(AnkiError::AnkiConnectError(parse_anki_connect_error(
                &error,
            )))
        } else if let Some(result) = anki_response.result {
            Ok(result)
        } else {
            handle_empty_response::<R>()
        }
    }
}

// Helper function to handle empty responses based on type
fn handle_empty_response<R: 'static>() -> Result<R, AnkiError> {
    // Check if R is the unit type () using std::any::TypeId
    if std::any::TypeId::of::<R>() == std::any::TypeId::of::<()>() {
        // This is safe because we've verified that R is ()
        // We need this because Rust doesn't allow us to directly return Ok(())
        // when the function expects to return Result<R, _>
        Ok(unsafe { std::mem::zeroed() })
    } else {
        Err(AnkiError::AnkiConnectError(AnkiConnectError::Other(
            "Empty response from AnkiConnect (both result and error are null)".to_string(),
        )))
    }
}

/// Parse an error message from AnkiConnect into a structured error
fn parse_anki_connect_error(error: &str) -> AnkiConnectError {
    if error.starts_with("deck was not found: ") {
        let deck_name = error.trim_start_matches("deck was not found: ").trim();
        AnkiConnectError::DeckNotFound(deck_name.to_string())
    } else if error.starts_with("model was not found: ") {
        let model_name = error.trim_start_matches("model was not found: ").trim();
        AnkiConnectError::ModelNotFound(model_name.to_string())
    } else if error == "cannot create note because it is a duplicate" {
        AnkiConnectError::DuplicateNote
    } else if error == "cannot create note because it is empty" {
        AnkiConnectError::EmptyNote
    } else if error.starts_with("invalid columnId: ") {
        let column_id = error.trim_start_matches("invalid columnId: ").trim();
        AnkiConnectError::InvalidColumnId(column_id.to_string())
    } else if error.starts_with("invalid card order: ") {
        let order = error.trim_start_matches("invalid card order: ").trim();
        AnkiConnectError::InvalidCardOrder(order.to_string())
    } else if error == "You must provide a \"data\", \"path\", or \"url\" field." {
        AnkiConnectError::MissingMediaField
    } else if error == "Model name already exists" {
        AnkiConnectError::ModelNameExists
    } else if error
        == "The field values you have provided would make an empty question on all cards."
    {
        AnkiConnectError::EmptyQuestion
    } else if error == "unsupported action" {
        AnkiConnectError::UnsupportedAction
    } else {
        AnkiConnectError::Other(error.to_string())
    }
}

///! taken from: https://github.com/btrkeks/ankiconnect-rs/blob/master/src/client/request.rs#L283
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CreateModelParams<'a> {
    pub model_name: &'a str,
    pub in_order_fields: &'a [&'a str],
    pub css: &'a str,
    pub card_templates: Vec<HashMap<String, String>>,
}
