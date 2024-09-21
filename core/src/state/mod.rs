pub mod commit;

use {
    crate::config::{CocoConfig, CommitKind},
    commit::ConventionalCommit,
    std::{
        collections::HashMap,
        sync::{Arc, Mutex},
    },
};

pub type Keys = Vec<(String, String)>;

#[derive(PartialEq, Eq, Clone)]
pub enum StepStatus {
    Valid,
    Invalid,
}

/// The global application state, which will be shared between the main component and its children.
pub struct AppState {
    pub config: CocoConfig,
    sender: Option<tokio::sync::mpsc::UnboundedSender<String>>,
    step_status: HashMap<String, StepStatus>,
    kind: Option<CommitKind>,
    scope: Option<String>,
    summary: Option<String>,
    body: Option<Vec<String>>,
    footer: Option<Vec<String>>,
    breaking: bool,
}

impl AppState {
    fn new() -> Self {
        Self {
            sender: None,
            step_status: HashMap::new(),
            config: CocoConfig::from_files(),
            kind: None,
            scope: None,
            summary: None,
            body: None,
            footer: None,
            breaking: false,
        }
    }

    // fn send(&self, action: String) {
    //     if let Some(sender) = self.sender.as_ref() {
    //         let _ = sender.send(action);
    //     }
    // }

    pub fn set_sender(&mut self, sender: tokio::sync::mpsc::UnboundedSender<String>) {
        self.sender = Some(sender);
    }

    pub fn get_kind(&self) -> Option<CommitKind> {
        self.kind.clone()
    }

    pub fn set_kind(&mut self, kind: Option<CommitKind>) {
        self.kind = kind;
    }

    pub fn set_step_status(&mut self, step: &str, status: StepStatus) {
        self.step_status.insert(step.to_string(), status);
    }

    pub fn get_step_status(&self, step: &str) -> Option<StepStatus> {
        self.step_status.get(step).cloned()
    }

    pub fn set_scope(&mut self, scope: Option<String>) {
        self.scope = scope;
    }

    pub fn get_scope(&self) -> Option<String> {
        self.scope.clone()
    }

    pub fn set_summary(&mut self, summary: String) {
        self.summary = Some(summary);
        self.update_commit_step_status();
    }

    pub fn get_summary(&self) -> Option<String> {
        self.summary.clone()
    }

    pub fn set_body(&mut self, body: &[String]) {
        self.body = Some(body.to_vec());
        self.update_commit_step_status();
    }

    pub fn get_body(&self) -> Option<Vec<String>> {
        self.body.clone()
    }

    pub fn set_footer(&mut self, footer: &[String]) {
        self.footer = Some(footer.to_vec());
        self.update_commit_step_status();
    }

    pub fn get_footer(&self) -> Option<Vec<String>> {
        self.footer.clone()
    }

    pub fn set_breaking(&mut self, breaking: bool) {
        self.breaking = breaking;
    }

    pub fn get_breaking(&self) -> bool {
        self.breaking
    }

    fn update_commit_step_status(&mut self) {
        // if summary length is > 0, and body and footer are not None, then set the status to valid
        let status = if self.summary.is_some()
            && !self.summary.as_ref().unwrap().is_empty()
            && self.body.is_some()
            && self.footer.is_some()
        {
            StepStatus::Valid
        } else {
            StepStatus::Invalid
        };

        self.set_step_status("commit", status);
    }

    pub fn get_commit(&self) -> ConventionalCommit {
        ConventionalCommit {
            // name of the kind
            kind: self.kind.as_ref().map(|k| k.name.clone()).unwrap_or_default(),
            emoji: self.kind.as_ref().map(|k| k.emoji.clone()),
            scope: self.scope.clone(),
            summary: self.summary.clone().unwrap_or_default(),
            body: self.body.clone(),
            footer: self.footer.clone(),
            breaking: self.breaking,
        }
    }
}

pub type MutexAppState = Arc<Mutex<AppState>>;

pub fn default_app_state() -> MutexAppState {
    MutexAppState::new(Mutex::new(AppState::new()))
}
