use std::sync::LazyLock;

use rayon::prelude::*;
use regex::Regex;

// use hyperscan::prelude::*;

#[derive(Debug)]
pub enum CalloutError {
    EmptyString,
    UnknownType,
    FailedToParseHeader,
}

#[derive(Debug)]
pub enum CalloutType {
    Word,
    Rule,
    ExampleKR,
    ExampleSentenceKR,
    OverviewKR,
    Example,
    Tip,
    Important,
    Exception,
    Warning,
}

impl CalloutType {
    fn as_string(&self) -> String {
        match *self {
            CalloutType::Word => "word".into(),
            CalloutType::Rule => "rule".into(),
            CalloutType::ExampleKR => "예".into(),
            CalloutType::ExampleSentenceKR => "예문-문장".into(),
            CalloutType::OverviewKR => "개요".into(),
            CalloutType::Example => "example".into(),
            CalloutType::Tip => "tip".into(),
            CalloutType::Important => "important".into(),
            CalloutType::Exception => "exception".into(),
            CalloutType::Warning => "warning".into(),
        }
    }

    fn from_str(value: &str) -> Result<Self, CalloutError> {
        match value {
            "word" => Ok(CalloutType::Word),
            "rule" => Ok(CalloutType::Rule),
            "예" => Ok(CalloutType::ExampleKR),
            "예문-문장" => Ok(CalloutType::ExampleSentenceKR),
            "개요" => Ok(CalloutType::OverviewKR),
            "example" => Ok(CalloutType::Example),
            "tip" => Ok(CalloutType::Tip),
            "important" => Ok(CalloutType::Important),
            "exception" => Ok(CalloutType::Exception),
            "warning" => Ok(CalloutType::Warning),
            _ => Err(CalloutError::UnknownType),
        }
    }
}
impl TryFrom<&str> for CalloutType {
    type Error = CalloutError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "word" => Ok(CalloutType::Word),
            "rule" => Ok(CalloutType::Rule),
            "예" => Ok(CalloutType::ExampleKR),
            "예문-문장" => Ok(CalloutType::ExampleSentenceKR),
            "개요" => Ok(CalloutType::OverviewKR),
            "example" => Ok(CalloutType::Example),
            "tip" => Ok(CalloutType::Tip),
            "important" => Ok(CalloutType::Important),
            "exception" => Ok(CalloutType::Exception),
            "warning" => Ok(CalloutType::Warning),
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

impl Into<String> for CalloutType {
    fn into(self) -> String {
        match self {
            CalloutType::Word => "word".into(),
            CalloutType::Rule => "rule".into(),
            CalloutType::ExampleKR => "예".into(),
            CalloutType::ExampleSentenceKR => "예문-문장".into(),
            CalloutType::OverviewKR => "개요".into(),
            CalloutType::Example => "example".into(),
            CalloutType::Tip => "tip".into(),
            CalloutType::Important => "important".into(),
            CalloutType::Exception => "exception".into(),
            CalloutType::Warning => "warning".into(),
        }
    }
}

#[derive(Debug)]
pub struct Callout {
    callout_type: CalloutType,
    header: String,
    content: Vec<String>,
    sub_callouts: Vec<Callout>,
}

impl Default for Callout {
    fn default() -> Self {
        Callout {
            callout_type: CalloutType::Word,
            header: "Default".into(),
            content: Vec::new(),
            sub_callouts: Vec::new(),
        }
    }
}

static RE_HEADER: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"^> \[!(.+?)\][+-] ([\u4E00-\u9FFF\u3400-\u4DBF\uF900-\uFAFF\u3040-\u309F\u30A0-\u30FF\u31F0-\u31FF\uAC00-\uD7AF\u2E80-\u2FD5\uFF5F-\uFF9F\u3000-\u303F\u31F0-\u31FF\u3220-\u3243\u3280-\u337FA-z0-9 .,?!'\"()\[\]{}\-+|*_/\\])(  [A-Za-zÀ-ÖØ-öø-ÿĀ-ſƀ-ɏ ]+)?\s*?(.)*$"#).unwrap()
});

impl Callout {
    pub fn new(
        callout_type: CalloutType,
        header: String,
        content: Vec<String>,
        sub_callouts: Vec<Callout>,
    ) -> Callout {
        Callout {
            callout_type,
            header,
            content,
            sub_callouts,
        }
    }

    pub fn to_anki_entry(&self) -> String {
        todo!()
    }
}

impl TryFrom<&str> for Callout {
    type Error = CalloutError;

