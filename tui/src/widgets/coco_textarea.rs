use {
    matetui::{
        ratatui::{
            buffer::Buffer,
            layout::{Constraint, Direction, Flex, Layout, Rect},
            prelude::{Alignment, Line, Span, Widget},
            style::{Color, Modifier, Style},
            text::Text,
            widgets::{Block, Padding, Paragraph},
        },
        widgets::textarea::{Input, TextArea, ValidationResult},
    },
    std::cmp,
};

#[derive(Clone, Debug)]
pub struct LabeledTextAreaTheme {
    pub header_bg: Color,
    pub header_fg: Color,
    pub header_sec: Color,
    pub main_bg: Color,
    pub main_fg: Color,
    pub main_sel: Color,
}

impl Default for LabeledTextAreaTheme {
    fn default() -> Self {
        Self {
            header_bg: Color::Green,
            header_fg: Color::White,
            header_sec: Color::Black,
            main_bg: Color::Black,
            main_fg: Color::White,
            main_sel: Color::LightRed,
        }
    }
}

/// Labeled Text Area
///
/// A variant of [TextArea] that adds a themed block around the text area and a reactive
/// header with a title, an optional subtitle, and an optional twitter-like character count.
#[derive(Clone)]
pub struct LabeledTextArea<'a> {
    inner: TextArea<'a>,
    max_char_count: Option<usize>,
    single_line: bool,
    title: String,
    subtitle: Option<String>,
    th: LabeledTextAreaTheme,
    touched: bool,
    active: bool,
}

impl<'a> Default for LabeledTextArea<'a> {
    fn default() -> Self {
        Self::new(LabeledTextAreaTheme::default())
    }
}

impl<'a> LabeledTextArea<'a> {
    /// Create a new [LabeledTextArea] with the given [LabeledTextAreaTheme].
    pub fn new(th: LabeledTextAreaTheme) -> Self {
        let mut textarea = TextArea::new(vec![String::new()]).with_block(
            Block::default()
                .padding(Padding::symmetric(2, 1))
                .style(Style::default().bg(th.main_bg).fg(th.main_fg)),
        );

        textarea.set_selection_style(Style::default().bg(th.main_sel));

        Self {
            inner: textarea,
            max_char_count: None,
            single_line: false,
            title: "Title".to_string(),
            subtitle: Some("Subtitle".to_string()),
            active: true,
            touched: true, // if it starts active, it should be considered touched already
            th,
        }
    }

    pub fn with_validations(
        mut self,
        validations: impl IntoIterator<
            Item = impl Fn(&str) -> Result<(), String> + Send + Sync + 'static,
        >,
    ) -> Self {
        self.inner = self.inner.with_validations(validations);
        self
    }

    pub fn is_valid(&self) -> bool {
        self.inner.is_valid()
    }

    pub fn validate(&mut self) -> ValidationResult {
        self.inner.validate()
    }

    /// Set the text area to be active or not.
    pub fn set_active(&mut self, active: bool) {
        self.active = active;

        if active {
            // if active, set the textarea as touched too
            self.touched = true;

            // enable cursor styles
            self.inner = self
                .inner
                .clone()
                .with_cursor_style(Style::default().add_modifier(Modifier::REVERSED))
        } else {
            // cancel selection an disable cursor styles
            self.inner.cancel_selection();
            self.inner = self.inner.clone().with_cursor_style(Style::default());
        }
    }

    /// Get the active status of the text area.
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Set the title of the text area.
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Set the subtitle of the text area.
    pub fn with_subtitle(mut self, subtitle: impl Into<String>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }

    /// Set the active status of the text area.
    pub fn with_active(mut self, active: bool) -> Self {
        self.set_active(active);
        self.touched = active;
        self
    }

    /// Set the maximum character count for the text area.
    pub fn with_max_char_count(mut self, max_char_count: usize) -> Self {
        self.max_char_count = Some(max_char_count);
        self
    }

    /// Set the maximum character count for the text area.
    pub fn set_max_char_count(&mut self, max_char_count: usize) {
        self.max_char_count = Some(max_char_count);
    }

    /// Set the text area to be single line or not.
    pub fn with_single_line(mut self, single_line: bool) -> Self {
        self.single_line = single_line;
        self
    }

    /// Get the text area lines.
    pub fn lines(&'a self) -> &'a [String] {
        self.inner.lines()
    }

    /// Get the text area text.
    pub fn text(&self) -> String {
        self.inner.lines().join("\n")
    }

    pub fn char_count(&self) -> usize {
        let lines = self.lines();
        lines.iter().map(|line| line.chars().count()).sum::<usize>() + lines.len() - 1
    }

    pub fn input(&mut self, input: impl Into<Input>) -> bool {
        if !self.active {
            return false;
        }

        // if the input is a character and the max_char_count is reached, return false
        let input: Input = input.into();
        match input.kind() {
            ":char" => {
                if let Some(max) = self.max_char_count {
                    if self.char_count() >= max {
                        return false;
                    }
                }
            }
            ":non-enter-newline" => {
                if self.single_line {
                    return false;
                }
            }
            _ => {}
        };

        self.inner.input(input)
    }

    // Get the ratatui area of the text area, from a given area
    pub fn get_rect(&self, area: Rect) -> Rect {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(self.get_height()), Constraint::Min(1)])
            .split(area);

        layout[0]
    }

    pub fn get_height(&self) -> u16 {
        const MIN_HEIGHT: usize = 1;
        cmp::max(self.lines().len(), MIN_HEIGHT) as u16 + 3
    }

    /// Get the touched status of the text area.
    ///
    /// A text area is considered touched if it has been active at least once.
    /// This is useful to know if the user has interacted with the text area.
    pub fn is_touched(&self) -> bool {
        self.touched
    }
}

impl Widget for &LabeledTextArea<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Split the area into two: one for the header and the other for the text area
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(1), // Fixed height for the header
                Constraint::Min(0),    // Remaining space for the TextArea
            ])
            .split(area);

        // Title and subtitle with different styles
        let lines = vec![Line::from(vec![
            Span::styled(
                &self.title,
                Style::default().fg(self.th.header_fg).add_modifier(Modifier::BOLD),
            ),
            Span::raw(" "),
            Span::styled(
                self.subtitle.as_deref().unwrap_or(""),
                Style::default().fg(self.th.header_sec),
            ),
        ])];

        // Right header (character count)
        let right_header = if let Some(max) = self.max_char_count {
            format!("{}/{}", self.char_count(), max)
        } else {
            String::new()
        };

        let header_block = Block::default()
            .padding(Padding::horizontal(2))
            .style(Style::default().bg(self.th.header_bg));

        let header_inner_area = header_block.inner(chunks[0]);
        // Create the header layout using Flex and SpaceAround
        let header_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(1), Constraint::Max(10)])
            .flex(Flex::Start)
            .split(header_inner_area);

        // Render the left header (title and subtitle)
        let left_paragraph = Paragraph::new(Text::from(lines));
        left_paragraph.render(header_layout[0], buf);

        // Render the right header (character count)
        let right_paragraph = Paragraph::new(right_header)
            .style(Style::default().fg(self.th.header_sec))
            .alignment(Alignment::Right);
        right_paragraph.render(header_layout[1], buf);

        // Render the TextArea in the second chunk
        self.inner.render(chunks[1], buf);

        // Render the header block
        header_block.render(chunks[0], buf);
    }
}
