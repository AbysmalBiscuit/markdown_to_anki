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
                        .index(2)
                )

        )
    // .subcommand(
    //     Command::new("stash")
    //         .args_conflicts_with_subcommands(true)
    //         .flatten_help(true)
    //         .args(push_args())
    //         .subcommand(Command::new("push").args(push_args()))
    //         .subcommand(Command::new("pop").arg(arg!([STASH])))
    //         .subcommand(Command::new("apply").arg(arg!([STASH]))),
    // )
}

// fn push_args() -> Vec<clap::Arg> {
//     vec![arg!(-m --message <MESSAGE>)]
// }
