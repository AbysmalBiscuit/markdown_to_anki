use jwalk::WalkDir;
use std::path::PathBuf;

use indicatif::ProgressBar;

pub fn find_markdown_files(input_dir: &PathBuf) -> Result<Vec<PathBuf>, jwalk::Error> {
    let pb = ProgressBar::new_spinner();
    WalkDir::new(input_dir)
        .into_iter()
        .map(|entry| {
            pb.tick();
            entry.map(|e| e.path())
        })
        .filter(|result| {
            result
                .as_ref()
                .is_ok_and(|path| path.extension().is_some_and(|ext| ext == "md"))
        })
        .collect()
}
