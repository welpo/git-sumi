extern crate tempfile;

use super::contains;
use super::run_isolated_git_sumi;
use super::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_fake_standard_config_home() {
    // Create a temporary directory.
    let tmp_dir = tempdir().unwrap();

    // Write the fake config file to the temporary home directory.
    let tmp_path = tmp_dir.path();
    fs::write(tmp_path.join("sumi.toml"), "gitmoji = true").unwrap();

    let mut cmd = run_isolated_git_sumi("");
    cmd.env("HOME", tmp_path) // Set HOME just for this Command.
        .arg("-C")
        .arg("🌼 chore(garden): plant sunflowers in the backyard")
        .assert()
        .failure()
        .stderr(contains("Invalid emoji"));
}

#[test]
fn test_none_ignores_standard_config_home() {
    let tmp_dir = tempdir().unwrap();
    let tmp_path = tmp_dir.path();
    fs::write(tmp_path.join("sumi.toml"), "gitmoji = true").unwrap();

    let mut cmd = Command::cargo_bin("git-sumi").unwrap();
    cmd.env("HOME", tmp_path)
        .arg("--config")
        .arg("none")
        .arg("-Cdf=toml")
        .arg("chore(garden): 🌼 plant sunflowers in the backyard")
        .assert()
        .success()
        .stdout(contains(
            "description = \"🌼 plant sunflowers in the backyard\"",
        ));
}

#[test]
fn test_gitmoji_env() {
    let mut cmd = run_isolated_git_sumi("");
    cmd.env("GIT_SUMI_GITMOJI", "true")
        .arg("🌼 chore(garden): plant sunflowers in the backyard")
        .assert()
        .failure()
        .stderr(contains("Invalid emoji"));
}

#[test]
fn test_config_file_in_parent_directory() {
    let tmp_dir = tempdir().unwrap();
    let child_dir = tmp_dir.path().join("child");
    fs::create_dir(&child_dir).unwrap();

    fs::write(tmp_dir.path().join("sumi.toml"), "gitmoji = true").unwrap();

    let mut cmd = run_isolated_git_sumi("");
    cmd.current_dir(&child_dir) // Change to child directory.
        .arg("-C")
        .arg("🌼 chore(garden): plant sunflowers in the backyard")
        .assert()
        .failure()
        .stderr(contains("Invalid emoji"));
}

#[test]
fn test_config_file_in_third_parent_directory() {
    let tmp_dir = tempdir().unwrap();
    let child_dir_1 = tmp_dir.path().join("child1");
    let child_dir_2 = child_dir_1.join("child2");
    let child_dir_3 = child_dir_2.join("child3");

    // Create the directory structure.
    fs::create_dir(&child_dir_1).unwrap();
    fs::create_dir(&child_dir_2).unwrap();
    fs::create_dir(&child_dir_3).unwrap();

    // Place the configuration file in the root temp directory.
    fs::write(tmp_dir.path().join("sumi.toml"), "gitmoji = true").unwrap();

    let mut cmd = run_isolated_git_sumi("");
    cmd.current_dir(&child_dir_3) // Change to the deepest child directory.
        .arg("-C")
        .arg("🌼 chore(garden): plant sunflowers in the backyard")
        .assert()
        .failure()
        .stderr(contains("Invalid emoji"));
}

#[test]
fn config_file_in_sumi_named_subdir_in_home() {
    // Create a temporary directory to act as the home directory.
    let tmp_home_dir = tempdir().unwrap();
    // Create another temporary directory to act as the current directory.
    let tmp_current_dir = tempdir().unwrap();

    // Create a sub-directory named "git_sumi" within the temporary home directory.
    let sumi_subdir = tmp_home_dir.path().join("sumi");
    fs::create_dir(&sumi_subdir).unwrap();
    // Write a fake config file in the "git_sumi" sub-directory.
    fs::write(sumi_subdir.join("sumi.toml"), "gitmoji = true").unwrap();

    // Create and configure the command.
    let mut cmd = Command::cargo_bin("git-sumi").unwrap();
    cmd.env("HOME", tmp_home_dir.path()) // Set HOME just for this Command.
        .current_dir(tmp_current_dir.path()) // Set the current directory to our empty temporary directory.
        .arg("-C")
        .arg("🌼 chore(garden): plant sunflowers in the backyard")
        .assert()
        .failure()
        .stderr(contains("Invalid emoji"));
}

