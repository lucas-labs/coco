use {
    super::app_router::AppRouter,
    cc_core::state::{default_app_state, MutexAppState},
    eyre::{Ok, Result},
    lool::{
        children,
        tui::{
            ratatui::{Frame, Rect},
            utils::component::pass_action_handler_to_children,
            Children, Component,
        },
    },
    tokio::sync::mpsc::UnboundedSender,
};

/// The global application status, which will be used to display different UIs based on the status.
#[derive(Clone, Default, PartialEq)]
#[allow(dead_code)]
pub enum AppStatus {
    #[default]
    Loading,
    Ready,
    Error,
}

/// 📋 » the main/parent component of the application
///
/// The main component act as a "router" between the main components of the application, but
/// also as the source of truth for the global application state.
///
/// The main components depends on the master status of the app. These are status that once
/// advanced to, the app will not be able to go back to the previous status:
/// - `Loading`: The app is starting (checking git repo status, reading config, etc.)
/// - `Ready`: The app is ready to be used. This is the main status of the app and it's the one
///   that will allow the user to interact with it.
/// - `Error`: The app has encountered a fatal error and it won't be able to recover from it. The
///   only way to continue is to quit the app.
///
/// This means that it will be the one to fetch the todos and then pass them down to the children.
/// It will also control which children are active and which are not.
///
/// TODO: implement the Loading and Error components and logic:
/// There are three main children components:
/// - `Loading`: A component that will be displayed when the app is starting. This component
///   will be the first to be active and it will just show a loading message or spinner (or
///   nothing). It will be active until the git repo status is checked and the config files have
///   been read and loaded.
/// - `AppRouter`: The main section of the application, where we will be able to interact with
///   the app. This will route between the different main sections of the app and will shouw the
///   forms and info to be able to build our conventional commits.
/// - `Error`: A section that will be displayed when there's an irrecoverable error. Once
///   this section is active, the app will show the error and wait for the user to quit the app.
///   This is the only section that might never be active, but it's important to have it in the
///   flow of the app, so that the app can handle errors gracefully.
pub struct MainComponent {
    command_tx: Option<UnboundedSender<String>>,
    children: Children,
    app_state: MutexAppState,
    app_status: AppStatus,
}

impl MainComponent {
    pub fn new() -> Self {
        let state = default_app_state();

        Self {
            command_tx: None,
            app_state: state.clone(),
            app_status: AppStatus::Ready,
            children: children!(
                "router" => AppRouter::new(state.clone())
            ),
        }
    }
}

impl Component for MainComponent {
    fn register_action_handler(&mut self, tx: UnboundedSender<String>) -> Result<()> {
        {
            let mut state = self.app_state.lock().unwrap();
            state.set_sender(tx.clone());
        }

        pass_action_handler_to_children(self, tx.clone())?;
        self.command_tx = Some(tx);
        Ok(())
    }

    fn get_children(&mut self) -> Option<&mut Children> {
        Some(&mut self.children)
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
        if self.app_status == AppStatus::Ready {
            let home = self.child_mut("router").unwrap();
            home.draw(f, area)?;
        }

        Ok(())
    }
}
