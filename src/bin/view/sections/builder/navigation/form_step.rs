use coco::core::config::CocoConfig;
use strum::{Display, EnumString};

#[derive(Default, EnumString, Display, PartialEq, Eq, Clone)]
#[strum(serialize_all = "kebab-case")]
pub enum FormStep {
    #[default]
    Type,
    Scope,
    Commit,
    BreakingChange,
    Preview,
}

impl FormStep {
    pub fn next(&self, config: &CocoConfig) -> Option<Self> {
        let next = match self {
            Self::Type => Self::Scope,
            Self::Scope => Self::Commit,
            Self::Commit => Self::BreakingChange,
            Self::BreakingChange => Self::Preview,
            Self::Preview => return None,
        };

        // if the next step is disabled by the config, skip it by going to the next one
        if !is_step_enabled(&next, config) {
            return next.next(config);
        }

        Some(next)
    }

    pub fn prev(&self, config: &CocoConfig) -> Option<Self> {
        let prev = match self {
            Self::Type => return None,
            Self::Scope => Self::Type,
            Self::Commit => Self::Scope,
            Self::BreakingChange => Self::Commit,
            Self::Preview => Self::BreakingChange,
        };

        // if the previous step is disabled by the config, skip it by going to the previous one
        if !is_step_enabled(&prev, config) {
            return prev.prev(config);
        }

        Some(prev)
    }
}

fn is_step_enabled(step: &FormStep, config: &CocoConfig) -> bool {
    match step {
        FormStep::Type => true,
        FormStep::Scope => config.ask_scope,
        FormStep::Commit => true,
        FormStep::BreakingChange => config.ask_breaking_change,
        FormStep::Preview => true,
    }
}
