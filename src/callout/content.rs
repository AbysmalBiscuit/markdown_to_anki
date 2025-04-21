use strum::Display;

#[derive(Display, Debug, Clone)]
pub enum CalloutContent {
    Text(String),
    SubCalloutIndex(usize),
}
