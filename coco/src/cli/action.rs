use {eyre::Result, pico_args::Arguments};

#[derive(Debug, Clone)]
pub enum Action {
    Help,
    Version,
    Default,
}

pub fn get_action() -> Result<Action> {
    let mut arguments = Arguments::from_env();

    if arguments.contains(["-h", "--help"]) {
        return Ok(Action::Help);
    }

    // check if wants version
    if arguments.contains(["-v", "--version"]) {
        return Ok(Action::Version);
    }

    Ok(Action::Default)
}
