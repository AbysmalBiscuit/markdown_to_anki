use strum::Display;

#[derive(Display, Debug, Clone)]
pub enum CalloutContent {
    Text(String),
    UnorderedListStart,
    UnorderedListEnd,
    UnorderedListItem(String),
    OrderedListStart,
    OrderedListEnd,
    OrderedListItem(String),
    SubCalloutIndex(usize),
    Blockquote(String),
    HorizontalLine,
}

impl CalloutContent {
    pub fn to_html(&self) -> String {
        match &self {
            CalloutContent::Text(value) => value.to_string(),
            CalloutContent::UnorderedListItem(value) => value.to_string(),
            CalloutContent::OrderedListItem(_) => todo!(),
            CalloutContent::SubCalloutIndex(_) => todo!(),
            CalloutContent::Blockquote(_) => todo!(),
            CalloutContent::HorizontalLine => todo!(),
            CalloutContent::UnorderedListStart => todo!(),
            CalloutContent::UnorderedListEnd => todo!(),
            CalloutContent::OrderedListStart => todo!(),
            CalloutContent::OrderedListEnd => todo!(),
        }
    }
}
