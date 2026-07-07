mod test_combined_rules;
mod test_comments;
mod test_commit_changes;
mod test_commit_range;
mod test_config;
mod test_conventional_commits;
mod test_display;
mod test_file_input;
mod test_gitmoji;
mod test_header_pattern_stripping;
mod test_single_rule;

use super::contains;
use super::run_isolated_git_sumi;
use assert_cmd::Command;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use tempfile::TempDir;

fn setup_git_repo() -> TempDir {
    let tmp_dir = TempDir::new().expect("Failed to create a temporary directory");
    let repo_dir = tmp_dir.path();

    // Initialize a git repository.
    Command::new("git")
        .args(["init"])
        .current_dir(repo_dir)
        .assert()
        .success();

    // Disable GPG signing (otherwise it can prompt for a passphrase during tests).
    Command::new("git")
        .args(["config", "commit.gpgsign", "false"])
        .current_dir(repo_dir)
        .assert()
        .success();

    // Set the user name and email.
    Command::new("git")
        .args(["config", "user.name", "Test User"])
        .current_dir(repo_dir)
        .assert()
        .success();
    Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(repo_dir)
        .assert()
        .success();

    tmp_dir
}

fn create_and_stage_file(repo_dir: &Path, file_name: &str, content: &str) {
    let file_path = repo_dir.join(file_name);
    let mut file = File::create(file_path).expect("Failed to create a file");
    writeln!(file, "{content}").expect("Failed to write to a file");
    drop(file);

    Command::new("git")
        .args(["add", file_name])
        .current_dir(repo_dir)
        .assert()
        .success();
}

#[test]
fn error_exits_no_commit() {
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("");
    let output = cmd.output().unwrap();
    assert_eq!(output.status.code(), Some(1));
}

#[test]
fn error_lint_no_rules() {
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("If linting is dead, then everything is permitted!")
        .assert()
        .failure()
        .stderr(contains("No rules enabled. Enable at least one rule with"));
}
