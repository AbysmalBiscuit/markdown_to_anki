use crate::anki_connect_client::{
    AnkiConnectClient,
    error::APIError,
    model::Model,
    note::Note,
    notes::params::{AddNote, AddNoteOptions, DuplicateScopeOptions},
};
use std::collections::{HashMap, HashSet};

use rayon::prelude::*;
use serde::Serialize;

use crate::callout::Callout;

use super::traits::InternalModelMethods;

#[derive(Debug, Default, Serialize)]
pub struct Basic {
    pub markdown_id: String,
    pub front: String,
    pub back: String,
}

impl InternalModelMethods for Basic {
    fn from_callout(&self, callout: &Callout, header_lang: Option<&str>) -> Self {
        Basic {
            markdown_id: callout.markdown_id.to_owned(),
            front: callout.header.clone(),
            back: callout.content_to_html(header_lang),
        }
    }

    fn create_model(&self, client: &AnkiConnectClient, css: &str) -> Result<Model, APIError> {
        let templates = [
            [
                ("Name", "Recognition"),
                (
                    "Front",
                    r#"<br>
<div class="center">{{Front}}</div>
<br>
{{Audio}}
<br>
<div class="center">TTS M:{{tts ko_KR voices=com.samsung.SMT-ko-KR-SMTg01,Microsoft_Heami:Front}}</div>
<br>
TTS W: {{tts ko_KR voices=com.samsung.SMT-ko-KR-SMTl01:Korean}}"#,
                ),
                (
                    "Back",
                    r#"{{FrontSide}}

<hr id=answer>

<div class="center">{{Back}}</div>
<br>
<!--{{Audio}}
<br>
<div class="center">TTS M:{{tts ko_KR voices=com.samsung.SMT-ko-KR-SMTg01,Microsoft_Heami:Front}}</div>
<br>
TTS W: {{tts ko_KR voices=com.samsung.SMT-ko-KR-SMTl01:Korean}}-->"#,
                ),
            ],
            [
                ("Name", "Recall"),
                (
                    "Front",
                    r#"<br>
<div class="center">{{Back}}</div>"#,
                ),
                (
                    "Back",
                    r#"{{FrontSide}}

<hr id=answer>

<div class="center">{{Front}}</div>
{{Audio}}
<br>
<div class="center">TTS M:{{tts ko_KR voices=com.samsung.SMT-ko-KR-SMTg01,Microsoft_Heami:Front}}</div>
<br>
TTS W: {{tts ko_KR voices=com.samsung.SMT-ko-KR-SMTl01:Korean}}"#,
                ),
            ],
            [
                ("Name", "Listen"),
                (
                    "Front",
                    r#"<br>
{{Audio}}
<br>
<div class="center">TTS M:{{tts ko_KR voices=com.samsung.SMT-ko-KR-SMTg01,Microsoft_Heami:Front}}</div>
<br>
TTS W: {{tts ko_KR voices=com.samsung.SMT-ko-KR-SMTl01:Korean}}"#,
                ),
                (
                    "Back",
                    r#"{{FrontSide}}

<hr id=answer>

<div class="center">{{Front}}</div>
<br>
<div class="center">{{Back}}</div>
<br>
<!--{{Audio}}
{{tts ko_KR voices=com.samsung.SMT-ko-KR-SMTg01,Microsoft_Heami:Front}}
{{Back}}-->"#,
                ),
            ],
        ];
        let card_templates = templates
            .par_iter()
            .map(|template| {
                template
                    .iter()
                    .map(|(k, v)| (k.to_string(), v.to_string()))
                    .collect::<HashMap<String, String>>()
            })
            .collect::<Vec<_>>();

        client.models.create_model(
            "md2anki Basic",
            vec!["MarkdownID", "Front", "Back", "Audio"],
            Some(css),
            false,
            card_templates,
        )
    }

    fn to_note(self, model: Model) -> Result<Note, APIError> {
        let mut field_values: HashMap<String, String> = HashMap::with_capacity(2);
        field_values.insert("MarkdownID".into(), self.markdown_id);
        field_values.insert("Front".into(), self.front);
        field_values.insert("Back".into(), self.back);
        let mut tags: HashSet<String> = HashSet::with_capacity(1);
        tags.insert("md2anki".to_string());
        let media: Vec<String> = Vec::new();
        todo!()
        // Note::new(model, field_values, tags, media)
        // Ok(InternalNote::new(model, field_values, tags))
    }

    fn to_add_note<'a>(&'a self, deck_name: &'a str, model_name: &'a str) -> AddNote<'a> {
        let mut fields: HashMap<&str, &str> = HashMap::with_capacity(3);
        fields.insert("MarkdownID", self.markdown_id.as_str());
        fields.insert("Front", self.front.as_str());
        fields.insert("Back", self.back.as_str());
        // fields.insert("MarkdownID", self.markdown_id);

        AddNote::new(
            deck_name,
            model_name,
            fields,
            AddNoteOptions::new(
                true,
                "deck",
                DuplicateScopeOptions::new(deck_name, true, false),
            ),
            Vec::new(),
            None,
            None,
            None,
        )
    }
}
