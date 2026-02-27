#![allow(unused)]
use std::sync::LazyLock;

use console::{Emoji, style};
use derive_new::new;
use indicatif::ProgressStyle;

pub static LOOKING_GLASS: Emoji<'_, '_> = Emoji("🔍  ", "");
pub static TRUCK: Emoji<'_, '_> = Emoji("🚚  ", "");
pub static CLIP: Emoji<'_, '_> = Emoji("🔗  ", "");
pub static PAPER: Emoji<'_, '_> = Emoji("📃  ", "");
pub static SPARKLE: Emoji<'_, '_> = Emoji("✨ ", ":-)");
pub static SYNC: Emoji<'_, '_> = Emoji("🔄 ", "");
pub static PLUS: Emoji<'_, '_> = Emoji("➕ ", "+");
pub static SHUFFLE: Emoji<'_, '_> = Emoji("🔀 ", "");
pub static CROSS: Emoji<'_, '_> = Emoji("❌ ", "X");
pub static RECYCLE: Emoji<'_, '_> = Emoji("♻️ ", "X");
pub static REPEAT: Emoji<'_, '_> = Emoji("⟳  ", "X");
pub static BAR_CHART: Emoji<'_, '_> = Emoji("📊  ", "X");

pub static SPINNER_STYLE: LazyLock<ProgressStyle> = LazyLock::new(|| {
    #[allow(clippy::expect_used)]
    ProgressStyle::with_template("{prefix:.bold.dim} {spinner} {wide_msg}")
        .expect("premade progress style should not panic")
        .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
});

#[derive(Debug, new)]
pub struct Step {
    step_num: usize,
    step_max: usize,
}

impl Step {
    pub fn print_step(&mut self, message: Option<&str>, emoji: Option<Emoji<'_, '_>>) {
        println!(
            "{0} {1}{2}",
            style(format!("[{}/{}]", self.step_num, self.step_max))
                .bold()
                .dim(),
            emoji.unwrap_or(Emoji("", "")),
            message.unwrap_or("")
        );
        self.step_num += 1;
    }
}

pub fn print_step(
    step_num: usize,
    step_max: usize,
    message: Option<&str>,
    emoji: Option<Emoji<'_, '_>>,
) {
    println!(
        "{0} {1}{2}",
        style(format!("[{step_num}/{step_max}]")).bold().dim(),
        emoji.unwrap_or(Emoji("", "")),
        message.unwrap_or("")
    );
}