#[test]
fn error_non_existent_config_file() {
    let random_name = format!("nonexistent{}.toml", rand::random::<u64>());
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("--config")
        .arg(&random_name)
        .assert()
        .failure()
        .stderr(contains(format!(
            "Configuration file '{random_name}' not found"
        )));
}

#[test]
fn error_config_is_dir() {
    let mut cmd = Command::cargo_bin("git-sumi").unwrap();
    cmd.arg("--config")
        .arg("tests/resources")
        .arg("twizzy")
        .assert()
        .failure()
        .stderr(contains(
            "The specified path 'tests/resources' is a directory, not a configuration file",
        ));
}

// Start of tests checking for actual config files.
// We don't use `run_isolated_git_sumi` here because we want to test the config file loading logic.
#[test]
fn error_fail_override_config_file_gitmoji() {
    let mut cmd = Command::cargo_bin("git-sumi").unwrap();
    cmd.arg("--config")
        .arg("tests/resources/good_config_ciqf.toml")
        .arg("-GC")
        .arg("🔥💯 refactor(HTML): remove unused code")
        .assert()
        .failure()
        .stderr(contains("Header must contain exactly 1 emoji, found 2"));
}

#[test]
fn success_override_quiet_with_env_var() {
    let mut cmd = Command::cargo_bin("git-sumi").unwrap();
    cmd.env("GIT_SUMI_QUIET", "false") // Should override config.
        .arg("--config")
        .arg("tests/resources/good_config_ciqf.toml") // Sets quiet = true.
        .arg("Commit message")
        .assert()
        .success()
        .stdout(contains("Input"))
        .stdout(contains("passed"));
}

#[test]
fn success_override_gitmoji_with_env_var() {
    let mut cmd = Command::cargo_bin("git-sumi").unwrap();
    cmd.env("GIT_SUMI_GITMOJI", "false") // Override gitmoji = true in config.
        .arg("--config")
        .arg("tests/resources/good_config_gitmoji.toml")
        .arg("-d") // Needs a rule/display/commit.
        .arg("refactor(HTML): remove unused code")
        .assert()
        .success()
        .stderr(contains("Header must contain exactly 1 emoji").not());
}

#[test]
fn error_valid_config_file_imperative() {
    let mut cmd = Command::cargo_bin("git-sumi").unwrap();
    cmd.arg("--config")
        .arg("tests/resources/good_config_imperative.toml")
        .arg("Adds a new feature")
        .assert()
        .failure()
        .stderr(contains("Description starts with a non-imperative verb"));
}

#[test]
fn error_valid_config_file_gitmoji() {
    let mut cmd = Command::cargo_bin("git-sumi").unwrap();
    cmd.arg("--config")
        .arg("tests/resources/good_config_gitmoji.toml")
        .arg("Add a new feature")
        .assert()
        .failure()
        .stderr(contains("Header must contain exactly 1 emoji, found 0"));
}

#[test]
fn error_valid_config_file_gitmoji_overridden_by_cli() {
    let mut cmd = Command::cargo_bin("git-sumi").unwrap();
    cmd.arg("--config")
        .arg("tests/resources/good_config_gitmoji_false.toml")
        .arg("--gitmoji")
        .arg("Add a new feature")
        .assert()
        .failure()
        .stderr(contains("Header must contain exactly 1 emoji, found 0"));
}

#[test]
fn error_valid_config_file_conventional_overridden_by_cli() {
    let mut cmd = Command::cargo_bin("git-sumi").unwrap();
    cmd.arg("--config")
        .arg("tests/resources/good_config_conventional_false.toml")
        .arg("-C")
        .arg("Add a new feature")
        .assert()
        .failure()
        .stderr(contains("Missing type in the commit summary"));
}

#[test]
fn error_valid_config_file_imperative_overridden_by_cli() {
    let mut cmd = Command::cargo_bin("git-sumi").unwrap();
    cmd.arg("--config")
        .arg("tests/resources/good_config_imperative_false.toml")
        .arg("--imperative")
        .arg("Adds a new feature")
        .assert()
        .failure()
        .stderr(contains(
            "Description starts with a non-imperative verb: 'Adds'",
        ));
}

