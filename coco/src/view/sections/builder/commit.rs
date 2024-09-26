use {
    super::navigation::commit_step::{InputType, NavigationDirection, NavigationResult},
    cc_core::{
        config::{CocoConfig, Theme},
        state::MutexAppState,
        t,
    },
    matetui::{
        component,
        ratatui::{
            crossterm::event::KeyEvent,
            layout::{Constraint, Direction, Layout, Rect},
        },
        widgets::textarea::{validators::required_validator, Input},
        Action, Component, ComponentAccessors, Frame,
    },
    tui::widgets::{CocoHeader, LabeledTextArea, LabeledTextAreaTheme, StatusHint},
};

component! {
    pub struct CommitStep {
        theme: Theme,
        config: CocoConfig,
        app_state: MutexAppState,
        summary_input: LabeledTextArea<'static>,
        body_input: LabeledTextArea<'static>,
        footer_input: LabeledTextArea<'static>,
        active_input: InputType,
    }
}

/// Calculate the maximum character count for the summary input, based on the current configuration
/// and the kind and scope of the commit.
fn calculate_summary_max_char_count(app_state: &MutexAppState) -> usize {
    let state = { app_state.lock().unwrap() };
    let scope = state.get_scope().unwrap_or_default();
    let kind = state.get_kind();
    let use_emoji = state.config.use_emoji;

    if let Some(kind) = kind {
        // the commit message has a format:
        // <type>(<scope>): {emoji} <summary>
        let scope_len = if scope.len() > 0 { scope.len() + 2 } else { 0 };
        let type_len = kind.name.len();
        let emoji_len = if use_emoji { 3 } else { 0 }; // emoji + 2 spaces
        state.config.max_summary_length - (type_len + scope_len + 1 + emoji_len)
    } else {
        state.config.max_summary_length
    }
}

impl CommitStep {
    pub fn new(theme: Theme, app_state: MutexAppState) -> Self {
        let max_summary_char_count = calculate_summary_max_char_count(&app_state);

        let summary_input = LabeledTextArea::new(LabeledTextAreaTheme {
            main_bg: theme.get("textarea:bg"),
            main_fg: theme.get("textarea:fg"),
            main_sel: theme.get("textarea:sel"),
            header_bg: theme.get("summary:bg"),
            header_fg: theme.get("summary:fg"),
            header_sec: theme.get("summary:sec"),
        })
        .with_title("summary")
        .with_subtitle(format!("* {}", t!("required")))
        .with_single_line(true)
        .with_max_char_count(max_summary_char_count)
        .with_validations([required_validator]);

        let optional_subtitle =
            format!("({}) alt/shift/ctrl + enter {}", t!("optional"), t!("for new line"));

        let body_input = LabeledTextArea::new(LabeledTextAreaTheme {
            main_bg: theme.get("textarea:bg"),
            main_fg: theme.get("textarea:fg"),
            main_sel: theme.get("textarea:sel"),
            header_bg: theme.get("body:bg"),
            header_fg: theme.get("body:fg"),
            header_sec: theme.get("body:sec"),
        })
        .with_title("body")
        .with_subtitle(&optional_subtitle)
        .with_active(false);

        let footer_input = LabeledTextArea::new(LabeledTextAreaTheme {
            main_bg: theme.get("textarea:bg"),
            main_fg: theme.get("textarea:fg"),
            main_sel: theme.get("textarea:sel"),
            header_bg: theme.get("footer:bg"),
            header_fg: theme.get("footer:fg"),
            header_sec: theme.get("footer:sec"),
        })
        .with_title("footer")
        .with_subtitle(&optional_subtitle)
        .with_active(false);

        let config = { app_state.lock().unwrap().config.clone() };

        Self {
            config,
            active_input: InputType::Summary,
            theme: theme.clone(),
            app_state: app_state.clone(),
            summary_input,
            body_input,
            footer_input,
            ..Default::default()
        }
    }

    fn navigate(&mut self, direction: NavigationDirection) {
        let result = match direction {
            NavigationDirection::Next => self.active_input.next(&self.config),
            NavigationDirection::Prev => self.active_input.prev(&self.config),
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
        // only if the active input is valid
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

    fn on_active_changed(&mut self, active: bool) {
        if active {
            // calculate the max char count for the summary input, sin ce it might have changed
            let max_summary_char_count = calculate_summary_max_char_count(&self.app_state);
            self.summary_input.set_max_char_count(max_summary_char_count);
        }
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) {
        let [header_area, title_area, area] = self.layout(area);

        let (kind, scope) = {
            let state = self.app_state.lock().unwrap();
            (state.get_kind(), state.get_scope())
        };

        // draw the header and title
        let header = CocoHeader::default()
            .left_fg(self.theme.get("logo:fg:1"))
            .right_fg(self.theme.get("logo:fg:2"));
        let title = StatusHint::new(kind, scope);
        f.render_widget(header, header_area);
        f.render_widget(title, title_area);

        // draw the text areas

        let (summary_area, second_textarea, third_textarea) = self.get_textareas_layout(area);

        // the summary is mandatory, so it's always rendered
        f.render_widget(&self.summary_input, summary_area);

        // the body and footer inputs are only rendered if they are not disabled by the user
        // in the config and if they have been active at least once (that's what the touched flag
        // is for, and it means that the user has navigated to the input at least once)

        let footer_area = {
            if self.config.ask_body && self.body_input.is_touched() {
                // if the body is enabled, render it and set the footer to be rendered in the
                // third slot of the layout (provided it's enabled)
                f.render_widget(&self.body_input, second_textarea);
                third_textarea
            } else {
                // if body is disabled, render the footer in the second slot (provided it's enabled)
                second_textarea
            }
        };

        // render the footer if it's enabled and has been touched
        if self.config.ask_footer && self.footer_input.is_touched() {
            f.render_widget(&self.footer_input, footer_area);
        }
    }
}
