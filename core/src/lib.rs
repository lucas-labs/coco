use {matetui::ratatui::prelude::Color, std::str::FromStr};

init_i18n!();

pub mod config;
pub mod git;
pub mod i18n;
pub mod state;

/// Convert a hex color to a [Color] or return [Color::Reset] if the conversion fails.
pub fn color(col: &str) -> Color {
    Color::from_str(col).unwrap_or(Color::Reset)
}

pub use i18n::setup_locale;
pub use i18n::t;
