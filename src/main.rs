#![allow(unused)]
mod anki_connect;
mod callout;
mod cli;
mod deck;
mod error;
mod note;
use callout::callout::Callout;
use cli::cli;
use jwalk::WalkDir;
use rayon::prelude::*;
use std::{
    fs::File,
    io::{self, Write},
    path::PathBuf,
};

fn create_markdown_anki_cards_file(
    input_dir: &PathBuf,
    output_file_path: PathBuf,
) -> io::Result<()> {
    let markdown_files: Vec<PathBuf> = WalkDir::new(input_dir)
        .into_iter()
        .map(|entry| entry.unwrap().path())
        .filter(|path| {
            path.extension().is_some_and(|ext| ext == "md") && path.ne(output_file_path.as_os_str())
        })
        .collect::<Vec<_>>();

    let callouts: Vec<Callout> = markdown_files
        .par_iter()
        .map(|path| Callout::extract_callouts(path).unwrap())
        .flatten()
        .collect();

    let mut output_file = File::create(output_file_path)?;

    let content = callouts
        .par_iter()
        .map(|callout| callout.to_anki_markdown_entry(None))
        .collect::<Vec<_>>()
        .join("\n\n");

    output_file.write_all(content.as_bytes())?;

    Ok(())
}

fn main() -> io::Result<()> {
    let matches = cli().get_matches();
    match matches.subcommand() {
        Some(("markdown", sub_matches)) => {
            let input_dir: PathBuf = sub_matches
                .get_one::<PathBuf>("input")
                .unwrap()
                .to_path_buf();
            let output_file_path: PathBuf = sub_matches
                .get_one::<PathBuf>("output_file")
                .map_or_else(|| input_dir.join("Anki cards.md"), |p| p.to_path_buf());
            create_markdown_anki_cards_file(&input_dir, output_file_path)?
        }
        Some(("sync", sub_matches)) => {
            let input_dir: PathBuf = sub_matches
                .get_one::<PathBuf>("input")
                .unwrap()
                .to_path_buf();
            let note_type: String = sub_matches
                .get_one::<String>("note_type")
                .unwrap()
                .to_string();
            let parent_deck: String = sub_matches.get_one::<String>("deck").unwrap().to_string();
            let _ = anki_connect::sync(&input_dir, parent_deck);
        }
        _ => unreachable!(),
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use std::env;

    use crate::callout::callout::Callout;

    #[test]
    fn test_kr_words() {
        let current_dir = match env::current_dir() {
            Ok(value) => value,
            Err(err) => panic!("{}", err),
        };
        let path = current_dir.join("demo/words.md");
        let callouts = match Callout::extract_callouts(&path) {
            Ok(value) => value,
            Err(err) => panic!("{}", err),
        };
        assert_eq!(49, callouts.len());
    }
}
