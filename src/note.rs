use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::callout::{callout::Callout, callout_type::CalloutType};

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
        let mut content = Vec::with_capacity((value.content.len() + 2) * 2);
        content.push(value.content.join("\n"));
        if !value.sub_callouts.is_empty() {
            for sub_callout in &value.sub_callouts {
                match sub_callout.callout_type {
                    CalloutType::Links => continue,
                    _ => content.push(sub_callout.sub_callout_to_html()),
                }
            }
        }

        // TODO: parse lines starting with - into unordered lists

        // TODO: parse lines starting with \d+\. into ordered lists

        Basic {
            front: value.header.clone(),
            back: content
                .par_iter()
                .map(|text| text.trim())
                .collect::<Vec<_>>()
                .join("\n"),
        }
    }
}
