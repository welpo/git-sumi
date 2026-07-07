use crate::errors::SumiError;
use std::sync::OnceLock;

static COMMENT_CHAR: OnceLock<String> = OnceLock::new();

/// Comment character used to strip comment lines before linting.
/// Cached for the process lifetime. Falls back to "#" when git is
/// unavailable or `core.commentChar` is unset, empty, or "auto".
pub fn commentchar() -> &'static str {
    COMMENT_CHAR.get_or_init(|| match get_git_commentchar() {
        Ok(c) if !c.is_empty() && c != "auto" => c,
        _ => "#".to_string(),
    })
}

pub fn get_git_commentchar() -> Result<String, SumiError> {
    let output = std::process::Command::new("git")
        .arg("config")
        .arg("--get")
        .arg("core.commentchar")
        .output()?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        if stderr.is_empty() {
            // commentchar isn't set, so fallback to #
            Ok("#".to_string())
        } else {
            Err(SumiError::GeneralError {
                details: format!("Failed to get git comment character: {}", stderr.trim()),
            })
        }
    }
}

pub fn remove_verbose_output(commit_message: &str) -> Result<String, SumiError> {
    let commentchar = get_git_commentchar()?;
    let cutline = format!(
        "{} ------------------------ >8 ------------------------",
        commentchar
    );

    match commit_message.lines().position(|line| line == cutline) {
        Some(i) => {
            let truncated: Vec<&str> = commit_message.lines().take(i).collect();
            Ok(truncated.join("\n"))
        }
        None => Ok(commit_message.to_string()),
    }
}

pub fn execute_git_commit(commit_message: &str) -> Result<std::process::Output, SumiError> {
    std::process::Command::new("git")
        .args(["commit", "-m", commit_message])
        .output()
        .map_err(SumiError::from)
}

pub fn get_commits_in_range(from: &str, to: &str) -> Result<Vec<(String, String)>, SumiError> {
    let range = format!("{from}..{to}");
    let output = std::process::Command::new("git")
        .args(["log", "--reverse", "-z", "--format=%H%n%B", &range])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(SumiError::GeneralError {
            details: format!(
                "Failed to get commits in range '{range}': {}",
                stderr.trim()
            ),
        });
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut commits = Vec::new();
    // With `-z`, each record is "<sha>\n<message>", NUL-terminated.
    // Empty messages are kept so they fail linting instead of being skipped.
    for record in stdout.split('\0') {
        if record.is_empty() {
            continue;
        }
        let (sha, message) = record.split_once('\n').unwrap_or((record, ""));
        commits.push((sha.to_string(), message.trim().to_string()));
    }

    Ok(commits)
}
