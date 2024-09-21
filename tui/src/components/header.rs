use {
    cc_core::{config::Theme, state::MutexAppState},
    eyre::Result,
    lool::tui::{
        ratatui::{layout::Rect, widgets::Paragraph, Alignment, Color, Line, Span, Style, Text},
        Component, Frame,
    },
    std::sync::atomic::{AtomicI32, Ordering},
    tokio::{sync::mpsc::UnboundedSender, time::sleep},
};

static ID_COUNT: AtomicI32 = AtomicI32::new(1);

fn id() -> i32 {
    ID_COUNT.fetch_add(1, Ordering::Relaxed)
}

pub struct LogoComponent {
    _theme: Theme,
    _app_state: MutexAppState,
    blink: bool,
    blink_state: bool,
    command_tx: Option<UnboundedSender<String>>,
    id: i32,
}

impl LogoComponent {
    pub fn new(_theme: Theme, _app_state: MutexAppState) -> Self {
        Self {
            _theme,
            _app_state,
            blink: false,
            blink_state: true,
            command_tx: None,
            id: id(),
        }
    }

    pub fn with_flashing(mut self, flashing: bool) -> Self {
        self.blink = flashing;
        self
    }

    pub fn toggle_flashing(&mut self) {
        self.blink = !self.blink;
    }

    fn colors(&self) -> (Color, Color) {
        // TODO: use theme colors instead of hardcoded colors
        if self.blink {
            if self.blink_state {
                (Color::Blue, Color::Magenta)
            } else {
                (Color::Magenta, Color::Blue)
            }
        } else {
            (Color::Blue, Color::Magenta)
        }
    }

    fn setup(&mut self) {
        let tx = self.command_tx.clone().unwrap();
        let blink_action = format!("{}:logo:flash", self.id);

        tokio::spawn(async move {
            loop {
                sleep(std::time::Duration::from_millis(500)).await;
                tx.send(blink_action.clone()).unwrap();
            }
        });
    }
}

impl Component for LogoComponent {
    fn register_action_handler(&mut self, tx: UnboundedSender<String>) -> Result<()> {
        self.command_tx = Some(tx);
        self.setup();
        Ok(())
    }

    fn receive_message(&mut self, message: String) -> Result<()> {
        let blink_action = format!("{}:logo:flash", self.id);

        if message == blink_action {
            self.blink_state = !self.blink_state;
        }

        Ok(())
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
        let mut lines = vec![];
        let (color1, color2) = self.colors();

        lines.push(Line::from(vec![
            Span::styled("╔══ ╔═╗ ", Style::default().fg(color1)),
            Span::styled("╔══ ╔═╗", Style::default().fg(color2)),
        ]));

        lines.push(Line::from(vec![
            Span::styled("╚══ ╚═╝ ", Style::default().fg(color1)),
            Span::styled("╚══ ╚═╝", Style::default().fg(color2)),
        ]));

        f.render_widget(Paragraph::new(Text::from(lines)).alignment(Alignment::Center), area);
        Ok(())
    }
}
