use {
    cc_core::{
        config::Theme,
        state::{MutexAppState, StepStatus},
    },
    matetui::{
        component,
        ratatui::{
            layout::{Constraint, Direction, Flex, Layout, Rect},
            prelude::{Color, Line, Stylize},
            widgets::Paragraph,
        },
        widgets::switch::Switch,
        Component, ComponentAccessors, Frame,
    },
};

component! {
    pub struct BreakingChangeStep {
        _theme: Theme,
        app_state: MutexAppState,
        breaking_change_choice: bool,
    }
}

impl BreakingChangeStep {
    pub fn new(theme: Theme, app_state: MutexAppState) -> Self {
        Self {
            breaking_change_choice: false,
            app_state: app_state.clone(),
            _theme: theme.clone(),
            ..Default::default()
        }
    }

    fn set_step_status(&self, step: &str, status: StepStatus) {
        let mut state = self.app_state.lock().unwrap();
        state.set_breaking(self.breaking_change_choice);
        state.set_step_status(step, status);
    }

    fn get_layout(&self, area: Rect) -> [Rect; 2] {
        let [title, rest] = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(2), Constraint::Min(0)])
            .areas(area);

        // center the switch horizontally inside the rest area
        let [switch] = Layout::default()
            .direction(Direction::Horizontal)
            .flex(Flex::Center)
            .constraints([Constraint::Max(14)])
            .areas(rest);

        [title, switch]
    }

    fn toggle_breaking_change(&mut self) {
        self.breaking_change_choice = !self.breaking_change_choice;
    }

    fn set_breaking_change(&mut self, choice: bool) {
        self.breaking_change_choice = choice;
    }
}

impl Component for BreakingChangeStep {
    fn receive_message(&mut self, message: String) {
        if self.is_active() {
            match message.as_str() {
                "kb:enter" | "kb:pagedown" => {
                    self.set_step_status("breaking-change", StepStatus::Valid);
                    self.send("builder:next")
                }
                "kb:pageup" => self.send("builder:prev"),
                "kb:left" => self.set_breaking_change(false),
                "kb:right" => self.set_breaking_change(true),
                "kb:space" => self.toggle_breaking_change(),
                _ => {}
            }
        }
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) {
        let [title_area, rest_area] = self.get_layout(area);
        let switch = Switch::with_status(self.breaking_change_choice)
            .with_color_on(Color::Green)
            .with_color_switch(Color::White)
            .with_color_off(Color::Black);

        let line = Line::from(vec![
            "Does this commit introduces a breaking change? (".into(),
            // Yes / No depending on the choice
            if self.breaking_change_choice {
                "Yes".bold().fg(Color::Green)
            } else {
                "No".bold().dim()
            },
            ")".into(),
        ]);

        f.render_widget(Paragraph::new(line).centered(), title_area);
        f.render_widget(switch, rest_area);
    }
}
