pub(crate) mod callout_content;
pub(crate) mod callout_error;
pub(crate) mod callout_type;
pub(crate) mod try_from;

use crate::error::GenericError;
use callout_content::CalloutContent;
use callout_error::CalloutError;
use callout_type::CalloutType;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rayon::prelude::*;
use regex::Regex;
use std::fmt::Display;
use std::fs::read_to_string;
use std::path::Path;
use std::sync::LazyLock;

use try_from::*;

#[derive(Debug)]
pub struct Callout {
    pub id: String,
    pub callout_type: CalloutType,
    pub header: String,
    pub content: Vec<CalloutContent>,
    pub sub_callouts: Vec<Callout>,
}

impl Callout {
    pub fn new(
        id: String,
        callout_type: CalloutType,
        header: String,
        content: Vec<CalloutContent>,
        sub_callouts: Vec<Callout>,
    ) -> Callout {
        Callout {
            id,
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
            .filter(Result::is_ok)
            .map(Result::unwrap)
            .collect();

        Ok(callouts)
    }

    pub fn to_html(&self, header_lang: Option<&str>) -> String {
        let mut content = Vec::with_capacity((self.content.len() + 2) * 2);
        content.push(
            self.content
                .clone()
                .par_iter()
                .filter_map(|item| match item {
                    CalloutContent::Text(text) => Some(text).cloned(),
                    CalloutContent::SubCalloutIndex(index) => self
                        .sub_callouts
                        .get(*index)
                        .and_then(|sub_callout| match sub_callout.callout_type {
                            CalloutType::Links => None,
                            _ => Some(sub_callout.to_html(None)),
                        }),
                    _ => None,
                })
                .collect::<Vec<_>>()
                .join("\n"),
        );

        let header = if self.header.is_empty() {
            self.callout_type.get_name(header_lang)
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

    pub fn to_anki_markdown_entry(&self, card_type: Option<&str>) -> String {
        format!(
            "<pre>\nSTART\n{}\n{}\nBack: {}\nEND\n</pre>",
            card_type.unwrap_or("Basic"),
            self.header,
            self.content
                .par_iter()
                .filter_map(|item| match item {
                    CalloutContent::Text(text) => Some(text).cloned(),
                    CalloutContent::SubCalloutIndex(index) => {
                        self.sub_callouts.get(*index).and_then(|sub_callout| {
                            match sub_callout.callout_type {
                                CalloutType::Links => None,
                                _ => Some(sub_callout.to_html(None)),
                            }
                        })
                    }
                    _ => None,
                })
                .map(|item| item.to_owned())
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

impl Default for Callout {
    fn default() -> Self {
        Callout {
            id: "".into(),
            callout_type: CalloutType::Word,
            header: "Default".into(),
            content: Vec::new(),
            sub_callouts: Vec::new(),
        }
    }
}

impl Display for Callout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self,)
    }
}
