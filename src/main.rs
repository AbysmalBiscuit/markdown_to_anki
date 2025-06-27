// #![allow(unused)]
#[cfg(all(feature = "ureq_blocking", feature = "reqwest_blocking"))]
compile_error!("Only one of `ureq_blocking` or `reqwest_blocking` features can be enabled.");
mod anki_connect;
mod callout;
mod cli;
mod commands;
mod deck;
mod error;
mod find_markdown_files;
mod macros;
mod model;
mod note_operation;
mod progress;

use crate::callout::Callout;
use crate::cli::{Cli, Commands};
use crate::commands::{create_markdown_anki_cards_file, sync};
use crate::error::M2AnkiError;
use crate::find_markdown_files::find_markdown_files;

use std::path::PathBuf;
use tracing::error;
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
        // sync_args has a .run() sync.run() -> add a method to the struct
        // an option is to do the impl on the SyncArgs
        Commands::Sync(sync_args) => match sync(sync_args) {
            Ok(_) => (),
            Err(err) => error!("{:?}", err),
        },
        // _ => unreachable!(),
    }

    Ok(())
}
