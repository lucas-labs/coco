use {
    cc_core::{
        config::Theme,
        state::{commit::Commit, MutexAppState},
        t,
    },
    matetui::{
        component,
        ratatui::{
            layout::{Constraint, Flex, Layout, Rect},
            style::{Style, Stylize},
            text::{Line, Text},
            widgets::{Block, Borders, Padding, Paragraph},
        },
        Action, Component, ComponentAccessors, Frame,
    },
};

component! {
    pub struct SummarySection {
        theme: Theme,
        app_state: MutexAppState,
        commit: Option<Commit>
    }
}

impl SummarySection {
    pub fn new(theme: Theme, app_state: MutexAppState) -> Self {
        Self {
            theme,
            app_state,
            ..Default::default()
        }
    }

    pub fn get_layout(&self, area: Rect, text_height: usize) -> [Rect; 2] {
        let [commit, _, footer] = Layout::vertical([
            Constraint::Length(text_height as u16),
            Constraint::Length(1),
            Constraint::Fill(1),
        ])
        .areas(area);

        let [footer_area] =
            Layout::horizontal([Constraint::Percentage(50)]).flex(Flex::Center).areas(footer);

        [commit, footer_area]
    }
}

impl Component for SummarySection {
    fn receive_message(&mut self, message: String) {
        match message.as_str() {
            "committing:done" => {
                let state = self.app_state.lock().unwrap();
                self.commit = Some(state.get_commit());
            }
            "kb:enter" => self.send_action(Action::Quit),
            _ => {}
        }
    }

    fn handle_key_events(
        &mut self,
        _key: matetui::ratatui::crossterm::event::KeyEvent,
    ) -> Option<Action> {
        Some(Action::Quit)
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) {
        if let Some(commit) = &self.commit {
            let mut text = Text::from(vec![
                Line::from(format!("{} 🍻", t!("Done! This is your commit"))).bold(),
                "".into(),
            ]);

            let commit_text = commit.as_text();
            for line in commit_text {
                text.push_line(line.clone());
            }

            let [commit_area, footer_area] = self.get_layout(area, text.height());

            f.render_widget(Paragraph::new(text), commit_area);
            f.render_widget(
                Paragraph::new(t!("Press any key to quit...")).centered().block(
                    Block::default()
                        .borders(Borders::TOP)
                        .border_style(Style::default().dim())
                        .padding(Padding::top(1)),
                ),
                footer_area,
            );
        }
    }
}
