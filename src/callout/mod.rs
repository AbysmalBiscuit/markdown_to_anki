pub mod callout_type;
pub mod content;
pub mod error;
pub mod try_from;

use callout_type::CalloutType;
use content::CalloutContent;
use error::CalloutError;
use rayon::iter::{Either, IntoParallelIterator, ParallelIterator};
use rayon::prelude::*;
use std::fmt::Display;
use std::fs::read_to_string;
use std::path::Path;

use crate::callout::callout_type::SupportedLanguages;
use crate::note_operation::NoteOperation;

#[derive(Debug)]
pub struct ExtractCalloutsResult {
    pub callouts: Vec<Callout>,
    pub failed: Vec<(String, CalloutError)>,
}

impl From<(Vec<Callout>, Vec<(String, CalloutError)>)> for ExtractCalloutsResult {
    fn from(value: (Vec<Callout>, Vec<(String, CalloutError)>)) -> Self {
        Self {
            callouts: value.0,
            failed: value.1,
        }
    }
}

#[derive(Debug, Default)]
pub struct Callout {
    pub markdown_id: String,
    pub operation: NoteOperation,
    pub type_: CalloutType,
    pub header: String,
    pub content: Vec<CalloutContent>,
    pub sub_callouts: Vec<Self>,
}

impl Callout {
    pub const fn new(
        markdown_id: String,
        callout_type: CalloutType,
        header: String,
        content: Vec<CalloutContent>,
        sub_callouts: Vec<Self>,
    ) -> Self {
        Self {
            markdown_id,
            operation: NoteOperation::Nop,
            type_: callout_type,
            header,
            content,
            sub_callouts,
        }
    }

    pub fn extract_callouts(path: &Path) -> ExtractCalloutsResult {
        let content: String = match read_to_string(path) {
            Ok(text) => text,
            Err(err) => {
                return ExtractCalloutsResult::from((
                    vec![],
                    vec![(String::new(), CalloutError::Io(err))],
                ));
            }
        };
        let blocks: Vec<String> = content
            .split("\n> [!")
            .skip(1)
            .collect::<Vec<_>>()
            .into_par_iter()
            .map(str::trim)
            .filter(|block| {
                !block.is_empty() // && (block.starts_with("word") || block.starts_with("rule"))
            })
            .map(|block| format!("> [!{block}"))
            .collect();

        let (callouts, failed): (Vec<Self>, Vec<(String, CalloutError)>) =
            blocks.into_par_iter().partition_map(|block| {
                let block = block
                    .par_split('\n')
                    .filter(|line| line.starts_with('>'))
                    .collect::<Vec<_>>();
                match Self::try_from(&block) {
                    Ok(callout) => match callout.type_ {
                        CalloutType::Word | CalloutType::Rule => {
                            if callout.markdown_id.is_empty() {
                                Either::Right((block.join("\n"), CalloutError::NoMarkdownID))
                            } else {
                                Either::Left(callout)
                            }
                        }
                        _ => Either::Right((String::new(), CalloutError::NotFlashcardCompatible)),
                    },
                    Err(err) => Either::Right((block.join("\n"), err)),
                }
            });
        let failed: Vec<(String, CalloutError)> = failed
            .into_par_iter()
            .filter(|(_, err)| !matches!(err, CalloutError::NotFlashcardCompatible))
            .collect();

        (callouts, failed).into()
    }

    pub fn content_to_html(&self, header_lang: &SupportedLanguages) -> String {
        if self.content.is_empty() {
            return String::new();
        }

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
                            .and_then(|sub_callout| match sub_callout.type_ {
                                CalloutType::Links => None,
                                _ => Some(sub_callout.to_html(header_lang)),
                            })
                            .unwrap_or(String::new()),
                    );
                }
            }
        }
        if !unconverted_content.is_empty() {
            content.push(markdown::to_html(&unconverted_content.join("\n\n")));
            unconverted_content.clear();
        }

        content.join("\n")
    }

    pub fn to_html(&self, header_lang: &SupportedLanguages) -> String {
        let header = if self.header.is_empty() {
            self.type_.get_name(header_lang)
        } else {
            self.header.clone()
        };

        format!(
            r#"<details data-callout="{0}" class="callout"><summary class="callout-title"><div class="callout-icon"></div>{1}</summary>{2}</details>"#,
            self.type_,
            header,
            self.content_to_html(header_lang),
        )
    }

    pub fn to_anki_markdown_entry(&self, card_type: Option<&str>) -> String {
        format!(
            "<pre>\nSTART\n{}\n{}\nBack: {}\nEND\n</pre>",
            card_type.unwrap_or("Basic"),
            self.header,
            self.content_to_html(&SupportedLanguages::default())
        )
    }
}

impl Display for Callout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
