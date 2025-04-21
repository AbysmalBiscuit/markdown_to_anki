use jwalk::WalkDir;
use std::io::Error as IOError;
use std::path::PathBuf;

use indicatif::{ParallelProgressIterator, ProgressBar, ProgressIterator, ProgressStyle};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::progress::SPINNER_STYLE;

pub fn find_markdown_files(input_dir: &PathBuf) -> Result<Vec<PathBuf>, IOError> {
    let pb = ProgressBar::new_spinner();
    Ok(WalkDir::new(input_dir)
        .into_iter()
        .map(|entry| {
            pb.tick();
            entry.unwrap().path()
        })
        .filter(|path| path.extension().is_some_and(|ext| ext == "md"))
        .collect::<Vec<_>>())
}
