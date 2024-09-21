use {
    cc_core::{
        config::Theme,
        state::{MutexAppState, StepStatus},
    },
    eyre::Result,
    lool::tui::{
        ratatui::{layout::Rect, widgets::Paragraph, Color, Constraint, Direction, Layout},
        widgets::gridselector::{GridSelector, GridSelectorState},
        Component, Frame,
    },
};

pub struct TypeStep {
    _theme: Theme,
    sender: Option<tokio::sync::mpsc::UnboundedSender<String>>,
    active: bool,
    app_state: MutexAppState,
    grid_state: GridSelectorState,
}

impl TypeStep {
    pub fn new(theme: Theme, app_state: MutexAppState) -> Self {
        let state = {
            let state = app_state.lock().unwrap();
            GridSelectorState::new(state.config.types.clone()).columns(5)
        };

        Self {
            active: true,
            _theme: theme.clone(),
            sender: None,
            app_state: app_state.clone(),
            grid_state: state,
        }
    }

    fn send(&self, action: &str) {
        if let Some(sender) = self.sender.as_ref() {
            let _ = sender.send(action.to_string());
        }
    }

    fn get_layout(&self, area: Rect) -> [Rect; 2] {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(2), Constraint::Min(0)])
            .areas(area)
    }
}

impl Component for TypeStep {
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
            match message.as_str() {
                "kb:right" => self.grid_state.move_right(),
                "kb:left" => self.grid_state.move_left(),
                "kb:down" => self.grid_state.move_down(),
                "kb:up" => self.grid_state.move_up(),
                "kb:home" => self.grid_state.move_to_row_start(),
                "kb:end" => self.grid_state.move_to_row_end(),
                "kb:enter" => {
                    self.grid_state.select();
                    let mut state = self.app_state.lock().unwrap();

                    let kind = self
                        .grid_state
                        .selected_index()
                        .map(|index| state.config.types[index].clone());

                    state.set_kind(kind);
                    state.set_step_status("type", StepStatus::Valid);
                    self.send("builder:next");
                    true
                }
                "kb:pagedown" => {
                    if self.grid_state.selected_index().is_some() {
                        self.send("builder:next");
                    }

                    true
                }
                _ => true,
            };
        }

        Ok(())
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
        let [title_area, rest_area] = self.get_layout(area);

        f.render_widget(
            Paragraph::new(
                "Select the type of your commit (use arrows to move around, enter to select)",
            )
            .centered(),
            title_area,
        );

        f.render_stateful_widget(
            GridSelector::default()
                .with_selected_color(Color::Green)
                .with_hovered_color(Color::Blue),
            rest_area,
            &mut self.grid_state,
        );

        Ok(())
    }
}
