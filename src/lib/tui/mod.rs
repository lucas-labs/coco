use std::str::FromStr;

use matetui::ratatui::prelude::Color;

pub mod components {
    mod logo;

    pub use logo::LogoComponent;
}

pub mod widgets {
    mod coco_header;
    pub mod coco_help;
    mod coco_logo;
    mod coco_textarea;
    mod commit_msg;
    mod status_hint;

    pub use {
        coco_header::CocoHeader,
        coco_logo::CocoLogo,
        coco_textarea::{LabeledTextArea, LabeledTextAreaTheme},
        commit_msg::CommitMessage,
        status_hint::StatusHint,
    };
}

/// Convert a hex color to a [Color] or return [Color::Reset] if the conversion fails.
pub fn color(col: &str) -> Color {
    Color::from_str(col).unwrap_or(Color::Reset)
}
