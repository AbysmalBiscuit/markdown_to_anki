mod callout;
use callout::Callout;
use jwalk::WalkDir;
use rayon::prelude::*;
use std::{env, error::Error, fs::read_to_string, io, path::Path};

// #[macro_use]
// extern crate hyperscan;

fn process_file(path: &Path) -> Result<Vec<Callout>, Box<dyn Error + Send + Sync>> {
    // let c =
    let content = read_to_string(path)?;
    // TODO: single thread: split file into callout blocks
    let binding: Vec<_> = content
        .split("\n> [!")
        .skip(1)
        .filter(|block| !block.trim().is_empty())
        .map(|block| format!("> [!{}", block))
        .collect::<Vec<String>>();

    // TODO: parallel: parse all callout blocks into a callout structs and convert the structs into
    // anki card text
    let vals: Vec<_> = binding
        // .par_iter()
        .iter()
        .map(|block| {
            block
                .par_split('\n')
                .filter(|line| line.starts_with('>'))
                .collect::<Vec<_>>()
        })
        .map(Callout::try_from)
        .collect();
    dbg!(&vals);
    let v = match &vals[0] {
        Ok(value) => value,
        Err(err) => panic!("{}", err),
    };
    dbg!(&v.callout_type, &v.header, &v.content, &v.sub_callouts);

    // TODO: merge callouts text into single string
    Ok(Vec::new())
}

fn main() -> io::Result<()> {
    let current_dir = env::current_dir()?;
    dbg!(&current_dir);
    // let args: Vec<String> = env::args().collect();
    // dbg!(args);
    // let target_dir = "/mnt/c/Users/Lev/Obsidian/Vault/Languages/한국어";
    let target_dir = current_dir;
    // dbg!(target_dir);
    let markdown_files: Vec<_> = WalkDir::new(target_dir)
        .into_iter()
        .map(|entry| entry.unwrap().path())
        .filter(|path| {
            path.extension().is_some_and(|ext| ext == "md")
            // TODO: remove name check for final version
                && path.file_name().is_some_and(|name| name.eq("words.md"))
        })
        .collect();
    dbg!(&markdown_files);
    markdown_files
        .par_iter()
        .map(|path| process_file(path))
        .collect::<Vec<_>>();

    // TODO: stream processed text into file

    Ok(())
}

#[cfg(test)]
mod test {
    use std::env;

    use crate::process_file;

    #[test]
    fn test_kr_words() {
        let current_dir = match env::current_dir() {
            Ok(value) => value,
            Err(err) => panic!("{}", err),
        };
        let path = current_dir.join("words.md");
        let callouts = match process_file(&path) {
            Ok(value) => value,
            Err(err) => panic!("{}", err),
        };
        assert_eq!(49, callouts.len());
    }
}
