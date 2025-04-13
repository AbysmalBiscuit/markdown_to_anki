// use std::ffi::OsString;
use std::path::PathBuf;

use clap::{Arg, Command};

pub fn cli() -> Command {
    Command::new("md2anki")
        .about("A markdown to Anki flashcard converter")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        // .subcommand(
        //     Command::new("convert")
        //         .about("Clones repos")
        //         .arg(arg!(<PATH> "The remote to clone"))
        //         .arg_required_else_help(true),
        // )
        // .subcommand(
        //     Command::new("diff")
        //         .about("Compare two commits")
        //         .arg(arg!(base: [COMMIT]))
        //         .arg(arg!(head: [COMMIT]))
        //         .arg(arg!(path: [PATH]).last(true))
        //         .arg(
        //             arg!(--color <WHEN>)
        //                 .value_parser(["always", "auto", "never"])
        //                 .num_args(0..=1)
        //                 .require_equals(true)
        //                 .default_value("auto")
        //                 .default_missing_value("always"),
        //         ),
        // )
        // .subcommand(
        //     Command::new("push")
        //         .about("pushes things")
        //         .arg(arg!(<REMOTE> "The remote to target"))
        //         .arg_required_else_help(true),
        // )
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
