use std::collections::{HashMap, HashSet};

use ankiconnect_rs::{Field, Model, Note, NoteBuilder, NoteError};

use crate::{callout::Callout, error::GenericError, utils::capitalize_first_letter};

use super::traits::InternalModel;

#[derive(Debug)]
pub struct Basic {
    pub front: String,
    pub back: String,
}

impl InternalModel for Basic {
    fn from_callout(&self, callout: &Callout, header_lang: Option<&str>) -> Self {
        Basic {
            front: callout.header.clone(),
            back: callout.content_to_html(header_lang),
        }
    }

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

    fn to_note(self, model: Model) -> Result<Note, NoteError> {
        let mut field_values: HashMap<String, String> = HashMap::with_capacity(2);
        field_values.insert("Front".into(), self.front);
        field_values.insert("Back".into(), self.back);
        let mut tags: HashSet<String> = HashSet::with_capacity(1);
        tags.insert("md2anki".to_string());
        Note::new(model, field_values, tags, Vec::new())
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
