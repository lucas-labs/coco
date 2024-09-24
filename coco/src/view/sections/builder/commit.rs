use {
    cc_core::{config::Theme, state::MutexAppState},
    matetui::{
        component,
        ratatui::{
            crossterm::event::KeyEvent,
            layout::{Constraint, Direction, Layout, Rect},
            style::Color,
        },
        widgets::textarea::{validators::required_validator, Input},
        Action, Component, ComponentAccessors, Frame,
    },
    tui::widgets::{CocoHeader, LabeledTextArea, StatusHint},
};

#[derive(PartialEq, Default)]
enum InputType {
    #[default]
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

component! {
    pub struct CommitStep {
        _theme: Theme,
        app_state: MutexAppState,
        summary_input: LabeledTextArea<'static>,
        body_input: LabeledTextArea<'static>,
        footer_input: LabeledTextArea<'static>,
        active_input: InputType,
    }
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
            summary_input,
            body_input,
            footer_input,
            ..Default::default()
        }
    }

    fn navigate(&mut self, direction: NavigationDirection) {
        let result = match direction {
            NavigationDirection::Next => self.active_input.next(),
            NavigationDirection::Prev => self.active_input.prev(),
        };

        match result {
            NavigationResult::PrevStep => self.send("builder:prev"),
            NavigationResult::NextStep => {
                if self.are_all_inputs_valid() {
                    self.update_inputs_active_states(None);
                    self.send("builder:next");
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
    fn get_textareas_layout(&self, area: Rect) -> (Rect, Rect, Rect) {
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

    /// Get the main layout
    fn layout(&self, area: Rect) -> [Rect; 3] {
        Layout::vertical([Constraint::Length(2), Constraint::Length(2), Constraint::Fill(1)])
            .areas(area)
    }
}

impl Component for CommitStep {
    fn receive_message(&mut self, message: String) {
        if self.is_active() {
            match message.as_str() {
                "kb:enter" | "kb:pagedown" => self.navigate(NavigationDirection::Next),
                "kb:pageup" => self.navigate(NavigationDirection::Prev),
                _ => {}
            }
        }
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> Option<Action> {
        let input = Input::from(key);

        // pass the input to the active input
        match self.active_input {
            InputType::Summary => self.summary_input.input(input),
            InputType::Body => self.body_input.input(input),
            InputType::Footer => self.footer_input.input(input),
        };

        None
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) {
        let [header_area, title_area, area] = self.layout(area);

        let (kind, scope) = {
            let state = self.app_state.lock().unwrap();
            (state.get_kind(), state.get_scope())
        };

        // draw the header and title
        let header = CocoHeader::default().left_fg(Color::Blue).right_fg(Color::Magenta);
        let title = StatusHint::new(kind, scope);
        f.render_widget(header, header_area);
        f.render_widget(title, title_area);

        let (summary_area, body_area, footer_area) = self.get_textareas_layout(area);

        f.render_widget(&self.summary_input, summary_area);
        f.render_widget(&self.body_input, body_area);
        f.render_widget(&self.footer_input, footer_area);
    }
}
