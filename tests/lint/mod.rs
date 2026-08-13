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
use std::path::{Path, PathBuf};
use tempfile::TempDir;

fn git_command() -> Command {
    const LOCAL_GIT_ENV_VARS: [&str; 15] = [
        "GIT_ALTERNATE_OBJECT_DIRECTORIES",
        "GIT_CONFIG",
        "GIT_CONFIG_PARAMETERS",
        "GIT_CONFIG_COUNT",
        "GIT_OBJECT_DIRECTORY",
        "GIT_DIR",
        "GIT_WORK_TREE",
        "GIT_IMPLICIT_WORK_TREE",
        "GIT_GRAFT_FILE",
        "GIT_INDEX_FILE",
        "GIT_NO_REPLACE_OBJECTS",
        "GIT_REPLACE_REF_BASE",
        "GIT_PREFIX",
        "GIT_SHALLOW_FILE",
        "GIT_COMMON_DIR",
    ];

    let mut command = Command::new("git");
    for variable in LOCAL_GIT_ENV_VARS {
        command.env_remove(variable);
    }
    command
}

fn setup_git_repo() -> TempDir {
    let tmp_dir = TempDir::new().expect("Failed to create a temporary directory");
    let repo_dir = tmp_dir.path();

    // Initialize a git repository.
    git_command()
        .args(["init"])
        .current_dir(repo_dir)
        .assert()
        .success();

    // Disable GPG signing (otherwise it can prompt for a passphrase during tests).
    git_command()
        .args(["config", "commit.gpgsign", "false"])
        .current_dir(repo_dir)
        .assert()
        .success();

    // Set the user name and email.
    git_command()
        .args(["config", "user.name", "Test User"])
        .current_dir(repo_dir)
        .assert()
        .success();
    git_command()
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

    git_command()
        .args(["add", file_name])
        .current_dir(repo_dir)
        .assert()
        .success();
}

fn prepare_git_commit_message(repo_dir: &Path, initial_message: Option<&str>) -> PathBuf {
    let mut command = git_command();
    command.args(["commit", "--edit", "--cleanup=verbatim"]);
    if let Some(message) = initial_message {
        command.args(["--message", message]);
    }
    command
        // Git receives COMMIT_EDITMSG as a nonexistent subcommand and aborts,
        // leaving its generated message file available for the test.
        .env("GIT_EDITOR", "git")
        .current_dir(repo_dir)
        .assert()
        .failure();

    let output = git_command()
        .args(["rev-parse", "--git-path", "COMMIT_EDITMSG"])
        .current_dir(repo_dir)
        .output()
        .unwrap();
    assert!(output.status.success());
    let message_path = PathBuf::from(String::from_utf8(output.stdout).unwrap().trim());
    let message_path = if message_path.is_absolute() {
        message_path
    } else {
        repo_dir.join(message_path)
    };
    assert!(message_path.exists());
    message_path
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
