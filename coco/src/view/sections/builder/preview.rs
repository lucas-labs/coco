use {
    cc_core::{
        config::Theme,
        state::{commit::ConventionalCommitMessage, MutexAppState, StepStatus},
    },
    matetui::{
        component,
        ratatui::{
            layout::{Alignment, Constraint, Direction, Flex, Layout, Rect},
            prelude::{Color, Line, Span, Style},
            style::Stylize,
            widgets::{Block, Padding, Paragraph},
        },
        Component, ComponentAccessors, Frame,
    },
    tui::widgets::CommitMessage,
};

component! {
    pub struct PreviewStep {
        _theme: Theme,
        app_state: MutexAppState,
        decision: bool,
    }
}

impl PreviewStep {
    pub fn new(theme: Theme, app_state: MutexAppState) -> Self {
        Self {
            _theme: theme.clone(),
            app_state: app_state.clone(),
            decision: true,
            ..Default::default()
        }
    }

    fn set_step_status(&self, step: &str, status: StepStatus) {
        let mut state = self.app_state.lock().unwrap();
        state.set_step_status(step, status);
    }

    fn get_layout(&self, wrapper: Rect, commit: &ConventionalCommitMessage) -> [Rect; 3] {
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
        let commit = { self.app_state.lock().unwrap().get_commit_message() };

        let [commit_area, description_area, decision_area] = self.get_layout(area, &commit);
        let block = Block::default().padding(Padding::symmetric(2, 1)).on_white();

        let description = Paragraph::new("Do you wish to continue and execute the commit?")
            .alignment(Alignment::Center);

        let yes_style = if self.decision {
            Style::default().fg(Color::Green).bold()
        } else {
            Style::default().fg(Color::Reset)
        };

        let no_style = if !self.decision {
            Style::default().fg(Color::Red).bold()
        } else {
            Style::default().fg(Color::Reset)
        };

        let decision = Paragraph::new(Line::from(vec![
            Span::from("Yes").style(yes_style),
            Span::from("    /    ").dim(),
            Span::from("No").style(no_style),
        ]))
        .alignment(Alignment::Center);

        // render all
        f.render_widget(CommitMessage::new(commit), block.inner(commit_area));
        f.render_widget(block, commit_area);
        f.render_widget(description, description_area);
        f.render_widget(decision, decision_area);
    }
}
