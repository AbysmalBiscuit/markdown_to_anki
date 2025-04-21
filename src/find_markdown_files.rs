use jwalk::WalkDir;
use std::io::Error as IOError;
use std::path::PathBuf;

pub fn find_markdown_files(input_dir: &PathBuf) -> Result<Vec<PathBuf>, IOError> {
    Ok(WalkDir::new(input_dir)
        .into_iter()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "md"))
        .collect::<Vec<_>>())
}
