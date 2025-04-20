use strum::Display;

#[derive(Display, Debug)]
pub enum CalloutError {
    EmptyString,
    UnknownType,
    FailedToParseHeader,
    UnknownContentType,
}
