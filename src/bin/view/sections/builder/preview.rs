use coco::{
    core::{
        config::Theme,
        state::{commit::ConventionalCommitMessage, MutexAppState, StepStatus},
    },
    t,
    tui::widgets::{CocoHeader, CommitMessage},
};
use matetui::{
    component,
    ratatui::{
        layout::{Alignment, Constraint, Direction, Flex, Layout, Rect},
        prelude::{Line, Span, Style},
        style::Stylize,
        widgets::{Block, Padding, Paragraph},
    },
    Component, ComponentAccessors, Frame,
};

component! {
    pub struct PreviewStep {
        theme: Theme,
        app_state: MutexAppState,
        decision: bool,
    }
}

impl PreviewStep {
    pub fn new(theme: Theme, app_state: MutexAppState) -> Self {
        Self {
            theme: theme.clone(),
            app_state: app_state.clone(),
            decision: true,
            ..Default::default()
        }
    }

    fn set_step_status(&self, step: &str, status: StepStatus) {
        let mut state = self.app_state.lock().unwrap();
        state.set_step_status(step, status);
    }

    /// Get the main layout
    fn layout(&self, area: Rect) -> [Rect; 2] {
        Layout::vertical([Constraint::Length(2), Constraint::Fill(1)]).areas(area)
    }

    fn get_body_layout(&self, wrapper: Rect, commit: &ConventionalCommitMessage) -> [Rect; 3] {
        // the biggest width of the commit message texts
        let (width, height) = commit.size();

        let [commit_area_wrapper, _, description_area, decision_area] = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(height + 2), // commit display area
                Constraint::Length(2),          // separator
                Constraint::Length(2),          // description message
                Constraint::Min(1),             // Yes / No area (Decision area)
            ])
            .areas(wrapper);

        let [commit_area] = Layout::default()
            .direction(Direction::Horizontal)
            .flex(Flex::Center)
            .constraints([Constraint::Length(width + 4)])
            .areas(commit_area_wrapper);

        [commit_area, description_area, decision_area]
    }
}

impl Component for PreviewStep {
    fn receive_message(&mut self, message: String) {
        if self.is_active() {
            match message.as_str() {
                "kb:enter" | "kb:pagedown" => {
                    if self.decision {
                        self.set_step_status("preview", StepStatus::Valid);
                        self.send("builder:done")
                    } else {
                        self.send("builder:restart")
                    }
                }
                "kb:pageup" => self.send("builder:prev"),
                "kb:left" => self.decision = true,
                "kb:right" => self.decision = false,
                "kb:space" => self.decision = !self.decision,
                _ => {}
            }
        }
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) {
        let [header_area, area] = self.layout(area);
        let header = CocoHeader::default()
            .left_fg(self.theme.get("logo:fg:1"))
            .right_fg(self.theme.get("logo:fg:2"));

        let commit = { self.app_state.lock().unwrap().get_commit_message() };

        let [commit_area, description_area, decision_area] = self.get_body_layout(area, &commit);
        let block = Block::default().padding(Padding::symmetric(2, 1)).on_white();

        let description = Paragraph::new(t!("Do you wish to continue and execute the commit?"))
            .alignment(Alignment::Center);

        let yes_style = if self.decision {
            Style::default().fg(self.theme.get("yes")).bold()
        } else {
            Style::default()
        };

        let no_style = if !self.decision {
            Style::default().fg(self.theme.get("no")).bold()
        } else {
            Style::default()
        };

        let decision = Paragraph::new(Line::from(vec![
            Span::from(t!("Yes")).style(yes_style),
            Span::from("    /    ").dim(),
            Span::from(t!("No")).style(no_style),
        ]))
        .alignment(Alignment::Center);

        // render all
        f.render_widget(header, header_area);
        f.render_widget(CommitMessage::new(commit), block.inner(commit_area));
        f.render_widget(block, commit_area);
        f.render_widget(description, description_area);
        f.render_widget(decision, decision_area);
    }
}
