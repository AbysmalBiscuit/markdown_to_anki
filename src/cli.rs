use std::path::PathBuf;

use clap::{ArgAction, Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "md2anki")]
#[command(about="Convert markdown callout notes to Anki flashcards", long_about = None)]
pub struct Cli {
    /// Verbose, can be passed multiple times to increase verbosity
    #[arg(short, long, action = ArgAction::Count, global=true)]
    pub verbose: u8,

    /// Quiet, can be passed multiple times to decrease verbosity
    #[arg(short, long, action = ArgAction::Count, global=true)]
    pub quiet: u8,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Convert notes to the format used by ObsidianToAnki
    #[command(arg_required_else_help = true)]
    ObsidianToAnki {
        /// Input directory used to search for notes
        input_dir: PathBuf,

        /// Output file, path to output file, if not specified, then a file will be created inside
        /// the input directory
        output_file: Option<PathBuf>,
    },
    /// Synchronize notes with Anki using AnkiConnect
    #[command(arg_required_else_help = true)]
    Sync(SyncArgs),
}

#[derive(Args, Debug)]
pub struct SyncArgs {
    /// Delete existing notes in deck before syncing
    #[arg(long = "delete")]
    pub delete_existing: bool,

    /// Name of deck to which cards should be added
    #[arg(short, long)]
    pub deck: Option<String>,

    /// The type of model to use among Basic, Word, Rule
    #[arg(short, long = "model", value_parser=["Basic", "Rule", "Word"], default_value = "Basic")]
    pub model_type_name: Option<String>,

    /// Name of the card model that should be used for the cards
    #[arg(long)]
    pub model_name: Option<String>,

    /// Path to css file containing card style rules. This is only needed if a new model needs
    /// to be created. If a model exists, the model style will be updated using the given CSS
    /// file.
    #[arg(short, long = "css")]
    pub css_file: Option<PathBuf>,

    /// 2 letter language code (ISO 639-1) to use for callout names.
    /// Falls back to English (en) if not specified or not supported.
    #[arg(short = 'l', long = "lang", default_value = "en")]
    pub header_lang: Option<String>,

    /// Input path used to search for notes
    // #[arg()]
    pub input_dir: PathBuf,
}
