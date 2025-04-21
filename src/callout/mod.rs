pub(crate) mod callout_type;
pub(crate) mod content;
pub(crate) mod error;
pub(crate) mod try_from;

use crate::error::GenericError;
use callout_type::CalloutType;
use content::CalloutContent;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rayon::prelude::*;
use std::fmt::Display;
use std::fs::read_to_string;
use std::path::Path;

#[derive(Debug)]
pub struct Callout {
    pub id: String,
    pub markdown_id: String,
    pub callout_type: CalloutType,
    pub header: String,
    pub content: Vec<CalloutContent>,
    pub sub_callouts: Vec<Callout>,
}

impl Callout {
    pub fn new(
        id: String,
        markdown_id: String,
        callout_type: CalloutType,
        header: String,
        content: Vec<CalloutContent>,
        sub_callouts: Vec<Callout>,
    ) -> Callout {
        Callout {
            id,
            markdown_id,
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

    pub fn content_to_html(&self, header_lang: Option<&str>) -> String {
        let mut content: Vec<String> = Vec::with_capacity(self.content.len());
        let mut unconverted_content: Vec<&str> = Vec::with_capacity(self.content.len());

        for item in &self.content {
            match item {
                CalloutContent::Text(text) => unconverted_content.push(text.as_str()),
                CalloutContent::SubCalloutIndex(index) => {
                    if !unconverted_content.is_empty() {
                        content.push(markdown::to_html(&unconverted_content.join("\n\n")));
                        unconverted_content.clear();
                    }
                    content.push(
                        self.sub_callouts
                            .get(*index)
                            .and_then(|sub_callout| match sub_callout.callout_type {
                                CalloutType::Links => None,
                                _ => Some(sub_callout.to_html(header_lang)),
                            })
                            .unwrap_or("".into()),
                    )
                }
            }
        }
        if !unconverted_content.is_empty() {
            content.push(markdown::to_html(&unconverted_content.join("\n\n")));
            unconverted_content.clear();
        }

        if !content.is_empty() {
            content
                .into_par_iter()
                .map(|content| format!(r#"<p dir="auto">{}</p>"#, content))
                .collect::<Vec<_>>()
                .join("\n")
        } else {
            "".to_string()
        }
    }

    pub fn to_html(&self, header_lang: Option<&str>) -> String {
        let header = if self.header.is_empty() {
            self.callout_type.get_name(header_lang)
        } else {
            self.header.clone()
        };

        format!(
            r#"<div data-callout="{0}" class="callout"><div class="callout-title"><div class="callout-icon"></div>{1}</div>{2}</div>"#,
            self.callout_type,
            header,
            self.content_to_html(header_lang)
        )
    }

    pub fn to_anki_markdown_entry(&self, card_type: Option<&str>) -> String {
        format!(
            "<pre>\nSTART\n{}\n{}\nBack: {}\nEND\n</pre>",
            card_type.unwrap_or("Basic"),
            self.header,
            self.content_to_html(None)
        )
    }
}

impl Default for Callout {
    fn default() -> Self {
        Callout {
            id: "".into(),
            markdown_id: "".into(),
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