#[test]
fn success_valid_config_file_title_len_overriden_to_zero() {
    let mut cmd = Command::cargo_bin("git-sumi").unwrap();
    cmd.arg("--config")
        .arg("tests/resources/good_config_max_length_no_whitespace.toml")
        .arg("--max-header-length")
        .arg("0")
        .arg("-E")
        .arg("lower")
        .arg("hehe")
        .assert()
        .success();
}

#[test]
fn error_types_enables_conventional() {
    let mut cmd = Command::cargo_bin("git-sumi").unwrap();
    cmd.arg("--config")
        .arg("tests/resources/good_config_types.toml")
        .arg("creative: Add a new feature")
        .assert()
        .failure()
        .stderr(contains("Invalid commit type"));
}

#[test]
fn error_valid_config_file_short_title_length() {
    let mut cmd = Command::cargo_bin("git-sumi").unwrap();
    cmd.arg("--config")
        .arg("tests/resources/good_config_max_length_no_whitespace.toml")
        .arg("title too long")
        .assert()
        .failure()
        .stderr(contains("Line number 1 is too long (14 > 2)"));
}

#[test]
fn error_bad_config_file_gitmoji_duplicated() {
    let mut cmd = Command::cargo_bin("git-sumi").unwrap();
    cmd.arg("--config")
        .arg("tests/resources/bad_config_duplicated_gitmoji.toml")
        .arg("duplicate gitmoji")
        .assert()
        .failure()
        .stderr(contains("TOML parse error at line 2, column 1"));
}

#[test]
fn error_bad_config_file_gitmoji_boolean_string() {
    let mut cmd = Command::cargo_bin("git-sumi").unwrap();
    cmd.arg("--config")
        .arg("tests/resources/bad_config_gitmoji_boolean_string.toml")
        .arg("duplicate gitmoji")
        .assert()
        .failure()
        .stderr(contains("TOML parse error at line 1"));
}

#[test]
fn success_format_from_config_and_cli_override() {
    let mut cmd = Command::cargo_bin("git-sumi").unwrap();
    cmd.arg("--config")
        .arg("tests/resources/good_config_df_json.toml")
        .arg("Optimise frame rate in Blighttown")
        .assert()
        .success()
        .stdout(contains(
            "\"description\": \"Optimise frame rate in Blighttown\"",
        ));
}

#[test]
fn success_override_format_from_cli() {
    let mut cmd = Command::cargo_bin("git-sumi").unwrap();
    cmd.arg("--config")
        .arg("tests/resources/good_config_df_json.toml")
        .arg("--format")
        .arg("toml")
        .arg("Optimise frame rate in Blighttown")
        .assert()
        .success()
        .stdout(contains(
            "description = \"Optimise frame rate in Blighttown\"",
        ));
}

// End of tests checking for actual config files.

fn get_default_config() -> &'static str {
    r#"# git-sumi ~ configuration file
# Config: https://sumi.rs/docs/configuration
# Rules: https://sumi.rs/docs/rules

# Suppresses progress messages.
quiet = false

# Displays parsed commit message.
display = false

# Sets display format: cli, json, table, toml.
format = "cli"

# Processes each non-empty line as an individual commit.
split_lines = false

# Rule: Include one valid Gitmoji.
# See https://gitmoji.dev/.
gitmoji = false

# Rule: Description must start with the specified case.
# Options: 'any', 'lower', 'upper'.
description_case = "any"

# Rule: Use the imperative mood in the description.
# Example: 'Fix bug' instead of 'Fixed bug'.
imperative = false

# Rule: Do not end commit header with a period.
no_period = false

# Rule: Header length limit.
# A value of 0 disables the rule.
max_header_length = 0

# Rule: Body line length limit.
# A value of 0 disables the rule.
max_body_length = 0

# Rule: No leading, trailing, or consecutive spaces.
whitespace = false

# Rule: Follow Conventional Commits format.
# See https://www.conventionalcommits.org/.
conventional = false

