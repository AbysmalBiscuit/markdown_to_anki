use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct CreateModelParams<'a> {
    model_name: &'a str,
    in_order_fields: &'a [&'a str],
    css: &'a str,
    card_templates: &'a [HashMap<String, String>],
}

