//! Git Commit Information
//!
//! This module provides a way to execute a `git show {hash}` command in order to get some
//! useful information about a commit.

use {crate::core::state::commit::CommitInfo, eyre::Result, lool::fail, std::process::Command};

/// print the commit information for the given hash.
pub fn commit_info(hash: &str, cwd: Option<&str>) -> Result<CommitInfo> {
    let cwd = cwd.unwrap_or("./");

    let output = Command::new("git")
        .arg("--no-pager")
        .arg("show")
        .arg(hash)
        .arg("--no-color")
        .arg("-s")
        .arg("--pretty=%H%n%an%n%ae%n%ad")
        .current_dir(cwd)
        .output()
        .expect("Failed to execute git show");

    // if the command failed, return fail!(error message)
    if !output.status.success() {
        return fail!(String::from_utf8_lossy(&output.stderr).to_string());
    }

    let out = String::from_utf8_lossy(&output.stdout).to_string();
    parse_log(out)
}

/// Parse the output of the `git show {hash}` with `--pretty=%H%n%an%n%ae%n%cn%n%ce%n%ad` into a
/// [CommitInfo] struct.
fn parse_log(out: String) -> Result<CommitInfo> {
    let lines: Vec<&str> = out.split('\n').collect();

    // if it doesn't have at least 4 lines, fail with the out message
    if lines.len() < 4 {
        return fail!(out);
    }

    Ok(CommitInfo {
        hash: lines[0].to_string(),
        author: lines[1].to_string(),
        author_email: lines[2].to_string(),
        date: lines[3].to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_commit_info() {
        let comm = commit_info("9c4170d822cfa47b90edb12058a6ddd1a7c404c9", None).unwrap();
        assert_eq!(comm.hash, "9c4170d822cfa47b90edb12058a6ddd1a7c404c9");
    }

    #[test]
    fn test_parser() {
        let command_output = [
            "a80e068a86f84b31b62976cc85fd1bdf059c6c83",
            "Mr. Foo Bar",
            "foobar@baz.com",
            "Sat Sep 21 03:54:47 2024 -0300",
        ]
        .join("\n");

        let info = parse_log(command_output);

        let info = info.unwrap();

        assert_eq!(info.hash, "a80e068a86f84b31b62976cc85fd1bdf059c6c83");
        assert_eq!(info.author, "Mr. Foo Bar");
        assert_eq!(info.author_email, "foobar@baz.com");
        assert_eq!(info.date, "Sat Sep 21 03:54:47 2024 -0300");
    }
}
