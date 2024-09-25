use {
    cc_core::{
        config::Theme,
        state::{MutexAppState, StepStatus},
    },
    matetui::{
        component,
        ratatui::{
            crossterm::event::KeyEvent,
            layout::Rect,
            prelude::{Constraint, Direction, Layout},
            widgets::{Paragraph, Wrap},
        },
        widgets::gridselector::{GridItem, GridSelector, GridSelectorState},
        Action, Component, ComponentAccessors, Frame,
    },
    tui::widgets::{CocoHeader, LabeledTextArea, LabeledTextAreaTheme, StatusHint},
};

component! {
    pub struct ScopeStep {
        theme: Theme,
        app_state: MutexAppState,
        grid_state: Option<GridSelectorState>,
        scope_input: Option<LabeledTextArea<'static>>,
    }
}

impl ScopeStep {
    pub fn new(theme: Theme, app_state: MutexAppState) -> Self {
        // if there are scopes in the config, we use a grid selector
        // and if not, we use a text area
        let grid_state = {
            let state = app_state.lock().unwrap();
            if !state.config.scopes.is_empty() {
                Some(GridSelectorState::new(state.config.scopes.clone()).columns(5))
            } else {
                None
            }
        };

        let scope_input = if grid_state.is_none() {
            let input = LabeledTextArea::new(LabeledTextAreaTheme {
                main_bg: theme.get("textarea:bg"),
                main_fg: theme.get("textarea:fg"),
                main_sel: theme.get("textarea:sel"),
                header_bg: theme.get("scope:bg"),
                header_fg: theme.get("scope:fg"),
                header_sec: theme.get("scope:sec"),
            })
            .with_title("Scope")
            .with_subtitle("optional")
            .with_single_line(true)
            .with_max_char_count(20);
            Some(input)
        } else {
            None
        };

        Self {
            theme: theme.clone(),
            app_state: app_state.clone(),
            grid_state,
            scope_input,
            ..Default::default()
        }
    }
}

impl Component for ScopeStep {
    fn receive_message(&mut self, message: String) {
        if self.is_active() {
            // handle messages for the grid selector
            if let Some(grid_state) = self.grid_state.as_mut() {
                match message.as_str() {
                    "kb:right" => grid_state.move_right(),
                    "kb:left" => grid_state.move_left(),
                    "kb:down" => grid_state.move_down(),
                    "kb:up" => grid_state.move_up(),
                    "kb:home" => grid_state.move_to_row_start(),
                    "kb:end" => grid_state.move_to_row_end(),
                    "kb:enter" | "kb:pagedown" => {
                        grid_state.select();
                        let selected: Option<GridItem> = grid_state.selected();
                        let mut state = self.app_state.lock().unwrap();
                        // selected to String arr
                        state.set_scope(selected.map(|item| item.into()));
                        state.set_step_status("scope", StepStatus::Valid);
                        self.send("builder:next");
                        true
                    }
                    "kb:pageup" => {
                        self.send("builder:prev");
                        true
                    }
                    _ => true,
                };
            } else if let Some(scope_input) = self.scope_input.as_mut() {
                match message.as_str() {
                    "kb:enter" | "kb:pagedown" => {
                        let mut state = self.app_state.lock().unwrap();
                        state.set_scope(Some(scope_input.text()));
                        state.set_step_status("scope", StepStatus::Valid);
                        self.send("builder:next");
                    }
                    "kb:pageup" => self.send("builder:prev"),
                    _ => {}
                }
            }
        }
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> Option<Action> {
        if let Some(scope_input) = self.scope_input.as_mut() {
            scope_input.input(key);
        }
        None
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) {
        let [header_area, title_area, area] =
            Layout::vertical([Constraint::Length(2), Constraint::Length(2), Constraint::Fill(1)])
                .areas(area);

        // TODO: consider using RCU locks for the app state
        //       if locking on each render is too slow, consider using RCU locks.
        //       See: https://crates.io/crates/keepcalm
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

        if let Some(scope_input) = self.scope_input.as_ref() {
            // render text area
            let [textarea_area] =
                Layout::vertical([Constraint::Length(scope_input.get_height())]).areas(area);

            f.render_widget(scope_input, textarea_area);
        } else if let Some(grid_state) = self.grid_state.as_mut() {
            // render grid selector
            let [description_area, rest] = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Max(2), Constraint::Min(0)])
                .areas(area);

            let desc = Paragraph::new(
                "Select the scope of your commit (use arrows to move around, enter to select)",
            )
            .centered()
            .wrap(Wrap { trim: true });

            f.render_widget(desc, description_area);
            f.render_stateful_widget(
                GridSelector::default()
                    .with_selected_color(self.theme.get("grid:selected"))
                    .with_hovered_color(self.theme.get("grid:hovered")),
                rest,
                grid_state,
            );
        }
    }
}
