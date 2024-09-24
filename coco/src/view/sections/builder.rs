mod breaking;
mod commit;
mod preview;
mod scope;
mod r#type;

use {
    breaking::BreakingChangeStep,
    cc_core::{
        config::Theme,
        state::{MutexAppState, StepStatus},
    },
    commit::CommitStep,
    matetui::{
        children, component,
        ratatui::prelude::{Frame, Rect},
        Component, ComponentAccessors,
    },
    preview::PreviewStep,
    r#type::TypeStep,
    scope::ScopeStep,
    strum::{Display, EnumString},
};

// TODO: conditional builder steps
//       - use the config to determine which steps to show, by reading the ask_* fields
//       - also, implement the missing `use_emoji` field, to be able to disable emojis in the
//         generated commit messages

#[derive(Default, EnumString, Display, PartialEq, Eq, Clone)]
#[strum(serialize_all = "kebab-case")]
enum FormStep {
    #[default]
    Type,
    Scope,
    Commit,
    BreakingChange,
    Preview,
}

impl FormStep {
    fn next(&self) -> Option<Self> {
        let next = match self {
            Self::Type => Self::Scope,
            Self::Scope => Self::Commit,
            Self::Commit => Self::BreakingChange,
            Self::BreakingChange => Self::Preview,
            Self::Preview => return None,
        };

        Some(next)
    }

    fn prev(&self) -> Option<Self> {
        let prev = match self {
            Self::Type => return None,
            Self::Scope => Self::Type,
            Self::Commit => Self::Scope,
            Self::BreakingChange => Self::Commit,
            Self::Preview => Self::BreakingChange,
        };

        Some(prev)
    }
}

component! {
    pub struct BuilderSection {
        _theme: Theme,
        app_state: MutexAppState,
        current_step: FormStep,
    }
}

impl BuilderSection {
    pub fn new(theme: Theme, app_state: MutexAppState) -> Self {
        Self {
            _theme: theme.clone(),
            app_state: app_state.clone(),
            current_step: FormStep::Type,
            children: children!(
                "type" => TypeStep::new(theme.clone(), app_state.clone()).as_active(),
                "scope" => ScopeStep::new(theme.clone(), app_state.clone()),
                "commit" => CommitStep::new(theme.clone(), app_state.clone()),
                "breaking-change" => BreakingChangeStep::new(theme.clone(), app_state.clone()),
                "preview" => PreviewStep::new(theme.clone(), app_state.clone())
            ),
            ..Default::default()
        }
    }

    fn set_step(&mut self, step: Option<FormStep>) {
        if let Some(step) = step {
            let currkey = self.current_step.to_string();
            let targetkey = step.to_string();
            let (current_valid, target_valid) = {
                let app_state = self.app_state.lock().unwrap();
                (
                    app_state.get_step_status(&currkey).unwrap_or(StepStatus::Invalid),
                    app_state.get_step_status(&targetkey).unwrap_or(StepStatus::Invalid),
                )
            };

            if current_valid == StepStatus::Valid || target_valid == StepStatus::Valid {
                self.current_step = step;

                // set all children to inactive except the current step
                for (key, component) in self.children.iter_mut() {
                    component.set_active(key == &self.current_step.to_string());
                }
            }
        }
    }
}

impl Component for BuilderSection {
    fn receive_message(&mut self, message: String) {
        match message.as_str() {
            "builder:next" => self.set_step(self.current_step.next()),
            "builder:prev" => self.set_step(self.current_step.prev()),
            "builder:restart" => self.set_step(Some(FormStep::Type)),
            _ => {}
        }
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) {
        let key = self.current_step.to_string();
        let component = self.child_mut(&key).unwrap();
        component.draw(f, area);
    }
}
