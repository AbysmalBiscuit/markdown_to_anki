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
