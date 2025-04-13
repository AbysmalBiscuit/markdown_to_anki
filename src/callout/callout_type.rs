use std::fmt::Display;

use crate::callout::callout_error::CalloutError;

#[derive(Debug)]
pub enum CalloutType {
    // Builtin callouts
    Abstract,
    Attention,
    Bug,
    Caution,
    Check,
    Cite,
    Danger,
    Done,
    ErrorCallout,
    Example,
    Fail,
    Failure,
    Faq,
    Help,
    Hint,
    Info,
    Links,
    Missing,
    Note,
    Question,
    Quote,
    Success,
    Summary,
    Tip,
    Tldr,
    Todo,
    Warning,
    // Custom callouts
    Word,
    Rule,
    Important,
    ExampleKR,
    ExampleSentenceKR,
    OverviewKR,
    Exception,
}

impl CalloutType {
    pub fn callout_default_header(&self) -> String {
        match self {
            // Builtin
            CalloutType::Abstract => "Abstract".into(),
            CalloutType::Attention => "Attention".into(),
            CalloutType::Bug => "Bug".into(),
            CalloutType::Caution => "Caution".into(),
            CalloutType::Check => "Check".into(),
            CalloutType::Cite => "Cite".into(),
            CalloutType::Danger => "Danger".into(),
            CalloutType::Done => "Done".into(),
            CalloutType::ErrorCallout => "Errorcallout".into(),
            CalloutType::Example => "Example".into(),
            CalloutType::Exception => "Exception".into(),
            CalloutType::Faq => "Faq".into(),
            CalloutType::Fail => "Fail".into(),
            CalloutType::Failure => "Failure".into(),
            CalloutType::Help => "Help".into(),
            CalloutType::Hint => "Hint".into(),
            CalloutType::Info => "Info".into(),
            CalloutType::Links => "Links".into(),
            CalloutType::Missing => "Missing".into(),
            CalloutType::Note => "Note".into(),
            CalloutType::Question => "Question".into(),
            CalloutType::Quote => "Quote".into(),
            CalloutType::Success => "Success".into(),
            CalloutType::Summary => "Summary".into(),
            CalloutType::Tldr => "Tldr".into(),
            CalloutType::Tip => "Tip".into(),
            CalloutType::Todo => "Todo".into(),
            CalloutType::Warning => "Warning".into(),
            // Custom
            CalloutType::Word => "Word".into(),
            CalloutType::Rule => "Rule".into(),
            CalloutType::ExampleKR => "예".into(),
            CalloutType::ExampleSentenceKR => "예문-문장".into(),
            CalloutType::OverviewKR => "개요".into(),
            CalloutType::Important => "important".into(),
        }
    }
}

impl TryFrom<&str> for CalloutType {
    type Error = CalloutError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            // Builtin
            "abstract" => Ok(CalloutType::Abstract),
            "attention" => Ok(CalloutType::Attention),
            "bug" => Ok(CalloutType::Bug),
            "caution" => Ok(CalloutType::Caution),
            "check" => Ok(CalloutType::Check),
            "cite" => Ok(CalloutType::Cite),
            "danger" => Ok(CalloutType::Danger),
            "done" => Ok(CalloutType::Done),
            "errorcallout" => Ok(CalloutType::ErrorCallout),
            "example" => Ok(CalloutType::Example),
            "exception" => Ok(CalloutType::Exception),
            "faq" => Ok(CalloutType::Faq),
            "fail" => Ok(CalloutType::Fail),
            "failure" => Ok(CalloutType::Failure),
            "help" => Ok(CalloutType::Help),
            "hint" => Ok(CalloutType::Hint),
            "info" => Ok(CalloutType::Info),
            "links" => Ok(CalloutType::Links),
            "missing" => Ok(CalloutType::Missing),
            "note" => Ok(CalloutType::Note),
            "question" => Ok(CalloutType::Question),
            "quote" => Ok(CalloutType::Quote),
            "success" => Ok(CalloutType::Success),
            "summary" => Ok(CalloutType::Summary),
            "tldr" => Ok(CalloutType::Tldr),
            "tip" => Ok(CalloutType::Tip),
            "todo" => Ok(CalloutType::Todo),
            "warning" => Ok(CalloutType::Warning),
            // Custom
            "word" => Ok(CalloutType::Word),
            "rule" => Ok(CalloutType::Rule),
            "예" => Ok(CalloutType::ExampleKR),
            "예문-문장" => Ok(CalloutType::ExampleSentenceKR),
            "개요" => Ok(CalloutType::OverviewKR),
            "important" => Ok(CalloutType::Important),
            _ => Err(CalloutError::UnknownType),
        }
    }
}
impl TryFrom<String> for CalloutType {
    type Error = CalloutError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        CalloutType::try_from(value.as_str())
    }
}

impl From<&CalloutType> for String {
    fn from(val: &CalloutType) -> Self {
        match val {
            // Builtin
            CalloutType::Abstract => "abstract".into(),
            CalloutType::Attention => "attention".into(),
            CalloutType::Bug => "bug".into(),
            CalloutType::Caution => "caution".into(),
            CalloutType::Check => "check".into(),
            CalloutType::Cite => "cite".into(),
            CalloutType::Danger => "danger".into(),
            CalloutType::Done => "done".into(),
            CalloutType::ErrorCallout => "errorcallout".into(),
            CalloutType::Example => "example".into(),
            CalloutType::Exception => "exception".into(),
            CalloutType::Faq => "faq".into(),
            CalloutType::Fail => "fail".into(),
            CalloutType::Failure => "failure".into(),
            CalloutType::Help => "help".into(),
            CalloutType::Hint => "hint".into(),
            CalloutType::Info => "info".into(),
            CalloutType::Links => "links".into(),
            CalloutType::Missing => "missing".into(),
            CalloutType::Note => "note".into(),
            CalloutType::Question => "question".into(),
            CalloutType::Quote => "quote".into(),
            CalloutType::Success => "success".into(),
            CalloutType::Summary => "summary".into(),
            CalloutType::Tldr => "tldr".into(),
            CalloutType::Tip => "tip".into(),
            CalloutType::Todo => "todo".into(),
            CalloutType::Warning => "warning".into(),
            // Custom
            CalloutType::Word => "word".into(),
            CalloutType::Rule => "rule".into(),
            CalloutType::ExampleKR => "예".into(),
            CalloutType::ExampleSentenceKR => "예문-문장".into(),
            CalloutType::OverviewKR => "개요".into(),
            CalloutType::Important => "important".into(),
        }
    }
}

impl Display for CalloutType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from(self))
    }
}
