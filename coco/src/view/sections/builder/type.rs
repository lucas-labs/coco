use {
    cc_core::{
        config::Theme,
        state::{MutexAppState, StepStatus},
    },
    matetui::{
        component,
        ratatui::{
            layout::Rect,
            prelude::{Color, Constraint, Layout},
            style::Stylize,
            text::Line,
            widgets::Paragraph,
        },
        widgets::gridselector::{GridSelector, GridSelectorState},
        Component, ComponentAccessors, Frame,
    },
    tui::widgets::CocoLogo,
};

component!(
    pub struct TypeStep {
        _theme: Theme,
        app_state: MutexAppState,
        grid_state: Option<GridSelectorState>,
    }
);

impl TypeStep {
    pub fn new(theme: Theme, app_state: MutexAppState) -> Self {
        let state = {
            let state = app_state.lock().unwrap();
            GridSelectorState::new(state.config.types.clone()).columns(5)
        };

        Self {
            app_state: app_state.clone(),
            grid_state: Some(state),
            _theme: theme.clone(),
            ..Default::default()
        }
    }

    fn get_layout(&self, area: Rect) -> [Rect; 4] {
        Layout::vertical([
            Constraint::Length(2),
            Constraint::Length(3),
            Constraint::Length(2),
            Constraint::Min(0),
        ])
        .areas(area)
    }
}

impl Component for TypeStep {
    fn receive_message(&mut self, message: String) {
        if self.is_active() {
            if let Some(ref mut grid_state) = self.grid_state {
                match message.as_str() {
                    "kb:right" => grid_state.move_right(),
                    "kb:left" => grid_state.move_left(),
                    "kb:down" => grid_state.move_down(),
                    "kb:up" => grid_state.move_up(),
                    "kb:home" => grid_state.move_to_row_start(),
                    "kb:end" => grid_state.move_to_row_end(),
                    "kb:enter" => {
                        grid_state.select();
                        let mut state = self.app_state.lock().unwrap();

                        let kind = grid_state
                            .selected_index()
                            .map(|index| state.config.types[index].clone());

                        state.set_kind(kind);
                        state.set_step_status("type", StepStatus::Valid);
                        self.send("builder:next");
                        true
                    }
                    "kb:pagedown" => {
                        if grid_state.selected_index().is_some() {
                            self.send("builder:next");
                        }

                        true
                    }
                    _ => true,
                };
            }
        };
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) {
        let [logo_area, help_area, title_area, rest_area] = self.get_layout(area);

        // #region Header
        let logo = CocoLogo::default().left_fg(Color::Blue).right_fg(Color::Magenta);
        f.render_widget(logo, logo_area);

        let line = Line::from(vec!["Press".into(), " F2 ".bold(), "for help".into()]);
        f.render_widget(Paragraph::new(line).centered(), help_area);
        // #endregion

        f.render_widget(
            Paragraph::new(
                "Select the type of your commit (use arrows to move around, enter to select)",
            )
            .centered(),
            title_area,
        );

        if let Some(ref mut grid_state) = self.grid_state {
            f.render_stateful_widget(
                GridSelector::default()
                    .with_selected_color(Color::Green)
                    .with_hovered_color(Color::Blue),
                rest_area,
                grid_state,
            );
        }
    }
}
