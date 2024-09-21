use {
    cc_core::{color, state::commit::ConventionalCommit},
    lool::tui::ratatui::{widgets::Paragraph, Buffer, Line, Rect, Span, Stylize, Widget},
};

pub struct CommitMessage {
    msg: ConventionalCommit,
}

impl CommitMessage {
    pub fn new(msg: ConventionalCommit) -> Self {
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

        if let Some(scope) = &self.msg.scope {
            if !scope.is_empty() {
                description_line.extend_from_slice(&[
                    "(".black(),
                    Span::from(scope).bold().fg(color("#125acc")),
                    ")".black(),
                ]);
            }
        }

        if self.msg.breaking {
            description_line.push(Span::from("!").black());
        }

        description_line.push(Span::from(": ").black());

        if let Some(emoji) = &self.msg.emoji {
            description_line.extend_from_slice(&[Span::from(format!("{} ", emoji)).bold()]);
        };

        description_line.push(Span::from(self.msg.summary).fg(color("#6a4ac3")));

        lines.push(Line::from(description_line));

        if let Some(body) = &self.msg.body {
            if !is_empty(body) {
                lines.push(Line::from(vec![Span::from("")]));

                for line in body {
                    lines.push(Line::from(vec![Span::from(line)]).fg(color("#f34e70")));
                }
            }
        }

        if let Some(footer) = &self.msg.footer {
            if !is_empty(footer) {
                lines.push(Line::from(vec![Span::from("")]));

                for line in footer {
                    lines.push(Line::from(vec![Span::from(line)]).fg(color("#db279f")));
                }
            }
        }

        Paragraph::new(lines).render(area, buf);
    }
}

/// returns true if the given all strings joined by \n ends up being empty or None
fn is_empty(s: &Vec<String>) -> bool {
    s.join("\n").is_empty()
}
