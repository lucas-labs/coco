//! Git List Staged
//!
//! This module provides a way to execute a `git diff --name-only --cached` command to get a list of
//! staged files.

use {eyre::Result, lool::fail, std::process::Command};

/// Get a list of staged files in the repository.
pub fn list_staged(cwd: Option<&str>) -> Result<Vec<String>> {
    let cwd = cwd.unwrap_or("./");

    let output = Command::new("git")
        .current_dir(cwd)
        .args(["--no-pager", "diff", "--name-only", "--cached"])
        .output()?;

    if !output.status.success() {
        return fail!("Failed to list staged files");
    }

    let out = String::from_utf8(output.stdout)?;

    if out.is_empty() {
        return Ok(vec![]);
    }

    Ok(out.trim().split('\n').map(|x| x.to_string()).collect())
}
