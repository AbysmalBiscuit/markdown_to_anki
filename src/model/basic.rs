use ankiconnect_rs::Field;

use crate::{callout::Callout, utils::capitalize_first_letter};

use super::traits::{AddNote, CreateModel, FromCallout, InternalModel};

#[derive(Debug)]
pub struct Basic {
    pub front: String,
    pub back: String,
}

impl FromCallout for Basic {
    fn from_callout(callout: &Callout, header_lang: Option<&str>) -> Self {
        Basic {
            front: callout.header.clone(),
            back: callout.content_to_html(header_lang),
        }
    }
}

impl AddNote for Basic {
    fn add_note(
        &self,
        deck_name: &ankiconnect_rs::Deck,
        model: &ankiconnect_rs::Model,
        note: ankiconnect_rs::Note,
        allow_duplicate: bool,
        duplicate_scope: Option<ankiconnect_rs::DuplicateScope>,
    ) -> ankiconnect_rs::Result<ankiconnect_rs::NoteId> {
        todo!()
    }
}

impl CreateModel for Basic {
    fn create_model(
        &self,
        client: &ankiconnect_rs::AnkiClient,
        css: &str,
    ) -> ankiconnect_rs::Result<ankiconnect_rs::models::ModelId> {
        let front = capitalize_first_letter(&self.front);
        let front = front.as_str();
        let back = capitalize_first_letter(&self.front);
        let back = back.as_str();
        let templates = [
            (
                "Recognition",
                r#"<br>
<div class="center">{{Front}}</div>
<br>
<div class="center">{{tts ko_KR voices=com.samsung.SMT-ko-KR-SMTg01,Microsoft_Heami:Front}}</div>"#,
                r#"{{FrontSide}}

<hr id=answer>

<div class="center">{{Back}}</div>
<br>
<!--{{tts ko_KR voices=com.samsung.SMT-ko-KR-SMTg01,Microsoft_Heami:Front}}-->"#,
            ),
            (
                "Recall",
                r#"<br>
<div class="center">{{Back}}</div>
<br>
<div class="center">{{tts ko_KR voices=com.samsung.SMT-ko-KR-SMTg01,Microsoft_Heami:Back}}</div>"#,
                r#"{{FrontSide}}

<hr id=answer>

<div class="center">{{Front}}</div>
<div class="center">{{tts ko_KR voices=com.samsung.SMT-ko-KR-SMTg01,Microsoft_Heami:Front}}</div>"#,
            ),
            (
                "Listen",
                r#"<br>
<div class="center">{{tts ko_KR voices=com.samsung.SMT-ko-KR-SMTg01,Microsoft_Heami:Front}}</div>"#,
                r#"
<hr id=answer>

<div class="center">{{Front}}</div>
<br>
<div class="center">{{tts ko_KR voices=com.samsung.SMT-ko-KR-SMTg01,Microsoft_Heami:Front}}</div>
<div class="center">{{Back}}</div>"#,
            ),
        ];
        client
            .models()
            .create_model("md2anki Basic", &[front, back], css, &templates)
    }
}

impl Default for Basic {
    fn default() -> Self {
        Basic {
            front: "".into(),
            back: "".into(),
        }
    }
}

impl InternalModel for Basic {}
