// #![allow(unused)]
mod anki;
mod callout;
mod cli;
mod client;
mod deck;
mod error;
mod find_markdown_files;
mod model;
mod obsidian_to_anki;
mod progress;
use crate::find_markdown_files::find_markdown_files;
use anki::sync::sync;
use callout::Callout;
use cli::{Cli, Commands};
use error::M2AnkiError;
use obsidian_to_anki::create_markdown_anki_cards_file;
use std::path::PathBuf;
use tracing_subscriber::FmtSubscriber;

use clap::Parser;

fn main() -> Result<(), M2AnkiError> {
    let args = Cli::parse();

    let verbosity = match args.verbose + 2 {
        0 => tracing::Level::ERROR,
        1 => tracing::Level::WARN,
        2 => tracing::Level::INFO,
        3 => tracing::Level::DEBUG,
        _ => tracing::Level::TRACE,
    };

    let subscriber = FmtSubscriber::builder()
        .with_max_level(verbosity) // Or your desired level
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set global default subscriber");

    match args.command {
        Commands::ObsidianToAnki {
            input_dir,
            output_file,
        } => {
            let output_file_path: PathBuf =
                output_file.map_or_else(|| input_dir.join("Anki cards.md"), |p| p.to_path_buf());
            create_markdown_anki_cards_file(&input_dir, output_file_path)?
        }
        Commands::Sync(sync_args) => sync(sync_args)?,
        // _ => unreachable!(),
    }

    Ok(())
}
