use {
    cc_core::{config::Theme, state::MutexAppState},
    eyre::Result,
    lool::{
        s,
        tui::{
            ratatui::{
                crossterm::event::KeyEvent,
                layout::{Constraint, Direction, Layout, Rect},
            },
            widgets::textarea::{validators::required_validator, Input},
            Action, Component, Frame,
        },
    },
    tokio::sync::mpsc::UnboundedSender,
    tui::widgets::LabeledTextArea,
};

#[derive(PartialEq)]
enum InputType {
    Summary,
    Body,
    Footer,
}

enum NavigationResult {
    PrevStep,
    Input(InputType),
    NextStep,
}

enum NavigationDirection {
    Next,
    Prev,
}

impl InputType {
    fn next(&self) -> NavigationResult {
        match self {
            Self::Summary => NavigationResult::Input(Self::Body),
            Self::Body => NavigationResult::Input(Self::Footer),
            Self::Footer => NavigationResult::NextStep,
        }
    }

    fn prev(&self) -> NavigationResult {
        match self {
            Self::Summary => NavigationResult::PrevStep,
            Self::Body => NavigationResult::Input(Self::Summary),
            Self::Footer => NavigationResult::Input(Self::Body),
        }
    }
}

pub struct CommitStep {
    // children: Children,
    _theme: Theme,
    app_state: MutexAppState,
    active: bool,
    sender: Option<UnboundedSender<String>>,
    summary_input: LabeledTextArea<'static>,
    body_input: LabeledTextArea<'static>,
    footer_input: LabeledTextArea<'static>,
    active_input: InputType,
}

impl CommitStep {
    pub fn new(theme: Theme, app_state: MutexAppState) -> Self {
        let summary_input = LabeledTextArea::default()
            .with_title("summary")
            .with_subtitle("* required")
            .with_single_line(true)
            .with_max_char_count(72)
            .with_validations([required_validator]);
        let body_input = LabeledTextArea::default()
            .with_title("body")
            .with_subtitle("optional")
            .with_active(false);
        let footer_input = LabeledTextArea::default()
            .with_title("footer")
            .with_subtitle("optional")
            .with_active(false);

        Self {
            active_input: InputType::Summary,
            _theme: theme.clone(),
            app_state: app_state.clone(),
            active: false,
            sender: None,
            summary_input,
            body_input,
            footer_input,
        }
    }

    fn send(&self, action: String) {
        if let Some(sender) = self.sender.as_ref() {
            let _ = sender.send(action);
        }
    }

    fn navigate(&mut self, direction: NavigationDirection) {
        let result = match direction {
            NavigationDirection::Next => self.active_input.next(),
            NavigationDirection::Prev => self.active_input.prev(),
        };

        match result {
            NavigationResult::PrevStep => self.send(s!("builder:prev")),
            NavigationResult::NextStep => {
                if self.are_all_inputs_valid() {
                    self.update_inputs_active_states(None);
                    self.send(s!("builder:next"));
                }
            }
            NavigationResult::Input(input) => self.update_inputs_active_states(Some(input)),
        }
    }

    fn is_active_input_valid(&self) -> bool {
        match self.active_input {
            InputType::Summary => self.summary_input.is_valid(),
            InputType::Body => self.body_input.is_valid(),
            InputType::Footer => self.footer_input.is_valid(),
        }
    }

    fn are_all_inputs_valid(&self) -> bool {
        self.summary_input.is_valid() && self.body_input.is_valid() && self.footer_input.is_valid()
    }

    fn update_inputs_active_states(&mut self, maybe_next_active: Option<InputType>) {
        // onlyif the active input is valid
        if !self.is_active_input_valid() {
            return;
        }

        if let Some(next_active) = maybe_next_active {
            self.active_input = next_active;
        }
        self.summary_input.set_active(self.active_input == InputType::Summary);
        self.body_input.set_active(self.active_input == InputType::Body);
        self.footer_input.set_active(self.active_input == InputType::Footer);
        self.set_app_state();
    }

    fn set_app_state(&mut self) {
        let mut app_state = self.app_state.lock().unwrap();
        app_state.set_summary(self.summary_input.text());
        app_state.set_body(self.body_input.lines());
        app_state.set_footer(self.footer_input.lines());
    }

    /// Calculate the layout for the commit steps, showing the summary, body, and footer inputs
    /// one below the other in a vertical layout.
    fn get_layout(&self, area: Rect) -> (Rect, Rect, Rect) {
        let areas = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(self.summary_input.get_height()),
                Constraint::Length(1),
                Constraint::Length(self.body_input.get_height()),
                Constraint::Length(1),
                Constraint::Length(self.footer_input.get_height()),
            ])
            .split(area);

        (areas[0], areas[2], areas[4])
    }
}

impl Component for CommitStep {
    fn register_action_handler(&mut self, tx: UnboundedSender<String>) -> Result<()> {
        self.sender = Some(tx.clone());
        Ok(())
    }

    fn receive_message(&mut self, message: String) -> Result<()> {
        if self.is_active() {
            match message.as_str() {
                "kb:enter" | "kb:pagedown" => self.navigate(NavigationDirection::Next),
                "kb:pageup" => self.navigate(NavigationDirection::Prev),
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

    fn handle_key_events(&mut self, key: KeyEvent) -> Result<Option<Action>> {
        let input = Input::from(key);

        // pass the input to the active input
        match self.active_input {
            InputType::Summary => self.summary_input.input(input),
            InputType::Body => self.body_input.input(input),
            InputType::Footer => self.footer_input.input(input),
        };

        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
        let (summary_area, body_area, footer_area) = self.get_layout(area);

        f.render_widget(&self.summary_input, summary_area);
        f.render_widget(&self.body_input, body_area);
        f.render_widget(&self.footer_input, footer_area);

        Ok(())
    }
}
