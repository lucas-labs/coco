use {
    cc_core::{config::Theme, git, state::MutexAppState},
    matetui::{
        child_downcast_mut, children, component,
        ratatui::{
            layout::{Constraint, Direction, Flex, Layout, Rect},
            style::Stylize,
            widgets::Paragraph,
        },
        Component, ComponentAccessors, Frame,
    },
    tui::components::LogoComponent,
};

component! {
    pub struct CommittingSection {
        theme: Theme,
        app_state: MutexAppState,
    }
}

impl CommittingSection {
    pub fn new(theme: Theme, app_state: MutexAppState) -> Self {
        Self {
            theme: theme.clone(),
            app_state,
            children: children![
                "logo" => LogoComponent::new(theme).with_blinking(true).as_active()
            ],
            ..Default::default()
        }
    }

    pub fn get_layout(&self, area: Rect) -> [Rect; 2] {
        // vertical and horizontal centered layout
        let [logo, description] = Layout::default()
            .constraints([Constraint::Length(2), Constraint::Length(1)])
            .flex(Flex::Center)
            .direction(Direction::Vertical)
            .areas(area);

        [logo, description]
    }

    fn spawn_commit(&self) {
        let commit_message = { self.app_state.lock().unwrap().get_commit_message() };

        let app_state = self.app_state.clone();
        let sender = self.action_sender.clone().unwrap();

        tokio::spawn(async move {
            // execute the commit
            let commit_info = git::commit(&commit_message, Some("./"));

            if let Ok(commit_info) = commit_info {
                {
                    let mut app_state = app_state.lock().unwrap();
                    app_state.set_commit_info(commit_info);
                }
                sender.send("committing:committed".into()).unwrap()
            } else {
                sender.send("committing:failed".into()).unwrap()
            }
        });
    }
}

impl Component for CommittingSection {
    fn receive_message(&mut self, message: String) {
        match message.as_str() {
            "committing:committed" => {
                if let Some(logo) = child_downcast_mut::<LogoComponent, _>(self, "logo") {
                    logo.stop_blinking();
                }
                self.send("committing:done")
            }
            "builder:done" => self.spawn_commit(),
            _ => {}
        }
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) {
        let [layout, description] = self.get_layout(area);

        let logo = self.child_mut("logo").unwrap();
        logo.draw(f, layout);

        let paragraph = Paragraph::new("executing").centered().dim();
        f.render_widget(paragraph, description);
    }
}
