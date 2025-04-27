#![allow(unused)]
use std::sync::LazyLock;

use console::{Emoji, style};
use indicatif::ProgressStyle;

pub static LOOKING_GLASS: Emoji<'_, '_> = Emoji("🔍  ", "");
pub static TRUCK: Emoji<'_, '_> = Emoji("🚚  ", "");
pub static CLIP: Emoji<'_, '_> = Emoji("🔗  ", "");
pub static PAPER: Emoji<'_, '_> = Emoji("📃  ", "");
pub static SPARKLE: Emoji<'_, '_> = Emoji("✨ ", ":-)");

pub static SPINNER_STYLE: LazyLock<ProgressStyle> = LazyLock::new(|| {
    ProgressStyle::with_template("{prefix:.bold.dim} {spinner} {wide_msg}")
        .unwrap()
        .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
});

pub fn print_step(
    step_num: usize,
    step_max: usize,
    message: Option<&str>,
    emoji: Option<Emoji<'_, '_>>,
) {
    println!(
        "{0} {1}{2}",
        style(format!("[{}/{}]", step_num, step_max)).bold().dim(),
        emoji.unwrap_or(Emoji("", "")),
        message.unwrap_or("")
    );
}
