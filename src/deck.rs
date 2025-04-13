use std::path::PathBuf;

use crate::callout::callout::Callout;

#[derive(Debug)]
struct Deck {
    name: String,
    source_file: PathBuf,
    callouts: Vec<Callout>,
}

impl Deck {
    pub fn get_qualified_name(&self) -> String {
        self.name.clone()
    }
}
