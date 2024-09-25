use {
    cc_core::config::Theme,
    matetui::{component, ratatui::layout::Rect, Component, Frame},
    tui::{help, widgets::coco_help::CocoHelp},
};

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

// "; observation => "Press 'q' or 'Ctrl+c' to quit the application",
// "Quit the application": ["q", "Ctrl+c"]; observation => "Press 'q' or 'Ctrl+c' to quit the application",
// "Navigate up": ["k", "Up arrow"],
// "Navigate down": ["j", "Down arrow"],
// "Navigate left": ["h", "Left arrow"],

impl Component for HelpSection {
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) {
        let sections = help! {
            "Help" => {
                "Toggle this help on and off": ["F2"],
            }
            "General" => {
                "Exit": ["ctrl+c"],
                "Go to the next step": ["pg-dn"],
                "Go to the previous step": ["pg-up"],
            }
            "Text Areas" => {
                "Submit": ["enter"],
                "New Line / carriage return": ["shift+enter", "alt+enter", "ctrl+enter"]; observation => "depends on the terminal",
            }
            "Switch" => {
                "Toggle the switch": ["space"],
                "Set the switch to true": ["🠆"],
                "Set the switch to false": ["🠄"],
                "Accept": ["enter"],
            }
        };

        let help = CocoHelp::new(sections);
        f.render_widget(help, area);
    }
}
