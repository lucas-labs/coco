use {
    super::sections::builder::BuilderSection,
    cc_core::state::MutexAppState,
    eyre::Result,
    lool::{
        children,
        tui::{
            ratatui::{Frame, Rect},
            utils::component::{pass_action_handler_to_children, pass_message_to_children},
            Children, Component,
        },
    },
    strum::{Display, EnumString},
};

#[derive(Default, EnumString, Display, PartialEq, Eq, Clone)]
#[strum(serialize_all = "kebab-case")]
enum ContentRoute {
    #[default]
    Builder,
    TodoView,
}

/// 📋 » index of the app
///
/// shows the list of todos
pub struct AppRouter {
    children: Children,
    sender: Option<tokio::sync::mpsc::UnboundedSender<String>>,
    current_route: ContentRoute,
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
                "builder" => BuilderSection::new(theme.clone(), app_state)
            ),
            current_route: ContentRoute::default(),
            sender: None,
        }
    }

    #[allow(dead_code)]
    fn route(&mut self, route: ContentRoute) {
        let current_component = self.child_mut(&self.current_route.to_string()).unwrap();
        current_component.set_active(false);

        let next_component = self.child_mut(&route.to_string()).unwrap();
        next_component.set_active(true);

        self.current_route = route.clone();
    }

    fn draw_route_content(&mut self, f: &mut Frame<'_>, rect: Rect) -> Result<()> {
        let key = self.current_route.to_string();
        let component = self.child_mut(&key).unwrap();
        component.draw(f, rect)?;

        Ok(())
    }
}

impl Component for AppRouter {
    fn register_action_handler(
        &mut self,
        tx: tokio::sync::mpsc::UnboundedSender<String>,
    ) -> Result<()> {
        self.sender = Some(tx.clone());
        pass_action_handler_to_children(self, tx)
    }

    fn get_children(&mut self) -> Option<&mut Children> {
        Some(&mut self.children)
    }

    fn receive_message(&mut self, message: String) -> Result<()> {
        match message.as_str() {
            _other => {
                // if let Some(sender) = self.sender.as_ref() {
                //     if other == "q" {
                //         let _ = sender.send(Action::Quit.to_string());
                //     }
                // }
            }
        }

        pass_message_to_children(self, message)
    }

    fn draw(&mut self, f: &mut Frame<'_>, rect: Rect) -> Result<()> {
        let buf = f.buffer_mut();
        buf.reset();
        self.draw_route_content(f, rect)
    }
}
