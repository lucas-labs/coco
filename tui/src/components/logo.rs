use {
    cc_core::config::Theme,
    matetui::{
        component,
        ratatui::{
            layout::Rect,
            prelude::{Alignment, Color, Line, Modifier, Span, Style, Text},
            widgets::Paragraph,
        },
        Component, Frame,
    },
    std::sync::atomic::{AtomicI32, Ordering},
    tokio::time::sleep,
    tokio_util::sync::CancellationToken,
};

static ID_COUNT: AtomicI32 = AtomicI32::new(1);

fn id() -> i32 {
    ID_COUNT.fetch_add(1, Ordering::Relaxed)
}

component! {
    pub struct LogoComponent {
        _theme: Theme,
        blink: bool,
        blink_state: bool,
        cancel_blink: CancellationToken,
        id: i32,
    }
}

impl LogoComponent {
    pub fn new(_theme: Theme) -> Self {
        Self {
            _theme,
            blink: false,
            blink_state: true,
            is_active: true,
            id: id(),
            ..Default::default()
        }
    }

    pub fn with_blinking(mut self, blink: bool) -> Self {
        self.blink = blink;
        self
    }

    pub fn stop_blinking(&mut self) {
        self.blink = false;
        self.cancel_blink.cancel();
    }

    pub fn start_blinking(&mut self) {
        self.blink = true;
        self.cancel_blink = CancellationToken::new();
        let tx = self.action_sender.clone().unwrap();
        let cancel = self.cancel_blink.clone();
        let blink_action = format!("{}:logo:blink", self.id);

        tokio::spawn(async move {
            loop {
                if cancel.is_cancelled() {
                    break;
                }

                sleep(std::time::Duration::from_millis(200)).await;
                tx.send(blink_action.clone()).unwrap();
            }
        });
    }

    fn colors(&self) -> (Style, Style) {
        // TODO: use theme colors instead of hardcoded colors
        if self.blink {
            if self.blink_state {
                (
                    Style::default().fg(Color::Blue).add_modifier(Modifier::DIM),
                    Style::default().fg(Color::Magenta),
                )
            } else {
                (
                    Style::default().fg(Color::Blue),
                    Style::default().fg(Color::Magenta).add_modifier(Modifier::DIM),
                )
            }
        } else {
            (Style::default().fg(Color::Blue), Style::default().fg(Color::Magenta))
        }
    }
}

impl Component for LogoComponent {
    fn init(&mut self, _area: matetui::ratatui::prelude::Size) {
        if self.blink {
            self.start_blinking();
        }
    }

    fn receive_message(&mut self, message: String) {
        let blink_action = format!("{}:logo:blink", self.id);
        let start_blink_action = format!("{}:logo:start-blinking", self.id);

        if message == blink_action {
            self.blink_state = !self.blink_state;
        }

        if message == start_blink_action && !self.blink {
            self.blink = true;
            self.start_blinking();
        }
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) {
        let mut lines = vec![];
        let (style1, style2) = self.colors();

        lines.push(Line::from(vec![
            Span::styled("╔══ ╔═╗ ", style1),
            Span::styled("╔══ ╔═╗", style2),
        ]));

        lines.push(Line::from(vec![
            Span::styled("╚══ ╚═╝ ", style1),
            Span::styled("╚══ ╚═╝", style2),
        ]));

        f.render_widget(Paragraph::new(Text::from(lines)).alignment(Alignment::Center), area);
    }
}
