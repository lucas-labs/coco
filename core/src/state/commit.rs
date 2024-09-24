use unicode_width::UnicodeWidthStr;

#[derive(Debug, Clone)]
pub struct CommitInfo {
    pub hash: String,
    pub author: String,
    pub author_email: String,
    pub date: String,
}

#[derive(Debug, Clone)]
pub struct Commit {
    pub info: Option<CommitInfo>,
    pub message: Option<ConventionalCommitMessage>,
}

#[derive(Debug, Clone)]
pub struct ConventionalCommitMessage {
    pub kind: String,
    pub emoji: String,
    pub scope: String,
    pub summary: String,
    pub body: Vec<String>,
    pub footer: Vec<String>,
    pub breaking: bool,
}

impl ConventionalCommitMessage {
    fn title_width(&self) -> u16 {
        UnicodeWidthStr::width(self.raw_title().as_str()) as u16
    }

    fn body_width(&self) -> u16 {
        self.body.iter().map(|s| UnicodeWidthStr::width(s.as_str())).max().unwrap_or(0) as u16
    }

    fn footer_width(&self) -> u16 {
        self.footer.iter().map(|s| UnicodeWidthStr::width(s.as_str())).max().unwrap_or(0) as u16
    }

    pub fn width(&self) -> u16 {
        self.title_width().max(self.body_width()).max(self.footer_width())
    }

    pub fn height(&self) -> u16 {
        let mut height = 1;

        let body = self.body.join("\n");
        if !body.is_empty() {
            height += (self.body.len() + 1) as u16;
        }

        let footer = self.footer.join("\n");

        if !footer.is_empty() {
            height += (self.footer.len() + 1) as u16;
        }

        height
    }

    pub fn raw_body(&self) -> String {
        self.body.join("\n").trim().to_string()
    }

    pub fn raw_footer(&self) -> String {
        self.footer.join("\n").trim().to_string()
    }

    pub fn raw_title(&self) -> String {
        format!(
            "{}{}{}: {}{}",
            self.kind,
            // if trimmed scope is not none or empty, then add it to the title
            if !self.scope.trim().is_empty() {
                format!("({})", self.scope.trim())
            } else {
                "".to_string()
            },
            if self.breaking { "!" } else { "" },
            // if trimmed emoji is not none or empty, then add it to the title
            if !self.emoji.trim().is_empty() {
                format!("{} ", self.emoji.trim())
            } else {
                "".to_string()
            },
            self.summary
        )
        .trim()
        .to_string()
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
