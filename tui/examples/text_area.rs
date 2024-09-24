use {
    matetui::{
        ratatui::{
            backend::CrosstermBackend,
            crossterm::{
                event::{self},
                execute,
                terminal::{
                    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
                },
            },
            layout::{Constraint, Direction, Layout},
            Terminal,
        },
        widgets::textarea::{Input, Key},
    },
    std::{cmp, io},
    tui::widgets::LabeledTextArea,
};

fn main() -> io::Result<()> {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut term = Terminal::new(backend)?;

    let mut textarea = LabeledTextArea::default()
        .with_title("summary")
        .with_subtitle("(* required)")
        .with_max_char_count(70);

    loop {
        term.draw(|f| {
            const MIN_HEIGHT: usize = 1;
            let height = cmp::max(textarea.lines().len(), MIN_HEIGHT) as u16 + 3;
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(height), Constraint::Min(1)])
                .split(f.area());

            // Render the textarea
            f.render_widget(&textarea, layout[0]);
        })?;
        match event::read()?.into() {
            Input { key: Key::Esc, .. } => break,
            input => {
                textarea.input(input);
            }
        }
    }

    disable_raw_mode()?;
    execute!(term.backend_mut(), LeaveAlternateScreen)?;
    term.show_cursor()?;

    Ok(())
}
