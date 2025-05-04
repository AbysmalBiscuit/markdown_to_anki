use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Params<P: Serialize> {
    action: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    params: Option<P>,
    version: u8,
}

impl<P: Serialize> Params<P> {
    pub fn new(action: &str, params: Option<P>) -> Self {
        Params {
            action: action.into(),
            params,
            version: 6,
        }
    }
}
