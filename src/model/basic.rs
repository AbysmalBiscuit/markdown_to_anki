use crate::http::{CreateModelParams, RequestSender};
use std::collections::{HashMap, HashSet};

use ankiconnect_rs::{NoteError, models::ModelId};
use rayon::prelude::*;

use crate::{callout::Callout, http::HttpRequestSender};

use super::traits::InternalModelMethods;

#[derive(Debug, Default)]
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

    fn create_model(
        &self,
        client: &ankiconnect_rs::AnkiClient,
        css: &str,
    ) -> ankiconnect_rs::Result<ankiconnect_rs::models::ModelId> {
        let templates = [
            [
                ("Name", "Recognition"),
                (
                    "Front",
                    r#"<br>
<div class="center">{{Front}}</div>
<br>
<div class="center">{{tts ko_KR voices=com.samsung.SMT-ko-KR-SMTg01,Microsoft_Heami:Front}}</div>"#,
                ),
                (
                    "Back",
                    r#"{{FrontSide}}

<hr id=answer>

<div class="center">{{Back}}</div>
<br>
<!--{{tts ko_KR voices=com.samsung.SMT-ko-KR-SMTg01,Microsoft_Heami:Front}}-->"#,
                ),
            ],
            [
                ("Name", "Recall"),
                (
                    "Front",
                    r#"<br>
<div class="center">{{Back}}</div>
<br>
<div class="center">{{tts ko_KR voices=com.samsung.SMT-ko-KR-SMTg01,Microsoft_Heami:Back}}</div>"#,
                ),
                (
                    "Back",
                    r#"{{FrontSide}}

<hr id=answer>

<div class="center">{{Front}}</div>
<div class="center">{{tts ko_KR voices=com.samsung.SMT-ko-KR-SMTg01,Microsoft_Heami:Front}}</div>"#,
                ),
            ],
            [
                ("Name", "Listen"),
                (
                    "Front",
                    r#"<br>
<div class="center">{{tts ko_KR voices=com.samsung.SMT-ko-KR-SMTg01,Microsoft_Heami:Front}}</div>"#,
                ),
                (
                    "Back",
                    r#"
<hr id=answer>

<div class="center">{{Front}}</div>
<br>
<div class="center">{{tts ko_KR voices=com.samsung.SMT-ko-KR-SMTg01,Microsoft_Heami:Front}}</div>
<div class="center">{{Back}}</div>"#,
                ),
            ],
        ];
        let templates = templates
            .par_iter()
            .map(|template| {
                template
                    .iter()
                    .map(|(k, v)| (k.to_string(), v.to_string()))
                    .collect::<HashMap<String, String>>()
            })
            .collect::<Vec<_>>();
        let sender = HttpRequestSender::new("localhost", 8765);
        let params = CreateModelParams {
            model_name: "md2anki Basic",
            in_order_fields: &["MarkdownID", "Front", "Back"],
            css,
            card_templates: templates,
        };
        let id = sender.send::<_, u64>("createModel", Some(params))?;
        Ok(ModelId(id))
    }

    fn to_note(self, model: ankiconnect_rs::Model) -> Result<ankiconnect_rs::Note, NoteError> {
        let mut field_values: HashMap<String, String> = HashMap::with_capacity(2);
        field_values.insert("MarkdownID".into(), self.markdown_id);
        field_values.insert("Front".into(), self.front);
        field_values.insert("Back".into(), self.back);
        let mut tags: HashSet<String> = HashSet::with_capacity(1);
        tags.insert("md2anki".to_string());
        let media = Vec::new();
        ankiconnect_rs::Note::new(model, field_values, tags, media)
        // Ok(InternalNote::new(model, field_values, tags))
    }
}
