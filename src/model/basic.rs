use crate::{
    anki_connect::{
        models_client::params::CreateModel,
        note::NoteId,
        notes_client::params::{
            AddNoteNote, AddNoteOptions, DuplicateScopeOptions, UpdateNoteFields,
            UpdateNoteFieldsNote,
        },
    },
    note_operation::NoteOperation,
};
use std::{borrow::Cow, collections::HashMap};

use rayon::prelude::*;
use serde::Serialize;

use crate::callout::Callout;

use super::InternalModelMethods;

#[derive(Debug, Default, Clone, Serialize)]
pub struct Basic<'a> {
    deck_name: &'a str,
    operation: NoteOperation,
    markdown_id: String,
    front: String,
    back: String,
}

impl<'a> InternalModelMethods<'a> for Basic<'a> {
    fn from_callout(
        &self,
        callout: &Callout,
        header_lang: Option<&str>,
        deck_name: &'a str,
    ) -> Self {
        Basic {
            deck_name: &deck_name,
            operation: callout.operation,
            markdown_id: callout.markdown_id.to_owned(),
            front: callout.header.clone(),
            back: callout.content_to_html(header_lang),
        }
    }

    fn to_create_model(&self, model_name: &'a str, css: Option<&'a str>) -> CreateModel<'a> {
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
            .into_par_iter()
            .map(|template| {
                template
                    .into_par_iter()
                    .map(|(k, v)| (Cow::from(k), Cow::from(v)))
                    .collect::<HashMap<Cow<'a, str>, Cow<'a, str>>>()
            })
            .collect::<Vec<_>>();

        CreateModel::new(
            model_name,
            vec!["MarkdownID", "Front", "Back", "Audio"],
            css,
            Some(false),
            card_templates,
        )
    }

    fn to_update_note(&'a self, note_id: &'a NoteId) -> UpdateNoteFields<'a> {
        let mut field_values: HashMap<&'a str, &'a str> = HashMap::with_capacity(3);
        field_values.insert("MarkdownID", self.markdown_id.as_str());
        field_values.insert("Front", self.front.as_str());
        field_values.insert("Back", self.back.as_str());
        UpdateNoteFields::new(UpdateNoteFieldsNote::new(
            note_id,
            field_values,
            self.get_audio(),
            // None,
            // video,
            None,
            // picture,
            None,
            // tags
            None,
        ))
    }

    fn get_fields(&'a self) -> HashMap<&'a str, &'a str> {
        let mut field_values: HashMap<&'a str, &'a str> = HashMap::with_capacity(3);
        field_values.insert("MarkdownID", self.markdown_id.as_str());
        field_values.insert("Front", self.front.as_str());
        field_values.insert("Back", self.back.as_str());
        field_values
    }

    fn to_add_note(&'a self, deck_name: &'a str, model_name: &'a str) -> AddNoteNote<'a> {
        let mut fields: HashMap<&str, &str> = HashMap::with_capacity(3);
        fields.insert("MarkdownID", self.markdown_id.as_str());
        fields.insert("Front", self.front.as_str());
        fields.insert("Back", self.back.as_str());

        AddNoteNote::new(
            deck_name,
            model_name,
            fields,
            AddNoteOptions::new(
                false,
                "deck",
                DuplicateScopeOptions::new(deck_name, true, false),
            ),
            Vec::new(),
            None,
            None,
            None,
        )
    }

    fn get_deck_name(&'a self) -> &'a str {
        self.deck_name
    }
    fn get_operation(&'a self) -> NoteOperation {
        self.operation
    }

    fn get_markdown_id(&'a self) -> &'a String {
        &self.markdown_id
    }

    fn get_audio(&'a self) -> Option<&'a Vec<super::MediaFile<'a>>> {
        // TODO: impletment this method
        None
    }

    fn get_picture(&'a self) -> Option<&'a Vec<super::MediaFile<'a>>> {
        // TODO: impletment this method
        None
    }

    fn get_video(&'a self) -> Option<&'a Vec<super::MediaFile<'a>>> {
        // TODO: impletment this method
        None
    }
}
