extern crate assert_cmd;
extern crate tempfile;

use super::contains;
use super::run_isolated_git_sumi;
use assert_cmd::Command;
use std::fs;
use std::fs::File;
use std::io::Write;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
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
    writeln!(file, "{}", content).expect("Failed to write to a file");
    drop(file);

    Command::new("git")
        .args(["add", file_name])
        .current_dir(repo_dir)
        .assert()
        .success();
}

#[test]
fn error_commit_with_no_changes() {
    let tmp_dir = setup_git_repo();
    let repo_dir = tmp_dir.path();

    let mut cmd = run_isolated_git_sumi("");
    cmd.current_dir(repo_dir)
        .arg("-C")
        .arg("--commit")
        .arg("feat: Add new feature")
        .assert()
        .failure()
        .stderr(contains("nothing to commit"));
}

#[test]
fn success_commit_with_staged_changes() {
    let tmp_dir = setup_git_repo();
    let repo_dir = tmp_dir.path();

    // Create a file and stage it.
    let file_path = repo_dir.join("new_file.txt");
    let mut file = File::create(file_path).expect("Failed to create a file");
    writeln!(file, "New content").expect("Failed to write to a file");

    Command::new("git")
        .args(["add", "new_file.txt"])
        .current_dir(repo_dir)
        .assert()
        .success();

    let mut cmd = run_isolated_git_sumi("");
    cmd.current_dir(repo_dir)
        .arg("-C")
        .arg("--commit")
        .arg("feat: Add new feature")
        .assert()
        .success();
}

#[test]
fn error_commit_in_non_repo_directory() {
    let non_repo_dir = TempDir::new().expect("Failed to create a temporary directory");

    let mut cmd = run_isolated_git_sumi("");
    cmd.current_dir(non_repo_dir.path())
        .arg("--commit")
        .arg("feat: Add new feature")
        .assert()
        .failure()
        .stderr(contains("not a git repository"));
}

#[test]
fn error_commit_lint_problems_suggest_force() {
    let tmp_dir = setup_git_repo();
    let repo_dir = tmp_dir.path();

    // Create and stage a file.
    let file_path = repo_dir.join("test.txt");
    let mut file = File::create(file_path).unwrap();
    writeln!(file, "Test content").unwrap();
    drop(file); // Close the file explicitly.

    create_and_stage_file(repo_dir, "test.txt", "Test content");

    let mut cmd = run_isolated_git_sumi("");
    cmd.current_dir(repo_dir)
        .arg("-C")
        .arg("--commit")
        .arg("non-conventional-commit message")
        .assert()
        .failure()
        .stdout(contains("Use the --force flag to commit despite errors"))
        .stderr(contains("Failed to parse as a conventional commit"));
}

#[test]
fn success_force_commit_with_incorrect_message() {
    let tmp_dir = setup_git_repo();
    let repo_dir = tmp_dir.path();

    // Create and stage a file.
    let file_path = repo_dir.join("test.txt");
    let mut file = File::create(file_path).unwrap();
    writeln!(file, "Test content").unwrap();
    drop(file); // Close the file explicitly.

    create_and_stage_file(repo_dir, "test.txt", "Test content");

    // Run the commit command with --force.
    let mut cmd = run_isolated_git_sumi("");
    cmd.current_dir(repo_dir)
        .arg("-C")
        .arg("--commit")
        .arg("--force")
        .arg("non-conventional-commit message")
        .assert()
        .success()
        .stderr(contains("Failed to parse as a conventional commit"));

    // Retrieve and inspect the git log.
    let log_output = Command::new("git")
        .args(["log", "--oneline"])
        .current_dir(repo_dir)
        .output()
        .expect("Failed to retrieve the git log");

    let log = String::from_utf8(log_output.stdout).unwrap();

    assert!(
        log.contains("non-conventional-commit message"),
        "The commit with the incorrect message was not found in the git log."
    );
}

fn setup_silent_git_fail(tmp_dir: &TempDir) -> String {
    // Create a mock git script that exits with an error code silently.
    let git_mock_path = tmp_dir.path().join("git");
    let mut git_mock = File::create(&git_mock_path).expect("Failed to create mock git file");
    writeln!(git_mock, "#!/bin/sh\nexit 1").expect("Failed to write mock git script");
    git_mock
        .sync_all()
        .expect("Failed to flush mock git script to file");

    let mut perms = git_mock
        .metadata()
        .expect("Failed to get mock git metadata")
        .permissions();
    #[cfg(unix)]
    perms.set_mode(0o755); // Set the script as executable.
    fs::set_permissions(&git_mock_path, perms).expect("Failed to set permissions for mock git");

    // Return the directory containing the mock `git` to be used in the PATH.
    tmp_dir.path().to_string_lossy().into_owned()
}

#[test]
fn error_git_fails_no_message() {
    let tmp_dir = setup_git_repo();
    let repo_dir = tmp_dir.path();

    // Create and stage a file.
    create_and_stage_file(repo_dir, "test.txt", "Test content");

    // Generate a path for the git mock that fails silently.
    let mock_path = setup_silent_git_fail(&tmp_dir);

    // Run the lint command using the local mock for git.
    let output = run_isolated_git_sumi("")
        .current_dir(repo_dir)
        // Set the local PATH to use our mock git script.
        .env("PATH", &mock_path)
        .arg("--commit")
        .arg("Initial commit")
        .output()
        .expect("Failed to execute git-sumi subcommand");

    assert!(!output.status.success());

    // Convert output.stderr to a string for the assertion.
    let stderr_output = String::from_utf8(output.stderr).expect("Failed to parse stderr as UTF-8");

    // Check if the stderr contains the expected message.
    assert!(stderr_output.contains("Commit failed. No additional error information available."));
}
