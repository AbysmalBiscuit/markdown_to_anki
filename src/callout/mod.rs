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
    pub content: Vec<String>,
    pub sub_callouts: Vec<Callout>,
}

impl Callout {
    pub fn new(
        id: String,
        callout_type: CalloutType,
        header: String,
        content: Vec<String>,
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

    pub fn sub_callout_to_html(&self, header_lang: Option<&str>) -> String {
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
                content.push(sub_callout.sub_callout_to_html(header_lang));
            }
        }

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

    // pub fn content_to_html(&self) -> String {
    //     let mut html = String::new();
    //
    //     let mut in_ol = false;
    //     let mut in_ul = false;
    //
    //     for item in &self.content {
    //         match item {
    //             CalloutContent::Text(value) => html.push_str(value),
    //             CalloutContent::UnorderedListItem(value) =>    ,
    //             CalloutContent::OrderedListItem(_) => todo!(),
    //             CalloutContent::SubCalloutIndex(_) => todo!(),
    //             CalloutContent::Blockquote(_) => todo!(),
    //             CalloutContent::HorizontalLine => todo!(),
    //             CalloutContent::UnorderedListStart => todo!(),
    //             CalloutContent::UnorderedListEnd => todo!(),
    //             CalloutContent::OrderedListStart => todo!(),
    //             CalloutContent::OrderedListEnd => todo!(),
    //         };
    //     }
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
                    _ => content.push(sub_callout.sub_callout_to_html(None)),
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
