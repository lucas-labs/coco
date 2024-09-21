mod cli;
mod view;

use {
    cli::{
        action::{get_action, Action},
        helpver::{help, version},
    },
    eyre::Result,
    lool::{
        components, s,
        tui::{Action as TuiAction, App, Kb},
    },
    view::main_component::MainComponent,
};

#[tokio::main]
async fn main() -> Result<()> {
    let action = get_action()?;

    match action {
        Action::Help | Action::Version => handle_cli_action(action)?,
        _ => {
            let main = MainComponent::new();

            let mut app = App::new(
                Kb::from([
                    ("<ctrl-c>", TuiAction::Quit.to_string()),
                    // ("<q>", TuiAction::Quit.to_string()),
                    // ("<esc>", TuiAction::Quit.to_string()),
                    ("<up>", s!("kb:up")),
                    ("<down>", s!("kb:down")),
                    ("<left>", s!("kb:left")),
                    ("<right>", s!("kb:right")),
                    // ("<ctrl-left>", s!("kb:prev")),
                    // ("<ctrl-right>", s!("builder:next")),
                    // ("<i>", s!("kb:i")),
                    // ("<backspace>", s!("kb:backspace")),
                    ("<home>", s!("kb:home")),
                    ("<end>", s!("kb:end")),
                    ("<enter>", s!("kb:enter")),
                    ("<pageup>", s!("kb:pageup")),
                    ("<pagedown>", s!("kb:pagedown")),
                    ("<space>", s!("kb:space")),
                    // ("<shift-g>", s!("kb:shift-g")),
                ]),
                components![main],
            )?;

            app.tick_rate = 32.into();
            app.frame_rate = 32.into();

            app.run().await?;

            println!("\n ╭\n{{ }} Chau!\n");
        }
    }

    Ok(())
}

fn handle_cli_action(action: Action) -> Result<()> {
    match action {
        Action::Help => help(),
        Action::Version => version(),
        _ => Ok(()),
        // Action::Default => seeker::collect(None),
        // Action::DefaultWithDir(dir) => seeker::collect(Some(dir)),
    }
}
