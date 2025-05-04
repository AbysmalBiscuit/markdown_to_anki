use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Response<R> {
    pub result: R,
    pub error: Option<String>,
}
