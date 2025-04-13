use strum_macros::Display;

#[derive(Display, Debug)]
pub enum CalloutError {
    EmptyString,
    UnknownType,
    FailedToParseHeader,
}
