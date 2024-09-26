mod cli;
mod view;

use {
    cli::{
        action::{get_action, Action},
        helpver::{help, version},
    },
    coco::{
        core::{git, setup_locale, state::default_app_state},
        t,
    },
    eyre::Result,
    matetui::{
        components, kb, ratatui::crossterm::style::Stylize, Action as MatetuiAction, App,
        ComponentAccessors,
    },
    rust_i18n::i18n,
    view::main_component::MainComponent,
};

i18n!();

#[tokio::main]
async fn main() -> Result<()> {
    setup_locale();
    let action = get_action()?;

    match action {
        Action::Coco(stage_check) => match git::list_staged(Some("./")) {
            Ok(staged) => {
                if staged.is_empty() && stage_check {
                    println!(
                        "{}",
                        "Nothing to commit! Stage your changes first ('git add .')".red()
                    );
                    return Ok(());
                }

                let state = default_app_state();

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
                    .with_components(components![MainComponent::new(state.clone()).as_active()]);

                app.run().await?;

                // if the commit was performed (state.commit.info is Some, print the commit)
                let commit = state.lock().unwrap().get_commit();

                if commit.info.is_some() && commit.message.is_some() {
                    println!("{}", state.lock().unwrap().get_commit());
                } else {
                    println!("{}", t!("The commit was aborted").yellow());
                }
            }
            Err(e) => println!("{}: {}", "Error listing staged files".red(), e),
        },
        // Handle other actions
        a => handle_cli_action(a)?,
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
