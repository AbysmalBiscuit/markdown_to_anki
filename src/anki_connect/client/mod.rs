use super::AnkiConnectClient;
use super::{error::APIError, response::Response};
use enum_dispatch::enum_dispatch;
use serde::{Serialize, de::DeserializeOwned};

#[enum_dispatch]
pub trait ClientBehavior {
    fn request_with_timeout<R, P>(
        &self,
        action: &str,
        params: Option<P>,
        timeout: Option<u8>,
    ) -> Result<Response<R>, APIError>
    where
        R: DeserializeOwned + std::fmt::Debug,
        P: Serialize + std::fmt::Debug;
    fn request<R, P>(&self, action: &str, params: Option<P>) -> Result<Response<R>, APIError>
    where
        R: DeserializeOwned + std::fmt::Debug,
        P: Serialize + std::fmt::Debug;
}

#[cfg(feature = "reqwest_blocking")]
mod reqwest_client;
#[cfg(feature = "ureq_blocking")]
mod ureq_client;

#[cfg(feature = "reqwest_blocking")]
pub use reqwest_client::ReqwestClient;
#[cfg(feature = "ureq_blocking")]
pub use ureq_client::UreqClient;
