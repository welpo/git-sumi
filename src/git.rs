use crate::errors::SumiError;

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
        .args(["log", "--reverse", "--format=%H%x00%B%x00%x00", &range])
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
    if stdout.trim().is_empty() {
        return Ok(vec![]);
    }

    let mut commits = Vec::new();
    for chunk in stdout.split("\0\0") {
        let chunk = chunk.trim();
        if chunk.is_empty() {
            continue;
        }
        if let Some((sha, message)) = chunk.split_once('\0') {
            let sha = sha.trim().to_string();
            let message = message.trim().to_string();
            if !sha.is_empty() && !message.is_empty() {
                commits.push((sha, message));
            }
        }
    }

    Ok(commits)
}
