use {
    cc_core::t,
    matetui::ratatui::{
        layout::{Alignment, Constraint, Layout},
        prelude::{Buffer, Line, Rect, Span, Widget},
        style::{Color, Style, Stylize},
        text::Text,
        widgets::Paragraph,
    },
};

#[derive(Default)]
pub struct CocoHeader {
    style_left: Style,
    style_right: Style,
}

impl CocoHeader {
    pub fn new(style_left: Style, style_right: Style) -> Self {
        Self {
            style_left,
            style_right,
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

    pub fn left_fg(mut self, color: Color) -> Self {
        self.style_left = self.style_left.fg(color);
        self
    }

    pub fn right_fg(mut self, color: Color) -> Self {
        self.style_right = self.style_right.fg(color);
        self
    }

    #[inline]
    fn layout(&self, area: Rect) -> [Rect; 2] {
        Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).areas(area)
    }
}

impl Widget for CocoHeader {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [left, right] = self.layout(area);

        let title_par = Paragraph::new(Text::from(Line::from(vec![
            "coco » ".into(),
            Span::styled("conventional ", self.style_left),
            Span::styled("commits", self.style_right),
        ])));

        let help_message_par = Paragraph::new(Text::from(Line::from(vec![
            t!("Press").into(),
            " F2 ".bold(),
            t!("for help").into(),
        ])))
        .alignment(Alignment::Right);

        title_par.render(left, buf);
        help_message_par.render(right, buf);
    }
}
