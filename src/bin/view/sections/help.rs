use coco::{core::config::Theme, help, t, tui::widgets::coco_help::CocoHelp};
use matetui::{component, ratatui::layout::Rect, Component, Frame};

component! {
    pub struct HelpSection {
        theme: Theme,
    }
}

impl HelpSection {
    pub fn new(theme: Theme) -> Self {
        Self {
            theme,
            ..Default::default()
        }
    }
}

impl Component for HelpSection {
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) {
        let sections = help! {
            t!("Help") => {
                t!("Show or hide this help") => ["F2"],
            }
            t!("General") => {
                t!("Exit") => ["ctrl+c"],
                t!("Go to the next step") => ["pg-dn"],
                t!("Go to the previous step") => ["pg-up"],
            }
            t!("Text Areas") => {
                t!("Submit") => ["enter"],
                t!("New Line / carriage return") => ["shift+enter", "alt+enter", "ctrl+enter"]; observation => t!("depends on the terminal"),
            }
            t!("Switch") => {
                t!("Toggle the switch") => ["space"],
                t!("Set the switch to true") => ["🠆"],
                t!("Set the switch to false") => ["🠄"],
                t!("Accept") => ["enter"],
            }
        };

        let help = CocoHelp::new(sections);
        f.render_widget(help, area);
    }
}
