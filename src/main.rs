#![allow(unused)]
mod anki;
mod callout;
mod cli;
mod deck;
mod error;
mod find_markdown_files;
mod model;
mod progress;
mod utils;
use crate::find_markdown_files::find_markdown_files;
use callout::Callout;
use cli::cli;
use error::GenericError;
use progress::{LOOKING_GLASS, print_step};
use rayon::prelude::*;
use std::{fs::File, io::Write, path::PathBuf};
use tracing::info;
use tracing::instrument;
use tracing::warn;
use tracing_indicatif::IndicatifLayer;
use tracing_subscriber::FmtSubscriber;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

fn create_markdown_anki_cards_file(
    input_dir: &PathBuf,
    output_file_path: PathBuf,
) -> Result<(), GenericError> {
    let max_step = 10;
    print_step(
        1,
        max_step,
        Some("Finding markdown files"),
        Some(LOOKING_GLASS),
    );
    let markdown_files = find_markdown_files(input_dir)?;

    if markdown_files.is_empty() {
        warn!(
            "Failed to find any markdown files in: '{}'",
            input_dir.to_str().unwrap()
        );
        return Ok(());
    }

    info!("Found {} markdown files.", &markdown_files.len());
    print_step(2, max_step, Some("Converting callouts"), None);

    let callouts: Vec<Callout> = markdown_files
        .par_iter()
        .map(|path| Callout::extract_callouts(path).unwrap())
        .flatten()
        .collect();

    let num_callouts = &callouts.len();

    info!("Found {} callouts", num_callouts);

    let mut output_file = File::create(&output_file_path)?;

    let content = callouts
        .par_iter()
        .map(|callout| callout.to_anki_markdown_entry(None))
        .collect::<Vec<_>>()
        .join("\n\n");

    output_file.write_all(content.as_bytes())?;

    info!(
        "Wrote {} callouts to '{}'",
        &num_callouts,
        output_file_path.to_str().unwrap()
    );

    Ok(())
}

fn main() -> Result<(), GenericError> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO) // Or your desired level
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set global default subscriber");

    tracing::info!("Hello from tracing!");
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
            let model_type: String = sub_matches
                .get_one::<String>("model_type")
                .unwrap()
                .to_string();
            let model_name: String = sub_matches
                .get_one::<String>("model_name")
                .cloned()
                .unwrap_or_else(|| format!("md2anki {}", &model_type));
            let parent_deck: String = sub_matches.get_one::<String>("deck").unwrap().to_string();
            let header_lang: Option<&str> = sub_matches
                .get_one::<String>("header_lang")
                .map(|value| value.as_str());
            anki::sync::sync(&input_dir, parent_deck, model_type, model_name, header_lang)?;
        }
        _ => unreachable!(),
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use std::env;

    use crate::callout::Callout;

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
