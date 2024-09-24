use matetui::ratatui::{
    layout::Alignment,
    prelude::{Buffer, Line, Rect, Span, Widget},
    style::{Color, Style},
    text::Text,
    widgets::Paragraph,
};

pub struct CocoLogo {
    style_left: Style,
    style_right: Style,
    align: Alignment,
}

impl Default for CocoLogo {
    fn default() -> Self {
        Self {
            style_left: Style::default().fg(Color::Reset),
            style_right: Style::default().fg(Color::Reset),
            align: Alignment::Center,
        }
    }
}

impl CocoLogo {
    pub fn new(style_left: Style, style_right: Style, align: Alignment) -> Self {
        Self {
            style_left,
            style_right,
            align,
        }
    }

    pub fn style_left(mut self, style: Style) -> Self {
        self.style_left = style;
        self
    }

    pub fn style_right(mut self, style: Style) -> Self {
        self.style_right = style;
        self
    }

    pub fn alignment(mut self, align: Alignment) -> Self {
        self.align = align;
        self
    }

    pub fn left_fg(mut self, color: Color) -> Self {
        self.style_left = self.style_left.fg(color);
        self
    }

    pub fn right_fg(mut self, color: Color) -> Self {
        self.style_right = self.style_right.fg(color);
        self
    }
}

impl Widget for CocoLogo {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut lines = vec![];

        lines.push(Line::from(vec![
            Span::styled("╔══ ╔═╗ ", self.style_left),
            Span::styled("╔══ ╔═╗", self.style_right),
        ]));

        lines.push(Line::from(vec![
            Span::styled("╚══ ╚═╝ ", self.style_left),
            Span::styled("╚══ ╚═╝", self.style_right),
        ]));

        let paragraph = Paragraph::new(Text::from(lines)).alignment(self.align);
        paragraph.render(area, buf);
    }
}
