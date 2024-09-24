use {
    cc_core::config::CommitKind,
    matetui::ratatui::{
        prelude::{Buffer, Line, Rect, Widget},
        style::Stylize,
        text::Text,
        widgets::Paragraph,
    },
};

/// will show the status hint from the conventional commit message... kinda like:
///
/// Creating a {type} commit [on scope {scope}]
#[derive(Default)]
pub struct StatusHint {
    kind: Option<CommitKind>,
    scope: Option<String>,
}

impl StatusHint {
    pub fn new(kind: Option<CommitKind>, scope: Option<String>) -> Self {
        Self { kind, scope }
    }
}

impl Widget for StatusHint {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut line = vec![];

        let (kind, emoji, desc) = if let Some(kind) = self.kind {
            let comm_kind = (if kind.name.is_empty() {
                "?"
            } else {
                kind.name.as_str()
            })
            .to_string()
            .green()
            .bold();

            let emoji = kind.emoji.to_string().into();
            let desc = kind.description.dim();

            (comm_kind, emoji, desc)
        } else {
            ("unknown".green().bold(), "❓".into(), "This commit kind is unknown!".dim())
        };

        line.extend(vec!["Creating a ".into(), kind, " commit".into()]);

        if let Some(scope) = self.scope {
            if !scope.is_empty() {
                line.extend(vec![" on scope ".into(), scope.blue()]);
            }
        }

        line.extend(vec![" | ".into(), emoji, " » ".dim(), desc]);

        let hint = Paragraph::new(Text::from(Line::from(line)));
        hint.render(area, buf);
    }
}
