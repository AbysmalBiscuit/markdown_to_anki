use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Params<'a, P>
where
    P: Serialize,
{
    action: &'a str,
    #[allow(clippy::struct_field_names)]
    #[serde(skip_serializing_if = "Option::is_none")]
    params: Option<P>,
    version: u8,
}

impl<'a, P: Serialize> Params<'a, P> {
    pub const fn new(action: &'a str, params: Option<P>) -> Self {
        Self {
            action,
            params,
            version: 6,
        }
    }
}
