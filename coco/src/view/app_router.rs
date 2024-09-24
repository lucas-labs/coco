use {
    super::sections::{
        builder::BuilderSection, committing::CommittingSection, help::HelpSection,
        summary::SummarySection,
    },
    cc_core::state::MutexAppState,
    matetui::{
        children, component,
        ratatui::prelude::{Frame, Rect},
        Component, ComponentAccessors,
    },
    strum::{Display, EnumString},
};

#[derive(Default, EnumString, Display, PartialEq, Eq, Clone, Debug)]
#[strum(serialize_all = "kebab-case")]
enum ContentRoute {
    #[default]
    Builder,
    Committing,
    Summary,
    Help,
}

component! {
    pub struct AppRouter {
        stashed_route: Option<ContentRoute>,
        current_route: ContentRoute,
    }
}

impl AppRouter {
    pub fn new(app_state: MutexAppState) -> Self {
        let state = app_state.clone();
        let theme = {
            let state = state.lock().unwrap();
            state.config.theme.clone()
        };

        Self {
            children: children!(
                "builder" => BuilderSection::new(theme.clone(), app_state.clone()).as_active(),
                "committing" => CommittingSection::new(theme.clone(), app_state.clone()),
                "summary" => SummarySection::new(theme.clone(), app_state.clone()),
                "help" => HelpSection::new(theme.clone())
            ),
            current_route: ContentRoute::default(),
            ..Default::default()
        }
    }

    fn route(&mut self, route: ContentRoute) {
        let current_component = self.child_mut(&self.current_route.to_string()).unwrap();
        current_component.set_active(false);

        let next_component = self.child_mut(&route.to_string()).unwrap();
        next_component.set_active(true);

        self.current_route = route.clone();
    }

    fn toggle_help(&mut self) {
        // if current route is help, go back to the stashed route (and clear the stash)
        // and if there's no stashed route, we show the help

        if self.current_route == ContentRoute::Help {
            if let Some(route) = self.stashed_route.take() {
                self.route(route);
            }
        } else {
            self.stashed_route = Some(self.current_route.clone());
            self.route(ContentRoute::Help);
        }
    }

    fn draw_route_content(&mut self, f: &mut Frame<'_>, rect: Rect) {
        let key = self.current_route.to_string();
        let component = self.child_mut(&key).unwrap();
        component.draw(f, rect);
    }
}

impl Component for AppRouter {
    fn receive_message(&mut self, message: String) {
        match message.as_str() {
            "builder:done" => self.route(ContentRoute::Committing),
            "committing:done" => self.route(ContentRoute::Summary),
            "kb:f2" => self.toggle_help(),
            _ => {}
        }
    }

    fn draw(&mut self, f: &mut Frame<'_>, rect: Rect) {
        let buf = f.buffer_mut();
        buf.reset();
        self.draw_route_content(f, rect);
    }
}
