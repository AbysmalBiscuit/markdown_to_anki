use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::callout::{Callout, callout_type::CalloutType, content::CalloutContent};

#[derive(Debug)]
pub struct Word {
    front: String,
    back: String,
    audio: String,
    notation: String,
    quick_notes: String,
    rules: String,
    examples: String,
    related_words_rules: String,
    select_conjugations: String,
    irregular_rules: String,
    additinoal_rules: String,
    phonetics: String,
    references: String,
}

#[derive(Debug)]
struct Rule {
    front: String,
    back: String,
    audio: String,
    notation: String,
    quick_notes: String,
    alternate_phrasing: String,
    rules: String,
    rule_alternate_meanings: String,
    other_rules_with_similar_meanings: String,
    rule_used_but_unrelated_to_primary: String,
    irregular_rules: String,
    phonetics: String,
    references: String,
}

#[derive(Debug)]
pub struct Basic {
    front: String,
    back: String,
}

impl From<&Callout> for Basic {
    fn from(value: &Callout) -> Self {
        // TODO: parse lines starting with - into unordered lists

        // TODO: parse lines starting with \d+\. into ordered lists

        Basic {
            front: value.header.clone(),
            back: value
                .content
                .par_iter()
                .filter_map(|item| match item {
                    CalloutContent::Text(text) => Some(text).cloned(),
                    CalloutContent::SubCalloutIndex(index) => value
                        .sub_callouts
                        .get(*index)
                        .and_then(|sub_callout| match sub_callout.callout_type {
                            CalloutType::Links => None,
                            _ => Some(sub_callout.to_html(None)),
                        }),
                    _ => None,
                })
                .map(|text| text.trim().to_string())
                .collect::<Vec<_>>()
                .join("\n"),
        }
    }
}
