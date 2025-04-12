mod callout;
mod cli;
use callout::Callout;
use cli::cli;
use jwalk::WalkDir;
use rayon::prelude::*;
use std::{
    error::Error,
    fs::{File, read_to_string},
    io::{self, Write},
    path::{Path, PathBuf},
};

fn extract_callouts(path: &Path) -> Result<Vec<Callout>, Box<dyn Error + Send + Sync>> {
    let content: String = read_to_string(path)?;
    let blocks: Vec<_> = content
        .split("\n> [!")
        .skip(1)
        .filter(|block| !block.trim().is_empty())
        .map(|block| format!("> [!{}", block))
        .collect::<Vec<String>>();

    let callouts: Vec<_> = blocks
        .par_iter()
        .map(|block| {
            block
                .par_split('\n')
                .filter(|line| line.starts_with('>'))
                .collect::<Vec<_>>()
        })
        .map(Callout::try_from)
        .map(|callout| callout.unwrap())
        .collect();

    Ok(callouts)
}

fn create_anki_cards_file(input_dir: PathBuf, output_file_path: PathBuf) -> io::Result<()> {
    // dbg!(&input_dir, &output_file_path);
    let markdown_files: Vec<_> = WalkDir::new(input_dir)
        .into_iter()
        .map(|entry| entry.unwrap().path())
        .filter(|path| {
            path.extension().is_some_and(|ext| ext == "md")
            // TODO: remove name check for final version
            && path.file_name().is_some_and(|name| name.eq("rules.md"))
            && path.ne(output_file_path.as_os_str())
        })
        .collect();
    // dbg!(&markdown_files);
    let callouts: Vec<Callout> = markdown_files
        .par_iter()
        .map(|path| extract_callouts(path).unwrap())
        .flatten()
        .collect();
    // dbg!(&callouts);

    let mut output_file = File::create(output_file_path)?;
    let content = callouts
        .par_iter()
        .map(|callout| callout.to_anki_entry(None))
        .collect::<Vec<_>>()
        .join("\n\n");
    // dbg!(&content);
    output_file.write_all(content.as_bytes())?;
    Ok(())
}

fn main() -> io::Result<()> {
    let matches = cli().get_matches();
    match matches.subcommand() {
        Some(("convert", sub_matches)) => {
            let paths = sub_matches
                .get_many::<PathBuf>("PATH")
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();
            let target_dir = paths.first().unwrap().to_path_buf();
            let output_file_path = paths
                .get(1)
                .map_or_else(|| target_dir.join("Anki cards.md"), |p| p.to_path_buf());
            create_anki_cards_file(target_dir, output_file_path)?
        }
        _ => unreachable!(),
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use std::env;

    use crate::extract_callouts;

    #[test]
    fn test_kr_words() {
        let current_dir = match env::current_dir() {
            Ok(value) => value,
            Err(err) => panic!("{}", err),
        };
        let path = current_dir.join("words.md");
        let callouts = match extract_callouts(&path) {
            Ok(value) => value,
            Err(err) => panic!("{}", err),
        };
        assert_eq!(49, callouts.len());
    }
}