# Rule: List of allowed commit scopes.
# An empty list allows all scopes. Example: ["docs", "cli"].
scopes_allowed = []

# Rule: List of allowed commit types.
# An empty list allows all types. Example: ["feat", "fix", "docs"].
types_allowed = []

# Rule: Header must match regex pattern.
# Example: '^JIRA-\d+:'.
header_pattern = """#
}

#[test]
fn success_init() {
    // If we use run_isolated_git_sumi, parallel tests will read from this dumped config.
    let tmp_dir = tempfile::tempdir().unwrap();
    let tmp_dir_path = tmp_dir.path();

    let mut cmd = Command::cargo_bin("git-sumi").unwrap();
    cmd.current_dir(tmp_dir_path)
        .arg("--init")
        .assert()
        .success();

    let default_path = tmp_dir_path.join("sumi.toml");
    assert!(default_path.exists());

    let file_contents = std::fs::read_to_string(default_path).unwrap();
    assert_eq!(file_contents.trim(), get_default_config().trim());
}

#[test]
fn error_invalid_init_value() {
    let mut cmd = run_isolated_git_sumi("");

    cmd.arg("--init")
        .arg("everything")
        .assert()
        .failure()
        .stderr(contains("Supported options: "));
}

#[test]
fn success_init_config() {
    let tmp_dir = tempfile::tempdir().unwrap();
    let tmp_dir_path = tmp_dir.path();

    let mut cmd = Command::cargo_bin("git-sumi").unwrap();
    cmd.current_dir(tmp_dir_path)
        .arg("--init")
        .arg("config")
        .assert()
        .success();

    let default_path = tmp_dir_path.join("sumi.toml");
    assert!(default_path.exists());

    let file_contents = std::fs::read_to_string(default_path).unwrap();
    assert_eq!(file_contents.trim(), get_default_config().trim());
}

#[test]
fn success_config_overwrite_no() {
    let tmp_dir = tempfile::tempdir().unwrap();
    let tmp_dir_path = tmp_dir.path();

    // Create an initial sumi.toml with custom content
    let initial_config_content = r#"# Initial configuration
quiet = true
"#;
    let config_path = tmp_dir_path.join("sumi.toml");
    std::fs::write(&config_path, initial_config_content).unwrap();

    // Attempt to initialize the default config, simulating 'no' to the overwrite prompt.
    let mut cmd = Command::cargo_bin("git-sumi").unwrap();
    cmd.current_dir(tmp_dir_path)
        .arg("--init")
        .write_stdin("n\n")
        .assert()
        .stdout(contains("Overwrite? (y/n) [n] "))
        .success();

    let file_contents = std::fs::read_to_string(config_path).unwrap();
    assert_eq!(
        file_contents, initial_config_content,
        "The configuration file should not have been overwritten."
    );
}

#[test]
fn success_config_overwrite_yes() {
    let tmp_dir = tempfile::tempdir().unwrap();
    let tmp_dir_path = tmp_dir.path();

    let initial_config_content = r#"# Initial configuration
quiet = true
"#;
    let config_path = tmp_dir_path.join("sumi.toml");
    std::fs::write(&config_path, initial_config_content).unwrap();

    // Attempt to initialize the default config, simulating 'yes' to the overwrite prompt.
    let mut cmd = Command::cargo_bin("git-sumi").unwrap();
    cmd.current_dir(tmp_dir_path)
        .arg("--init")
        .write_stdin("y\n")
        .assert()
        .stdout(contains("Overwrite? (y/n) [n] "))
        .success();

    let file_contents = std::fs::read_to_string(config_path).unwrap();
    let expected_default_config = get_default_config().trim();

    assert_eq!(
        file_contents.trim(),
        expected_default_config,
        "The configuration file should have been overwritten with the default configuration."
    );
}

fn init_tmp_git_repo() -> tempfile::TempDir {
    let tmp_dir = tempfile::tempdir().unwrap();
    let tmp_dir_path = tmp_dir.path();

    // Initialise a git repository
    let output = std::process::Command::new("git")
        .args(["init"])
        .current_dir(tmp_dir_path)
        .output()
        .expect("Failed to execute git init");

    assert!(output.status.success());

    tmp_dir
}

#[test]
fn success_init_hook() {
    let tmp_dir = init_tmp_git_repo();
    let tmp_dir_path = tmp_dir.path();
    let mut cmd = Command::cargo_bin("git-sumi").unwrap();
    cmd.current_dir(tmp_dir_path)
        .arg("--init")
        .arg("commit-msg")
        .assert()
        .success();

    let hook_path = tmp_dir_path.join(".git/hooks/commit-msg");
    assert!(hook_path.exists());

    let file_contents = std::fs::read_to_string(hook_path).unwrap();
    assert!(file_contents.contains("git-sumi"));
}

#[test]
fn error_init_hook_in_non_git_repo() {
    let tmp_dir = tempfile::tempdir().unwrap();
    let tmp_dir_path = tmp_dir.path();
    let mut cmd = Command::cargo_bin("git-sumi").unwrap();
    cmd.current_dir(tmp_dir_path)
        .arg("--init")
        .arg("commit-msg")
        .assert()
        .failure()
        .stderr(contains("No .git directory found"));
}

#[test]
fn success_init_hook_overwrite_yes() {
    let tmp_dir = init_tmp_git_repo();
    let tmp_dir_path = tmp_dir.path();
    let hooks_dir = tmp_dir_path.join(".git/hooks");
    fs::create_dir_all(&hooks_dir).unwrap(); // Ensure the hooks directory exists.
    let hook_path = hooks_dir.join("commit-msg");
    fs::write(&hook_path, "#!/bin/sh\necho \"Existing hook\"").unwrap(); // Create a dummy hook.

    let mut cmd = Command::cargo_bin("git-sumi").unwrap();
    cmd.current_dir(tmp_dir_path)
        .arg("--init")
        .arg("commit-msg")
        .write_stdin("y\n") // Simulate user typing 'y' and pressing Enter.
        .assert()
        .success()
        .stdout(contains("Overwrite? (y/n) [n]")); // Verify the prompt is displayed

    let hook_content = fs::read_to_string(hook_path).expect("Failed to read hook file");
    assert!(
        hook_content.contains("Commit-msg hook generated by git-sumi"),
        "The commit-msg hook does not contain the expected content."
    );
}

#[test]
fn success_init_hook_overwrite_no() {
    let tmp_dir = init_tmp_git_repo();
    let tmp_dir_path = tmp_dir.path();
    let hooks_dir = tmp_dir_path.join(".git/hooks");
    fs::create_dir_all(&hooks_dir).unwrap();
    let original_hook_content = "#!/bin/sh\necho \"Existing hook\"";
    let hook_path = hooks_dir.join("commit-msg");
    fs::write(&hook_path, original_hook_content).unwrap(); // Create a dummy hook.

    let mut cmd = Command::cargo_bin("git-sumi").unwrap();
    cmd.current_dir(tmp_dir_path)
        .arg("--init")
        .arg("commit-msg")
        .write_stdin("n\n") // Simulate user typing 'n' and pressing Enter.
        .assert()
        .success()
        .stdout(contains("Overwrite? (y/n) [n] "));

    let hook_content = fs::read_to_string(&hook_path).expect("Failed to read hook file");
    assert_eq!(
        hook_content, original_hook_content,
        "The commit-msg hook content should remain unchanged."
    );
    assert!(
        !hook_content.contains("Commit-msg hook generated by git-sumi"),
        "The commit-msg hook should not contain the default content generated by git-sumi."
    );
}

#[test]
fn success_prepare_commit_message_stdout() {
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("-C")
        .arg("-H")
        .arg("42")
        .arg("--prepare-commit-message")
        .assert()
        .success()
        .stdout(contains("# Header length limit: 42"))
        .stdout(contains("# Follow Conventional Commits format"));
}

#[test]
fn success_prepare_commit_msg_hook_creation() {
    let tmp_dir = init_tmp_git_repo();
    let tmp_dir_path = tmp_dir.path();

    let mut cmd = Command::cargo_bin("git-sumi").unwrap();
    cmd.current_dir(tmp_dir_path)
        .arg("--init")
        .arg("prepare-commit-msg")
        .assert()
        .success();

    let hook_path = tmp_dir_path.join(".git/hooks/prepare-commit-msg");
    assert!(
        hook_path.exists(),
        "The prepare-commit-msg hook file should be created."
    );

    let file_contents = fs::read_to_string(hook_path).unwrap();
    assert!(
        file_contents.contains("# Prepend the rules to the commit message template"),
        "The prepare-commit-msg file should contain the hook script."
    );
}

const PREPARE_COMMIT_MSG_HOOK: &str = r#"#!/usr/bin/env bash
set -euo pipefail

COMMIT_MSG_FILE="${1}"
COMMIT_SOURCE="${2:-}"

# Do nothing if the message was created with `git commit -m`.
if [ "${COMMIT_SOURCE}" = "message" ]; then
    exit 0
fi

CUSTOM_MSG="$(git-sumi --prepare-commit-message)"

# Prepend the rules to the commit message template.
TEMP_FILE="$(mktemp)"
echo "${CUSTOM_MSG}" > "${TEMP_FILE}"
cat "${COMMIT_MSG_FILE}" >> "${TEMP_FILE}"
mv "${TEMP_FILE}" "${COMMIT_MSG_FILE}"
"#;

#[test]
fn success_prepare_commit_msg_hook_overwrite_yes() {
    let tmp_dir = tempdir().unwrap();
    let tmp_dir_path = tmp_dir.path();

    let initial_hook_content = r#"#!/bin/bash
# Sample hook script
"#;
    let hook_path = tmp_dir_path.join(".git/hooks/prepare-commit-msg");
    fs::create_dir_all(hook_path.parent().unwrap()).unwrap();
    fs::write(&hook_path, initial_hook_content).unwrap();

    let mut cmd = Command::cargo_bin("git-sumi").unwrap();
    cmd.current_dir(tmp_dir_path)
        .arg("--init")
        .arg("prepare-commit-msg")
        .write_stdin("y\n") // Simulating 'yes' to overwrite prompt.
        .assert()
        .success();

    let expected_hook_content = PREPARE_COMMIT_MSG_HOOK;

    let file_contents = fs::read_to_string(hook_path).unwrap();
    assert_eq!(
        file_contents, expected_hook_content,
        "The prepare-commit-msg hook should have been overwritten with the new content."
    );
}

#[test]
fn success_prepare_commit_msg_hook_overwrite_no() {
    let tmp_dir = tempdir().unwrap();
    let tmp_dir_path = tmp_dir.path();

    let initial_hook_content = r#"#!/bin/bash
# Sample hook script
"#;
    let hook_path = tmp_dir_path.join(".git/hooks/prepare-commit-msg");
    fs::create_dir_all(hook_path.parent().unwrap()).unwrap();
    fs::write(&hook_path, initial_hook_content).unwrap();

    let mut cmd = Command::cargo_bin("git-sumi").unwrap();
    cmd.current_dir(tmp_dir_path)
        .arg("--init")
        .arg("prepare-commit-msg")
        .write_stdin("n\n") // Simulating 'no' to overwrite prompt.
        .assert()
        .success();

    let file_contents = fs::read_to_string(hook_path).unwrap();
    assert_eq!(
        file_contents, initial_hook_content,
        "The prepare-commit-msg hook should not have been overwritten."
    );
}

#[test]
fn success_initialize_all_hooks() {
    let tmp_dir = init_tmp_git_repo();
    let tmp_dir_path = tmp_dir.path();

    let mut cmd = Command::cargo_bin("git-sumi").unwrap();
    cmd.current_dir(tmp_dir_path)
        .arg("--init")
        .arg("hooks")
        .assert()
        .success();

    let hooks_dir = tmp_dir_path.join(".git/hooks");
    let hook_files = fs::read_dir(hooks_dir).unwrap();
    let hook_file_names: Vec<String> = hook_files
        .map(|entry| entry.unwrap().file_name().into_string().unwrap())
        .collect();

    assert!(
        hook_file_names.contains(&"commit-msg".to_string()),
        "The commit-msg hook should be created."
    );
    assert!(
        hook_file_names.contains(&"prepare-commit-msg".to_string()),
        "The prepare-commit-msg hook should be created."
    );
}
