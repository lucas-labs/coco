use unicode_width::UnicodeWidthStr;

pub struct ConventionalCommit {
    pub kind: String,
    pub emoji: Option<String>,
    pub scope: Option<String>,
    pub summary: String,
    pub body: Option<Vec<String>>,
    pub footer: Option<Vec<String>>,
    pub breaking: bool,
}

impl ConventionalCommit {
    fn title_width(&self) -> u16 {
        UnicodeWidthStr::width(self.raw_title().as_str()) as u16
    }

    fn body_width(&self) -> u16 {
        self.body
            .as_ref()
            .map(|b| b.iter().map(|s| UnicodeWidthStr::width(s.as_str())).max().unwrap_or(0))
            .unwrap_or(0) as u16
    }

    fn footer_width(&self) -> u16 {
        self.footer
            .as_ref()
            .map(|f| f.iter().map(|s| UnicodeWidthStr::width(s.as_str())).max().unwrap_or(0))
            .unwrap_or(0) as u16
    }

    pub fn width(&self) -> u16 {
        self.title_width().max(self.body_width()).max(self.footer_width())
    }

    pub fn height(&self) -> u16 {
        let mut height = 1;

        if let Some(body_lines) = &self.body {
            let body = body_lines.join("\n");
            if !body.is_empty() {
                height += (body_lines.len() + 1) as u16;
            }
        }

        if let Some(footer_lines) = &self.footer {
            let footer = footer_lines.join("\n");

            if !footer.is_empty() {
                height += (footer_lines.len() + 1) as u16;
            }
        }

        height
    }

    pub fn raw_body(&self) -> String {
        self.body.as_ref().map(|b| b.join("\n")).unwrap_or("".to_string())
    }

    pub fn raw_footer(&self) -> String {
        self.footer.as_ref().map(|f| f.join("\n")).unwrap_or("".to_string())
    }

    pub fn raw_title(&self) -> String {
        format!(
            "{}{}{}: {}{}",
            self.kind,
            // if trimmed scope is not none or empty, then add it to the title
            self.scope
                .as_ref()
                .map(|s| if !s.trim().is_empty() {
                    format!("({})", s.trim())
                } else {
                    "".to_string()
                })
                .unwrap_or("".to_string()),
            if self.breaking { "!" } else { "" },
            // if trimmed emoji is not none or empty, then add it to the title
            self.emoji
                .as_ref()
                .map(|e| if !e.trim().is_empty() {
                    format!("{} ", e.trim())
                } else {
                    "".to_string()
                })
                .unwrap_or("".to_string()),
            self.summary
        )
    }

    pub fn raw_commit(&self) -> String {
        let mut commit = self.raw_title();

        let raw_body = self.raw_body();
        let raw_footer = self.raw_footer();

        if !raw_body.is_empty() {
            commit.push_str("\n\n");
            commit.push_str(raw_body.as_str());
        }

        if !raw_footer.is_empty() {
            commit.push_str("\n\n");
            commit.push_str(raw_footer.as_str());
        }

        commit
    }

    pub fn size(&self) -> (u16, u16) {
        (self.width(), self.height())
    }
}
