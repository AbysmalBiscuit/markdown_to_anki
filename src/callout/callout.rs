use crate::callout::callout_error::CalloutError;
use crate::callout::callout_type::CalloutType;
use crate::error::GenericError;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rayon::prelude::*;
use regex::Regex;
use std::fmt::Display;
use std::fs::read_to_string;
use std::path::Path;
use std::sync::LazyLock;

static RE_HEADER: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"^(?:> )?> \[!(.+?)\][+-]? ?([\u4E00-\u9FFF\u3400-\u4DBF\uF900-\uFAFF\u3040-\u309F\u30A0-\u30FF\u31F0-\u31FF\uAC00-\uD7AF\u2E80-\u2FD5\uFF5F-\uFF9F\u3000-\u303F\u31F0-\u31FF\u3220-\u3243\u3280-\u337FA-Za-z0-9.,?!'"()\[\]{}\-+|*_/\\]+(?: [\u4E00-\u9FFF\u3400-\u4DBF\uF900-\uFAFF\u3040-\u309F\u30A0-\u30FF\u31F0-\u31FF\uAC00-\uD7AF\u2E80-\u2FD5\uFF5F-\uFF9F\u3000-\u303F\u31F0-\u31FF\u3220-\u3243\u3280-\u337FA-Za-z0-9.,?!'"()\[\]{}\-+|*_/\\]+)*)?(  [A-Za-zÀ-ÖØ-öø-ÿĀ-ſƀ-ɏ ]*)? *(.*?)?$"#).unwrap()
});

#[derive(Debug)]
pub struct Callout {
    pub callout_type: CalloutType,
    pub header: String,
    pub content: Vec<String>,
    pub sub_callouts: Vec<Callout>,
}

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

    pub fn extract_callouts(path: &Path) -> Result<Vec<Callout>, GenericError> {
        let content: String = read_to_string(path)?;
        let blocks: Vec<String> = content
            .split("\n> [!")
            .skip(1)
            .collect::<Vec<_>>()
            .into_par_iter()
            .map(|block| block.trim())
            .filter(|block| {
                !block.is_empty() && (block.starts_with("word") || block.starts_with("rule"))
            })
            .map(|block| format!("> [!{}", block))
            .collect();

        let callouts: Vec<Callout> = blocks
            .par_iter()
            .map(|block| {
                block
                    .par_split('\n')
                    .filter(|line| line.starts_with('>'))
                    .collect::<Vec<_>>()
            })
            .map(Callout::try_from)
            .map(Result::unwrap)
            .collect();

        Ok(callouts)
    }

    pub fn sub_callout_to_html(&self) -> String {
        let mut content = Vec::with_capacity((self.content.len() + 2) * 2);
        content.push(
            self.content
                .clone()
                .par_iter()
                .map(|line| {
                    if line.starts_with("> ") {
                        line.strip_prefix("> ").unwrap().to_string()
                    } else {
                        line.to_string()
                    }
                })
                .collect::<Vec<_>>()
                .join("\n"),
        );

        if !self.sub_callouts.is_empty() {
            for sub_callout in &self.sub_callouts {
                content.push(sub_callout.sub_callout_to_html());
            }
        }

        let header = if self.header.is_empty() {
            self.callout_type.callout_default_header()
        } else {
            self.header.clone()
        };

        let content_text = if !content.is_empty() {
            format!(
                r#"<div class="callout-content">{0}</div>"#,
                content
                    .into_par_iter()
                    .map(|content| format!(r#"<p dir="auto">{}</p>"#, content))
                    .collect::<Vec<_>>()
                    .join("\n")
            )
        } else {
            "".to_string()
        };

        format!(
            r#"<div data-callout="{0}" class="callout"><div class="callout-title"><div class="callout-icon"></div>{1}</div>{2}</div>"#,
            self.callout_type, header, content_text
        )
    }

    // pub fn to_html(&self) -> String {
    //     let mut content: Vec<String> = Vec::with_capacity((&self.content.len() + 2) * 2);
    //
    //     content.push(self.content.join("\n"));
    //     if !&self.sub_callouts.is_empty() {
    //         for sub_callout in &self.sub_callouts {
    //             match sub_callout.callout_type {
    //                 CalloutType::Links => continue,
    //                 _ => content.push(sub_callout.sub_callout_to_html()),
    //             }
    //         }
    //     }
    //
    //     content
    //         .par_iter()
    //         .map(|text| text.trim())
    //         .collect::<Vec<_>>()
    //         .join("\n")
    // }

    pub fn to_anki_markdown_entry(&self, card_type: Option<&str>) -> String {
        let note_type = card_type.unwrap_or("Basic");
        let mut content = Vec::with_capacity((self.content.len() + 2) * 2);
        content.push(self.content.join("\n"));
        if !self.sub_callouts.is_empty() {
            for sub_callout in &self.sub_callouts {
                match sub_callout.callout_type {
                    CalloutType::Links => continue,
                    _ => content.push(sub_callout.sub_callout_to_html()),
                }
            }
        }
        format!(
            "<pre>\nSTART\n{}\n{}\nBack: {}\nEND\n</pre>",
            note_type,
            self.header,
            content.join("\n")
        )
    }
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

        let mut content: Vec<String> = Vec::with_capacity(content_length);
        if !emoji.is_empty() {
            content.push(emoji);
        }
        if !transliteration.is_empty() {
            content.push(transliteration);
        }

        let mut sub_callouts: Vec<Callout> = Vec::with_capacity(content_length);
        let mut prev: &str = "";
        let mut line: &str;
        let mut next: &str = "";

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

impl Display for Callout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self,)
    }
}
