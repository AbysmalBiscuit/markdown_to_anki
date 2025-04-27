// use std::ffi::OsString;
use std::path::PathBuf;

use clap::{Arg, Command};

pub fn cli() -> Command {
    Command::new("md2anki")
        .about("A markdown to Anki flashcard converter")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .arg(
            Arg::new("verbosity")
                .long("verbose")
                .short('v')
                .help("enable verbose output")
                .action(clap::ArgAction::Count)
        )
        .subcommand(
            Command::new("markdown")
                .about("convert `word` and `rule` blocks to a format that can be imported with ObsidianToAnki")
                .arg_required_else_help(true)
                .arg(
                    Arg::new("input")
                        .help("directory to search for files")
                        .index(1)
                        .required(true)
                        .value_parser(clap::value_parser!(PathBuf))
                )
                .arg(
                    Arg::new("output_file")
                        .help("path to output file, if not specified, then a file ")
                        .index(2)
                        .required(false)
                ),

        )
        .subcommand(
            Command::new("sync")
                .about("sync cards to Anki")
                .arg_required_else_help(true)
                .arg(
                    Arg::new("input")
                        .help("directory to search for files")
                        .index(1)
                        .required(true)
                        .value_parser(clap::value_parser!(PathBuf))
                )
                .arg(
                    Arg::new("deck")
                    .help("name of deck to which cards should be added")
                    .long("deck")
                    .short('d')
                    .default_value("md2anki")
                )
                .arg(
                    Arg::new("model_type_name")
                        .help("the type of model to use among Basic, Word, Rule")
                        .long("model")
                        .short('m')
                        .default_value("Basic")
                )
                .arg(
                    Arg::new("model_name")
                        .help("name of the card model that should be used for the cards")
                        .long("model-name")
                        .default_value(None)
                )
                .arg(
                    Arg::new("css_file")
                        .help("path to css file containing card style rules")
                        .long("css")
                        .short('c')
                        .default_value(None)
                        .value_parser(clap::value_parser!(PathBuf))
                )
                .arg(
                    Arg::new("header_lang")
                        .help("2 letter language code (ISO 639-1) to use for callout names. Falls back to English if not specified or not supported.")
                        .long("lang")
                        .short('l')
                        .default_value("")
                )
        )
}
