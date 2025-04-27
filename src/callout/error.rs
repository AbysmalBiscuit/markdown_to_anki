use strum::{Display, EnumMessage};

#[derive(Display, Debug, EnumMessage)]
pub enum CalloutError {
    EmptyString,
    UnknownType,
    #[strum(
        message = "first line should be formatted as a callout '> [!TYPE] TEXT TRANSLITERATION EMOJI'"
    )]
    FailedToParseHeader,
    Io(std::io::Error),
    NoMarkdownID,
}
