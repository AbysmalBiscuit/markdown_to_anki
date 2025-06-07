use enum_dispatch::enum_dispatch;
use serde::{Serialize, de::DeserializeOwned};
use std::fmt::Debug;
use strum::{Display, EnumString};

use super::{
    cards_client::CardsClient, client::ClientBehavior, decks_client::DecksClient, error::APIError,
    models_client::ModelsClient, notes_client::NotesClient,
};

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
        match self
            .request_with_timeout::<params::TestConnectionParams, params::TestConnectionParams>(
                "apiReflect",
                Some(params::TestConnectionParams::new(
                    vec!["actions".into()],
                    vec!["apiReflect".into()],
                )),
                Some(1),
            ) {
            Ok(_) => Ok(true),
            Err(err) => match err {
                APIError::UreqError(_) => Ok(false),
                _ => Err(APIError::FailedConnection(err.to_string())),
            },
        }
    }

    pub fn multi<P, R>(&self, actions: Vec<&params::Action<P>>) -> Result<Vec<R>, APIError>
    where
        P: Serialize + std::fmt::Debug,
        R: DeserializeOwned + std::fmt::Debug,
    {
        self.request::<Vec<R>, _>("multi", Some(params::Multi::new(actions)))
            .map(|response| response.result.unwrap())
    }
}

pub mod response {

    use derive_new::new;
    use serde::{Deserialize, Serialize};
    use std::fmt::Debug;

    #[derive(Debug, Serialize, Deserialize, new)]
    #[serde(rename_all = "camelCase")]
    pub struct BasicResponse {
        pub result: Option<String>,
        pub error: Option<String>,
    }
}

pub mod params {
    use derive_new::new;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, new)]
    #[serde(rename_all = "camelCase")]
    pub struct TestConnectionParams {
        scopes: Vec<String>,
        actions: Vec<String>,
    }

    #[derive(Debug, Serialize, new)]
    #[serde(rename_all = "camelCase")]
    pub struct Multi<'a, P: Serialize + std::fmt::Debug> {
        actions: Vec<&'a Action<'a, P>>,
    }

    #[derive(Debug, Serialize, new)]
    #[serde(rename_all = "camelCase")]
    pub struct Action<'a, P: Serialize + std::fmt::Debug> {
        action: &'a str,
        version: u8,
        params: &'a P,
    }
}
