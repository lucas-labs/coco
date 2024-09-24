use cc_core::config::Theme;
use matetui::{component, ratatui::layout::Rect, Component, Frame};
use tui::help;
use tui::widgets::coco_help::CocoHelp;

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
        // TODO: Real help
        //       for now, the key glossary is just a placeholder with fictitious data; should be
        //       replaced with actual key bindings once all of them are implemented
        let sections = help! {
            "General" => {
                "Quit the application": ["q", "Ctrl+c"]; observation => "Press 'q' or 'Ctrl+c' to quit the application",
                "Navigate up": ["k", "Up arrow"],
                "Navigate down": ["j", "Down arrow"],
                "Navigate left": ["h", "Left arrow"],
            }
            "Switches" => {
                "Toggle switch": ["space"],
                "Switch to ON": ["Y"],
                "Switch to OFF": ["N"],
            }
        };

        let help = CocoHelp::new(sections);
        f.render_widget(help, area);
    }
}
