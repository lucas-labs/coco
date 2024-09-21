use {
    cc_core::{
        config::Theme,
        state::{commit::ConventionalCommit, MutexAppState, StepStatus},
    },
    eyre::Result,
    lool::tui::{
        ratatui::{
            layout::{Alignment, Constraint, Direction, Flex, Layout, Rect},
            style::Stylize,
            widgets::{Block, Padding, Paragraph},
            Color, Line, Span, Style,
        },
        Component, Frame,
    },
    tokio::sync::mpsc::UnboundedSender,
    tui::widgets::CommitMessage,
};

pub struct PreviewStep {
    // children: Children,
    _theme: Theme,
    app_state: MutexAppState,
    active: bool,
    sender: Option<UnboundedSender<String>>,
    decision: bool,
}

impl PreviewStep {
    pub fn new(theme: Theme, app_state: MutexAppState) -> Self {
        Self {
            _theme: theme.clone(),
            app_state: app_state.clone(),
            active: false,
            decision: true,
            sender: None,
        }
    }

    fn send(&self, action: &str) {
        if let Some(sender) = self.sender.as_ref() {
            let _ = sender.send(action.to_string());
        }
    }

    fn set_step_status(&self, step: &str, status: StepStatus) {
        let mut state = self.app_state.lock().unwrap();
        state.set_step_status(step, status);
    }

    fn get_layout(&self, wrapper: Rect, commit: &ConventionalCommit) -> [Rect; 3] {
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
    fn register_action_handler(&mut self, tx: UnboundedSender<String>) -> Result<()> {
        self.sender = Some(tx.clone());
        Ok(())
    }

    fn receive_message(&mut self, message: String) -> Result<()> {
        if self.is_active() {
            match message.as_str() {
                "kb:enter" | "kb:pagedown" => {
                    if self.decision {
                        self.set_step_status("preview", StepStatus::Valid);
                        self.send("builder:confirmed")
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

        Ok(())
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn is_active(&self) -> bool {
        self.active
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
        let commit = { self.app_state.lock().unwrap().get_commit() };

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

        Ok(())
    }
}
