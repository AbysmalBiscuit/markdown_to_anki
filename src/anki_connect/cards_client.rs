use super::{AnkiConnectClient, error::APIError, response::Response};

#[derive(Debug, Clone)]
pub struct CardsClient<'a>(pub &'a AnkiConnectClient);

impl CardsClient<'_> {}

pub mod params {
    use derive_new::new;
    use serde::Serialize;

    #[derive(Debug, Serialize, new)]
    #[serde(rename_all = "camelCase")]
    pub struct ActionPrams {
        var: String,
    }
}
