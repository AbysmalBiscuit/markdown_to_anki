use std::sync::LazyLock;
use strum_macros::Display;

// use rayon::prelude::*;
use regex::Regex;

// use hyperscan::prelude::*;

#[derive(Display, Debug)]
pub enum CalloutError {
    EmptyString,
    UnknownType,
    FailedToParseHeader,
}

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

impl From<CalloutType> for String {
    fn from(val: CalloutType) -> Self {
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

#[derive(Debug)]
pub struct Callout {
    pub callout_type: CalloutType,
    pub header: String,
    pub content: Vec<String>,
    pub sub_callouts: Vec<Callout>,
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
    Regex::new(r#"^(?:> )?> \[!(.+?)\][+-]? ?([\u4E00-\u9FFF\u3400-\u4DBF\uF900-\uFAFF\u3040-\u309F\u30A0-\u30FF\u31F0-\u31FF\uAC00-\uD7AF\u2E80-\u2FD5\uFF5F-\uFF9F\u3000-\u303F\u31F0-\u31FF\u3220-\u3243\u3280-\u337FA-z0-9.,?!'"()\[\]{}\-+|*_/\\]+(?: [\u4E00-\u9FFF\u3400-\u4DBF\uF900-\uFAFF\u3040-\u309F\u30A0-\u30FF\u31F0-\u31FF\uAC00-\uD7AF\u2E80-\u2FD5\uFF5F-\uFF9F\u3000-\u303F\u31F0-\u31FF\u3220-\u3243\u3280-\u337FA-z0-9.,?!'"()\[\]{}\-+|*_/\\])*)?(  [A-Za-zÀ-ÖØ-öø-ÿĀ-ſƀ-ɏ ]*)?\s*(.*?)?$"#).unwrap()
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

    // TODO: this funciton
    // fn parse_description(&self) -> Vec<&str> {
    //     todo!()
    // }

    fn sub_callout_to_anki(&self) -> String {
        let mut output = Vec::with_capacity((self.content.len() + 2) * 2);
        output.push(self.content.join("\n"));
        if !self.sub_callouts.is_empty() {
            for sub_callout in &self.sub_callouts {
                output.push(sub_callout.sub_callout_to_anki());
            }
        }
        format!(
            "* {}\n* {}\n * {}",
            self.header,
            self.content.join("\n* "),
            output.join("\n")
        )
    }

    pub fn to_anki_entry(&self, card_type: Option<&str>) -> String {
        let card_type = card_type.unwrap_or("Basic");
        let mut output = Vec::with_capacity((self.content.len() + 2) * 2);
        output.push(self.content.join("\n"));
        if !self.sub_callouts.is_empty() {
            for sub_callout in &self.sub_callouts {
                match sub_callout.callout_type {
                    CalloutType::Links => continue,
                    _ => output.push(sub_callout.sub_callout_to_anki()),
                }
            }
        }
        format!(
            "<pre>\nSTART\n{}\n{}\nBack: {}\nEND\n</pre>",
            card_type,
            self.header,
            output.join("\n")
        )
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

        let caps = RE_HEADER.captures(header_line).unwrap_or_else(|| {
            dbg!("panicking", &value);
            panic!("Failed to parse header.");
        });

        //     .expect(
        //     "first line should be formatted as a callout '> [!TYPE] TEXT TRANSLITERATION EMOJI'",
        // );

        let callout_type: CalloutType = caps[1].try_into()?;
        let header: String = caps
            .get(2)
            .map_or(String::new(), |m| m.as_str().to_string());
        let transliteration = caps
            .get(3)
            .map(|m| m.as_str().trim().to_string())
            .unwrap_or("".to_string());
        let emoji = caps
            .get(4)
            .map(|m| m.as_str().trim().to_string())
            .unwrap_or("".to_string());

        let mut content = Vec::with_capacity(content_length);
        if !emoji.is_empty() {
            content.push(emoji);
        }
        if !transliteration.is_empty() {
            content.push(transliteration);
        }

        // dbg!(&value);

        let mut sub_callouts: Vec<Callout> = Vec::with_capacity(content_length);
        let mut prev = "";
        let mut line: &str;
        let mut next = "";

        // TODO: rewrite this to be a loop around indeces instead of iter
        'split_loop: loop {
            if !prev.is_empty() {
                if prev.starts_with("> ^") {
                    break 'split_loop;
                }
                content.push(prev.to_string());
                prev = "";
            }
            if !next.is_empty() {
                line = next;
                next = "";
            } else {
                line = *value_iter.next().unwrap_or(&"");
                next = *value_iter.next().unwrap_or(&"");
            }

            if line.is_empty() {
                break 'split_loop;
            }

            if line.starts_with("> > [!") {
                let mut sub_callout_vector: Vec<&str> = Vec::with_capacity(content_length);
                sub_callout_vector.push(line);
                'sub_callout: loop {
                    let next_line = value_iter.next().unwrap_or(&"");
                    if next_line.is_empty() {
                        break 'sub_callout;
                    }
                    if !next_line.starts_with("> >") {
                        prev = next_line;
                        break 'sub_callout;
                    }
                    sub_callout_vector.push(next_line);
                }
                sub_callouts.push(sub_callout_vector.try_into()?);
            } else {
                line = line.strip_prefix("> ").unwrap_or("");
                if line.starts_with('^') {
                    break 'split_loop;
                }
                content.push(line.trim().to_string());
            }
        }
        if content.last().map_or_else(|| "", |item| item).is_empty() {
            content.pop();
        }
        if content.last().map_or_else(|| "", |item| item).is_empty() {
            content.pop();
        }
        Ok(Callout::new(callout_type, header, content, sub_callouts))
    }
}
