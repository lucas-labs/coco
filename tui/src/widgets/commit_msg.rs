use {
    cc_core::{color, state::commit::ConventionalCommitMessage},
    matetui::ratatui::{
        prelude::{Buffer, Line, Rect, Span, Stylize, Widget},
        widgets::Paragraph,
    },
};

pub struct CommitMessage {
    msg: ConventionalCommitMessage,
}

impl CommitMessage {
    pub fn new(msg: ConventionalCommitMessage) -> Self {
        Self { msg }
    }
}

impl Widget for CommitMessage {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut lines = vec![];
        // # Title

        // kind(scope)!: {emoji} summary
        // the ! is only shown if breaking change
        // the (scope) is only shown if scope is present
        // the emoji is only shown if emoji is present
        let mut description_line = vec![Span::from(self.msg.kind).bold().fg(color("#8cc265"))];

        if !self.msg.scope.is_empty() {
            description_line.extend_from_slice(&[
                "(".black(),
                Span::from(self.msg.scope).bold().fg(color("#125acc")),
                ")".black(),
            ]);
        }

        if self.msg.breaking {
            description_line.push(Span::from("!").black());
        }

        description_line.push(Span::from(": ").black());

        description_line.extend_from_slice(&[Span::from(format!("{} ", self.msg.emoji)).bold()]);

        description_line.push(Span::from(self.msg.summary).fg(color("#6a4ac3")));

        lines.push(Line::from(description_line));

        if !is_empty(&self.msg.body) {
            lines.push(Line::from(vec![Span::from("")]));

            for line in self.msg.body {
                lines.push(Line::from(vec![Span::from(line)]).fg(color("#f34e70")));
            }
        }

        if !is_empty(&self.msg.footer) {
            lines.push(Line::from(vec![Span::from("")]));

            for line in self.msg.footer {
                lines.push(Line::from(vec![Span::from(line)]).fg(color("#db279f")));
            }
        }

        Paragraph::new(lines).render(area, buf);
    }
}

/// returns true if the given all strings joined by \n ends up being empty or None
fn is_empty(s: &Vec<String>) -> bool {
    s.join("\n").is_empty()
}
