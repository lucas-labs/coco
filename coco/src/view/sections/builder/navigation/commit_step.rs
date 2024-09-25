use cc_core::config::CocoConfig;

#[derive(PartialEq, Default)]
pub enum InputType {
    #[default]
    Summary,
    Body,
    Footer,
}

pub enum NavigationResult {
    PrevStep,
    Input(InputType),
    NextStep,
}

pub enum NavigationDirection {
    Next,
    Prev,
}

impl InputType {
    pub fn next(&self, config: &CocoConfig) -> NavigationResult {
        let next = match self {
            Self::Summary => NavigationResult::Input(Self::Body),
            Self::Body => NavigationResult::Input(Self::Footer),
            Self::Footer => NavigationResult::NextStep,
        };

        match next {
            NavigationResult::Input(next) if !is_step_enabled(&next, config) => next.next(config),
            _ => next,
        }
    }

    pub fn prev(&self, config: &CocoConfig) -> NavigationResult {
        let prev = match self {
            Self::Summary => NavigationResult::PrevStep,
            Self::Body => NavigationResult::Input(Self::Summary),
            Self::Footer => NavigationResult::Input(Self::Body),
        };

        match prev {
            NavigationResult::Input(prev) if !is_step_enabled(&prev, config) => prev.prev(config),
            _ => prev,
        }
    }
}

fn is_step_enabled(step: &InputType, config: &CocoConfig) -> bool {
    match step {
        InputType::Summary => true,
        InputType::Body => config.ask_body,
        InputType::Footer => config.ask_footer,
    }
}
