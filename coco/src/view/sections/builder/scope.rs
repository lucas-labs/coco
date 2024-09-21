use {
    cc_core::{
        config::Theme,
        state::{MutexAppState, StepStatus},
    },
    eyre::Result,
    lool::tui::{
        ratatui::{
            crossterm::event::KeyEvent,
            layout::Rect,
            widgets::{Paragraph, Wrap},
            Color, Constraint, Direction, Layout,
        },
        widgets::gridselector::{GridItem, GridSelector, GridSelectorState},
        Action, Component, Frame,
    },
    tui::widgets::LabeledTextArea,
};

pub struct ScopeStep {
    _theme: Theme,
    sender: Option<tokio::sync::mpsc::UnboundedSender<String>>,
    active: bool,
    app_state: MutexAppState,
    grid_state: Option<GridSelectorState>,
    scope_input: Option<LabeledTextArea<'static>>,
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
            let input = LabeledTextArea::default()
                .with_title("Scope")
                .with_subtitle("optional")
                .with_single_line(true)
                .with_max_char_count(20);
            Some(input)
        } else {
            None
        };

        Self {
            active: false,
            _theme: theme.clone(),
            sender: None,
            app_state: app_state.clone(),
            grid_state,
            scope_input,
        }
    }

    fn send(&self, action: &str) {
        if let Some(sender) = self.sender.as_ref() {
            let _ = sender.send(action.to_string());
        }
    }
}

impl Component for ScopeStep {
    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn is_active(&self) -> bool {
        self.active
    }

    fn register_action_handler(
        &mut self,
        tx: tokio::sync::mpsc::UnboundedSender<String>,
    ) -> Result<()> {
        self.sender = Some(tx.clone());
        Ok(())
    }

    fn receive_message(&mut self, message: String) -> Result<()> {
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

        Ok(())
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> Result<Option<Action>> {
        if let Some(scope_input) = self.scope_input.as_mut() {
            scope_input.input(key);
        }

        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
        if let Some(scope_input) = self.scope_input.as_ref() {
            // render text area

            let areas = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(scope_input.get_height())])
                .split(area);

            f.render_widget(scope_input, areas[0]);
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
                    .with_selected_color(Color::Green)
                    .with_hovered_color(Color::Blue),
                rest,
                grid_state,
            );
        }

        Ok(())
    }
}
