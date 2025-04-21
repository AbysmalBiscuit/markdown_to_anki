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
    pub front: String,
    pub back: String,
}
impl Basic {
    pub fn from_callout(callout: &Callout, header_lang: Option<&str>) -> Self {
        Basic {
            front: callout.header.clone(),
            back: callout.content_to_html(header_lang),
        }
    }
}
