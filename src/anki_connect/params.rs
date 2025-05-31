use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Params<'a, P: Serialize> {
    action: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    params: Option<P>,
    version: u8,
}

impl<'a, P: Serialize> Params<'a, P> {
    pub fn new(action: &'a str, params: Option<P>) -> Self {
        Params {
            action: action.into(),
            params,
            version: 6,
        }
    }
}