    fn try_from(value: &str) -> Result<Callout, CalloutError> {
        let content_length = (value.par_lines().collect::<Vec<_>>().len() + 1).max(3);
        let mut split_iter = value.split_terminator('\n').map(|line| line.trim());

        let header_line = match split_iter.next() {
            Some(line) => line,
            None => panic!("{:?}", CalloutError::EmptyString),
        };

        let caps = RE_HEADER.captures(header_line).expect(
            "first line should be formatted as a callout '> [!TYPE] TEXT TRANSLITERATION EMOJI'",
        );

        let callout_type: CalloutType = caps[1].try_into()?;
        let header: String = caps[2].to_string();
        let transliteration = caps[3].to_string();
        let emoji = caps[4].to_string();

        let mut content = Vec::with_capacity(content_length);
        if !emoji.is_empty() {
            content.push(emoji);
        }
        if !transliteration.is_empty() {
            content.push(transliteration);
        }
        match callout_type {
            CalloutType::Word => {
                let sub_callouts = Vec::new();
                let mut in_links_callout = false;
                for line in split_iter {
                    if line.starts_with("> ^") {
                        break;
                    }
                    if line.starts_with("> > [!links]") {
                        in_links_callout = true;
                        continue;
                    } else if in_links_callout {
                        if line.starts_with("> >") {
                            continue;
                        } else {
                            in_links_callout = false;
                        }
                    }
                    content.push(
                        line.strip_prefix("> ")
                            .expect("text should be prefixed with a '> '")
                            .to_string(),
                    );
                }
                Ok(Callout::new(callout_type, header, content, sub_callouts))
            }
            _ => {
                let mut sub_callouts: Vec<Callout> = Vec::with_capacity(content_length);
                let mut prev = "";
                if true {
                    'split_loop: loop {
                        if !prev.is_empty() {
                            content.push(prev.to_string());
                            prev = "";
                        }
                        let mut line = split_iter.next().unwrap_or("");
                        if line.is_empty() {
                            break 'split_loop;
                        }
                        line = line
                            .strip_prefix("> ")
                            .expect("text should be prefixed with a '> '");
                        if line.starts_with("> [!") {
                            let mut sub_callout_vector: Vec<&str> =
                                Vec::with_capacity(content_length);
                            'sub_callout: loop {
                                let next_line = split_iter.next().ok_or("").unwrap();
                                if next_line.is_empty() {
                                    break 'sub_callout;
                                }
                                if !next_line.starts_with("> ") {
                                    prev = next_line;
                                    break 'sub_callout;
                                }
                                sub_callout_vector.push(next_line);
                            }
                            sub_callouts.push(sub_callout_vector.join("\n").try_into()?);
                        }
                    }
                }
                Ok(Callout::new(callout_type, header, content, sub_callouts))
            }
        }
    }
}

impl TryFrom<String> for Callout {
    type Error = CalloutError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Callout::try_from(value.as_str())
    }
}
impl TryFrom<Vec<&str>> for Callout {
    type Error = CalloutError;
    fn try_from(value: Vec<&str>) -> Result<Self, Self::Error> {
        let content_length = (value.len() + 1).max(3);
        let mut value_iter = value.iter();

        let header_line = match value_iter.next() {
            Some(line) => line,
            None => panic!("{:?}", CalloutError::EmptyString),
        };

        let caps = RE_HEADER.captures(header_line).expect(
            "first line should be formatted as a callout '> [!TYPE] TEXT TRANSLITERATION EMOJI'",
        );

        let callout_type: CalloutType = caps[1].try_into()?;
        let header: String = caps[2].to_string();
        let transliteration = caps[3].to_string();
        let emoji = caps[4].to_string();

        let mut content = Vec::with_capacity(content_length);
        if !emoji.is_empty() {
            content.push(emoji);
        }
        if !transliteration.is_empty() {
            content.push(transliteration);
        }
        match callout_type {
            CalloutType::Word => {
                let sub_callouts = Vec::new();
                let mut in_links_callout = false;
                for line in value_iter {
                    if line.starts_with("> ^") {
                        break;
                    }
                    if line.starts_with("> > [!links]") {
                        in_links_callout = true;
                        continue;
                    } else if in_links_callout {
                        if line.starts_with("> >") {
                            continue;
                        } else {
                            in_links_callout = false;
                        }
                    }
                    content.push(
                        line.strip_prefix("> ")
                            .expect("text should be prefixed with a '> '")
                            .to_string(),
                    );
                }
                Ok(Callout::new(callout_type, header, content, sub_callouts))
            }
            _ => {
                let mut sub_callouts: Vec<Callout> = Vec::with_capacity(content_length);
                let mut prev = "";
                if true {
                    'split_loop: loop {
                        if !prev.is_empty() {
                            content.push(prev.to_string());
                            prev = "";
                        }
                        let mut line = *value_iter.next().unwrap_or(&"");
                        if line.is_empty() {
                            break 'split_loop;
                        }
                        line = line
                            .strip_prefix("> ")
                            .expect("text should be prefixed with a '> '");
                        if line.starts_with("> [!") {
                            let mut sub_callout_vector: Vec<&str> =
                                Vec::with_capacity(content_length);
                            'sub_callout: loop {
                                let next_line = value_iter.next().ok_or("").unwrap();
                                if next_line.is_empty() {
                                    break 'sub_callout;
                                }
                                if !next_line.starts_with("> ") {
                                    prev = next_line;
                                    break 'sub_callout;
                                }
                                sub_callout_vector.push(next_line);
                            }
                            sub_callouts.push(sub_callout_vector.join("\n").try_into()?);
                        }
                    }
                }
                Ok(Callout::new(callout_type, header, content, sub_callouts))
            }
        }
    }
}
