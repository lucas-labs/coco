//! Git Commit
//!
//! This module provides a way to execute a git commit command.
use crate::{
    git::commit_info::commit_info,
    state::commit::{CommitInfo, ConventionalCommitMessage},
};
use eyre::Result;
use lool::fail;
use std::process::Command;

/// Commit the changes to the repository with the given commit message in the directory specified by
/// `cwd`. If `cwd` is not provided, the current working directory is used.
///
/// Returns a `CommitResult` which is either a `Commit` or the error returned by the git command.
pub fn commit(ccm: &ConventionalCommitMessage, cwd: Option<&str>) -> Result<CommitInfo> {
    let message = ccm.raw_commit();
    let cwd = cwd.unwrap_or("./");

    let output = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(message)
        .current_dir(cwd)
        .output()
        .expect("Failed to execute git commit");

    // Return the Commit or the error message returned by the git command
    if output.status.success() {
        let commit_out = String::from_utf8_lossy(&output.stdout).to_string();
        let (hash, _) = parse_commit_output(&commit_out)?;

        // get the commit info (git show {hash})
        commit_info(&hash, Some(cwd))
    } else {
        fail!("{}", String::from_utf8_lossy(&output.stderr).to_string())
    }
}

/// parse the output of the `git commit` command into a String tuple containing the commit hash and
/// the branch name where the commit was made.
fn parse_commit_output(commit_out: &str) -> Result<(String, String)> {
    for line in commit_out.lines() {
        if line.starts_with('[') {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                let branch = parts[0][1..].to_string();
                let hash = parts[1].trim_end_matches(']').to_string();
                return Ok((hash, branch));
            }
        }
    }

    fail!("Failed to parse commit output")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_parsing() {
        let output = vec![
            "[master e63e7aa] the first line of the message!",
            " 1 file changed, 0 insertions(+), 0 deletions(-)",
            " create mode 100644 wqeqwex.txt",
        ]
        .join("\n");

        let (hash, branch) = parse_commit_output(&output).unwrap();
        assert_eq!(hash, "e63e7aa");
        assert_eq!(branch, "master");
    }
}
