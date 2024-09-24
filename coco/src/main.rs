mod cli;
mod view;

use {
    cc_core::git,
    cli::{
        action::{get_action, Action},
        helpver::{help, version},
    },
    eyre::Result,
    matetui::{
        components, kb, ratatui::crossterm::style::Stylize, Action as MatetuiAction, App,
        ComponentAccessors,
    },
    view::main_component::MainComponent,
};

// TODO: i18n
//       implement internationalization for all the literal strings in the app.
//
//       > Check the [rust-i18n](https://github.com/longbridgeapp/rust-i18n) crate for a possible
//         half baked solution.

#[tokio::main]
async fn main() -> Result<()> {
    let action = get_action()?;

    match action {
        Action::Help | Action::Version => handle_cli_action(action)?,
        Action::Coco(stage_check) => match git::list_staged(Some("./")) {
            Ok(staged) => {
                if staged.is_empty() && stage_check {
                    println!(
                        "{}",
                        "Nothing to commit! Stage your changes first ('git add .')".red()
                    );
                    return Ok(());
                }

                let mut app = App::default()
                    .with_frame_rate(32)
                    .with_tick_rate(1)
                    .with_keybindings(kb! {
                        "<ctrl-c>" => MatetuiAction::Quit,
                        "<up>" => "kb:up",
                        "<down>" => "kb:down",
                        "<left>" => "kb:left",
                        "<right>" => "kb:right",
                        "<home>" => "kb:home",
                        "<end>" => "kb:end",
                        "<enter>" => "kb:enter",
                        "<pageup>" => "kb:pageup",
                        "<pagedown>" => "kb:pagedown",
                        "<space>" => "kb:space",
                        "<f2>" => "kb:f2",
                    })
                    .with_components(components![MainComponent::new().as_active()]);

                app.run().await?;

                // TODO: Show the final commit message before quitting
                //       When we have succesfully committed the changes, we should show the final
                //       commit information to the user before quitting the app.
                //       To do this, we will need access to the `AppState` from the `main` function.
                //       This means that we will need to actually create the `AppState` there and
                //       pass it down to the `MainComponent`.
                println!("\n ╭\n{{ }} Chau!\n");
            }
            Err(e) => println!("{}: {}", "Error listing staged files".red(), e),
        },
    }

    Ok(())
}

fn handle_cli_action(action: Action) -> Result<()> {
    match action {
        Action::Help => help(),
        Action::Version => version(),
        _ => Ok(()),
    }
}
