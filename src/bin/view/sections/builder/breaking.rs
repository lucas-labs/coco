use coco::{
    core::{
        config::Theme,
        state::{MutexAppState, StepStatus},
    },
    t,
    tui::widgets::CocoHeader,
};
use matetui::{
    component,
    ratatui::{
        layout::{Constraint, Direction, Flex, Layout, Rect},
        prelude::{Line, Stylize},
        text::Span,
        widgets::Paragraph,
    },
    widgets::switch::Switch,
    Component, ComponentAccessors, Frame,
};

component! {
    pub struct BreakingChangeStep {
        theme: Theme,
        app_state: MutexAppState,
        breaking_change_choice: bool,
    }
}

impl BreakingChangeStep {
    pub fn new(theme: Theme, app_state: MutexAppState) -> Self {
        Self {
            breaking_change_choice: false,
            app_state: app_state.clone(),
            theme: theme.clone(),
            ..Default::default()
        }
    }

    fn set_step_status(&self, step: &str, status: StepStatus) {
        let mut state = self.app_state.lock().unwrap();
        state.set_breaking(self.breaking_change_choice);
        state.set_step_status(step, status);
    }

    fn toggle_breaking_change(&mut self) {
        self.breaking_change_choice = !self.breaking_change_choice;
    }

    fn set_breaking_change(&mut self, choice: bool) {
        self.breaking_change_choice = choice;
    }

    /// Get the main layout
    fn layout(&self, area: Rect) -> [Rect; 2] {
        Layout::vertical([Constraint::Length(2), Constraint::Fill(1)]).areas(area)
    }

    fn get_body_layout(&self, area: Rect) -> [Rect; 2] {
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
        let [header_area, area] = self.layout(area);
        let header = CocoHeader::default()
            .left_fg(self.theme.get("logo:fg:1"))
            .right_fg(self.theme.get("logo:fg:2"));

        let [title_area, rest_area] = self.get_body_layout(area);
        let switch = Switch::with_status(self.breaking_change_choice)
            .with_color_on(self.theme.get("switch:on"))
            .with_color_switch(self.theme.get("switch:switch"))
            .with_color_off(self.theme.get("switch:off"));

        let line = Line::from(vec![
            t!("Does this commit introduces a breaking change?").into(),
            " (".into(),
            // Yes / No depending on the choice
            if self.breaking_change_choice {
                let yes: Span = t!("Yes").into();
                yes.bold().fg(self.theme.get("yes"))
            } else {
                let no: Span = t!("No").into();
                no.bold().dim()
            },
            ")".into(),
        ]);

        f.render_widget(header, header_area);
        f.render_widget(Paragraph::new(line).centered(), title_area);
        f.render_widget(switch, rest_area);
    }
}
