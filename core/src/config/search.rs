use {
    eyre::{OptionExt, Result},
    std::{
        env::{current_dir, current_exe},
        path::{Path, PathBuf},
    },
};

const POSSIBLE_CONFIG_FILE_NAMES: [&str; 3] = ["coco.yml", "coco.yaml", ".cocorc"];

/// Try to find the paths to the global and current directory config files.
///
/// Searches for the global and current directory config files.
///
/// - The **global config file** is searched in the home directory and the executable directory
///   (_first found_).
/// - The **current directory config file** is searched in the current directory and its parents
///   up to the root.
pub fn fetch_config_paths() -> Result<(Option<PathBuf>, Option<PathBuf>)> {
    // Get current directory and executable directory
    let exe_dir = current_exe()?
        .parent()
        .map(Path::to_path_buf)
        .ok_or_eyre("Unable to get executable directory")?;
    let home_dir = dirs::home_dir().ok_or_eyre("Unable to get home directory")?;

    get_config_paths(&home_dir, &exe_dir)
}

/// `@internal`
///
/// Get the paths to the global and current directory config files if they exist.
///
/// Separated from fetch_config_paths for testing purposes (to be able specify custom directories).
fn get_config_paths(home_dir: &Path, exe_dir: &Path) -> Result<(Option<PathBuf>, Option<PathBuf>)> {
    // Get current directory
    let current_dir = current_dir()?;

    // Find the global config file from the home or executable directory
    let global_config = match find_config_file_in_dir(home_dir)? {
        Some(path) => Some(path),
        None => find_config_file_in_dir(exe_dir)?,
    };

    // Find the config file in the current directory or upwards to the root
    let current_config = find_nearest_config_file(current_dir.as_path())?;

    Ok((global_config, current_config))
}

/// Returns the path to the nearest config file in the directory or its parents up to the root.
fn find_nearest_config_file(start_dir: &Path) -> Result<Option<PathBuf>> {
    let mut dir = Some(start_dir);
    while let Some(current_path) = dir {
        if let Some(path) = find_config_file_in_dir(current_path)? {
            return Ok(Some(path));
        }
        dir = current_path.parent(); // Move to the parent directory
    }
    Ok(None)
}

/// Returns the path to the config file in a specific directory if it exists.
fn find_config_file_in_dir(dir: &Path) -> Result<Option<PathBuf>> {
    for &file_name in &POSSIBLE_CONFIG_FILE_NAMES {
        let candidate = dir.join(file_name);
        if candidate.exists() {
            return Ok(Some(candidate));
        }
    }

    Ok(None)
}
