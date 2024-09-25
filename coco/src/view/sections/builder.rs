mod breaking;
mod commit;
mod preview;
mod scope;
mod r#type;
mod navigation {
    pub mod form_step;
}

use {
    breaking::BreakingChangeStep,
    cc_core::{
        config::{CocoConfig, Theme},
        state::{MutexAppState, StepStatus},
    },
    commit::CommitStep,
    matetui::{
        children, component,
        ratatui::prelude::{Frame, Rect},
        Component, ComponentAccessors,
    },
    navigation::form_step::FormStep,
    preview::PreviewStep,
    r#type::TypeStep,
    scope::ScopeStep,
};

component! {
    pub struct BuilderSection {
        app_state: MutexAppState,
        current_step: FormStep,
        config: CocoConfig,
    }
}

impl BuilderSection {
    pub fn new(theme: Theme, app_state: MutexAppState) -> Self {
        let config = { app_state.lock().unwrap().config.clone() };

        Self {
            app_state: app_state.clone(),
            current_step: FormStep::Type,
            config,
            children: children!(
                "type" => TypeStep::new(theme.clone(), app_state.clone()).as_active(),
                "scope" => ScopeStep::new(theme.clone(), app_state.clone()),
                "commit" => CommitStep::new(theme.clone(), app_state.clone()),
                "breaking-change" => BreakingChangeStep::new(theme.clone(), app_state.clone()),
                "preview" => PreviewStep::new(theme, app_state)
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
            "builder:next" => self.set_step(self.current_step.next(&self.config)),
            "builder:prev" => self.set_step(self.current_step.prev(&self.config)),
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
