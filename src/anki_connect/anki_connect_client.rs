use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::time::Duration;
use strum::{Display, EnumString};

use super::{
    cards_client::CardsClient, client::ClientBehavior, decks_client::DecksClient, error::APIError,
    models_client::ModelsClient, notes_client::NotesClient, params::Params, response::Response,
};
use ureq::Agent;

#[cfg(feature = "reqwest_blocking")]
use super::client::ReqwestClient;

#[cfg(feature = "ureq_blocking")]
use super::client::UreqClient;

#[derive(Debug, Clone, Display, EnumString)]
#[enum_dispatch(ClientBehavior)]
pub enum AnkiConnectClient {
    #[cfg(feature = "reqwest_blocking")]
    ReqwestClient(ReqwestClient),
    #[cfg(feature = "ureq_blocking")]
    UreqClient(UreqClient),
}

// #[derive(Debug, Clone)]
// pub struct AnkiConnectClient {
//     agent: Agent,
//     url: String,
// }

impl AnkiConnectClient {
    pub fn new(url: Option<&str>, port: Option<u32>) -> Self {
        #[cfg(feature = "reqwest_blocking")]
        {
            Self::ReqwestClient(ReqwestClient::new(url, port))
        }
        #[cfg(feature = "ureq_blocking")]
        {
            Self::UreqClient(UreqClient::new(url, port))
        }

        // fallback if neither feature is enabled
        // #[cfg(not(any(feature = "ureq_blocking", feature = "reqwest_blocking")))]
        // compile_error!("Enable at least one of `ureq_blocking` or `reqwest_blocking` features.");
    }

    pub fn cards(&self) -> CardsClient<'_> {
        CardsClient(self)
    }

    pub fn decks(&self) -> DecksClient<'_> {
        DecksClient(self)
    }

    pub fn models(&self) -> ModelsClient<'_> {
        ModelsClient(self)
    }

    pub fn notes(&self) -> NotesClient<'_> {
        NotesClient(self)
    }

    pub fn test_connection(&self) -> Result<bool, APIError> {
        match self.request_with_timeout::<TestConnectionParams, TestConnectionParams>(
            "apiReflect",
            Some(TestConnectionParams {
                scopes: vec!["actions".into()],
                actions: vec!["apiReflect".into()],
            }),
            Some(1),
        ) {
            Ok(_) => Ok(true),
            Err(err) => match err {
                APIError::UreqError(_) => Ok(false),
                _ => {
                    dbg!(&err);
                    Err(APIError::FailedConnection(err.to_string()))
                }
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct TestConnectionParams {
    scopes: Vec<String>,
    actions: Vec<String>,
}
