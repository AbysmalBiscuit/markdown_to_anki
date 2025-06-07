use std::{fs::File, io::Write, path::PathBuf};

use tracing::{info, warn};

use crate::{
    callout::{Callout, ExtractCalloutsResult},
    error::M2AnkiError,
    find_markdown_files,
    progress::{LOOKING_GLASS, print_step},
};

use rayon::prelude::*;

pub fn create_markdown_anki_cards_file(
    input_dir: &PathBuf,
    output_file_path: PathBuf,
) -> Result<(), M2AnkiError> {
    let max_step = 10;
    print_step(
        1,
        max_step,
        Some("Finding markdown files"),
        Some(LOOKING_GLASS),
    );
    let markdown_files = find_markdown_files(input_dir)?;

    if markdown_files.is_empty() {
        warn!(
            "Failed to find any markdown files in: '{}'",
            input_dir.to_str().unwrap()
        );
        return Ok(());
    }

    info!("Found {} markdown files.", &markdown_files.len());
    print_step(2, max_step, Some("Converting callouts"), None);

    let callouts_results: Vec<ExtractCalloutsResult> = markdown_files
        .par_iter()
        .map(|path| Callout::extract_callouts(path))
        // .flatten()
        .collect();

    let callouts: Vec<Callout> = callouts_results
        .into_par_iter()
        .map(|result| result.callouts)
        .flatten()
        .collect();

    let num_callouts = &callouts.len();

    info!("Found {} callouts", num_callouts);

    let mut output_file = File::create(&output_file_path)?;

    let content = callouts
        .par_iter()
        .map(|callout| callout.to_anki_markdown_entry(None))
        .collect::<Vec<_>>()
        .join("\n\n");

    output_file.write_all(content.as_bytes())?;

    info!(
        "Wrote {} callouts to '{}'",
        &num_callouts,
        output_file_path.to_str().unwrap()
    );

    Ok(())
}
