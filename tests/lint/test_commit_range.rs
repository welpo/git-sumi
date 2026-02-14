extern crate assert_cmd;
extern crate tempfile;

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

    Command::new("git")
        .args(["init"])
        .current_dir(repo_dir)
        .assert()
        .success();

    Command::new("git")
        .args(["config", "commit.gpgsign", "false"])
        .current_dir(repo_dir)
        .assert()
        .success();

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

fn create_commit(repo_dir: &Path, file_name: &str, message: &str) {
    let file_path = repo_dir.join(file_name);
    let mut file = File::create(file_path).expect("Failed to create a file");
    writeln!(file, "content for {file_name}").expect("Failed to write to a file");
    drop(file);

    Command::new("git")
        .args(["add", file_name])
        .current_dir(repo_dir)
        .assert()
        .success();

    Command::new("git")
        .args(["commit", "-m", message])
        .current_dir(repo_dir)
        .assert()
        .success();
}

#[test]
fn success_lint_range_all_valid() {
    let tmp_dir = setup_git_repo();
    let repo_dir = tmp_dir.path();

    create_commit(repo_dir, "init.txt", "feat: initial commit");
    create_commit(repo_dir, "a.txt", "feat: add feature a");
    create_commit(repo_dir, "b.txt", "fix: fix bug b");
    create_commit(repo_dir, "c.txt", "docs: update docs");

    let mut cmd = run_isolated_git_sumi("");
    cmd.current_dir(repo_dir)
        .args(["--from", "HEAD~3", "--to", "HEAD", "-C"])
        .assert()
        .success();
}

#[test]
fn error_lint_range_some_invalid() {
    let tmp_dir = setup_git_repo();
    let repo_dir = tmp_dir.path();

    create_commit(repo_dir, "init.txt", "feat: initial commit");
    create_commit(repo_dir, "a.txt", "feat: add feature a");
    create_commit(repo_dir, "b.txt", "not a conventional commit");
    create_commit(repo_dir, "c.txt", "also bad");

    let mut cmd = run_isolated_git_sumi("");
    cmd.current_dir(repo_dir)
        .args(["--from", "HEAD~3", "--to", "HEAD", "-C"])
        .assert()
        .failure()
        .stderr(contains("2 out of 3 commits failed linting"));
}

#[test]
fn success_empty_range() {
    let tmp_dir = setup_git_repo();
    let repo_dir = tmp_dir.path();

    create_commit(repo_dir, "a.txt", "feat: initial");

    let mut cmd = run_isolated_git_sumi("");
    cmd.current_dir(repo_dir)
        .args(["--from", "HEAD", "--to", "HEAD", "-C"])
        .assert()
        .success()
        .stdout(contains("No commits found in range"));
}

#[test]
fn error_invalid_revision() {
    let tmp_dir = setup_git_repo();
    let repo_dir = tmp_dir.path();

    create_commit(repo_dir, "a.txt", "feat: initial");

    let mut cmd = run_isolated_git_sumi("");
    cmd.current_dir(repo_dir)
        .args(["--from", "nonexistent", "--to", "HEAD", "-C"])
        .assert()
        .failure()
        .stderr(contains("Failed to get commits in range"));
}

#[test]
fn error_to_without_from() {
    let mut cmd = run_isolated_git_sumi("");
    cmd.args(["--to", "HEAD", "-C"])
        .assert()
        .failure()
        .stderr(contains("required"));
}

#[test]
fn error_from_without_to() {
    let mut cmd = run_isolated_git_sumi("");
    cmd.args(["--from", "HEAD", "-C"])
        .assert()
        .failure()
        .stderr(contains("required"));
}

#[test]
fn error_conflicts_with_positional() {
    let mut cmd = run_isolated_git_sumi("");
    cmd.args(["--from", "HEAD~1", "--to", "HEAD", "some message"])
        .assert()
        .failure()
        .stderr(contains("cannot be used with"));
}

#[test]
fn error_empty_from_value() {
    let mut cmd = run_isolated_git_sumi("");
    cmd.args(["--from", "", "--to", "HEAD", "-C"])
        .assert()
        .failure()
        .stderr(contains("value must not be empty"));
}

#[test]
fn error_empty_to_value() {
    let mut cmd = run_isolated_git_sumi("");
    cmd.args(["--from", "HEAD", "--to", "", "-C"])
        .assert()
        .failure()
        .stderr(contains("value must not be empty"));
}

#[test]
fn error_whitespace_only_from_value() {
    let mut cmd = run_isolated_git_sumi("");
    cmd.args(["--from", "  ", "--to", "HEAD", "-C"])
        .assert()
        .failure()
        .stderr(contains("value must not be empty"));
}

#[test]
fn error_conflicts_with_file_flag() {
    let mut cmd = run_isolated_git_sumi("");
    cmd.args(["--from", "HEAD~1", "--to", "HEAD", "--file", "msg.txt"])
        .assert()
        .failure()
        .stderr(contains("cannot be used with"));
}

/// Each commit violates a different rule: non-conventional, non-imperative, ends with period.
#[test]
fn error_each_commit_violates_different_rule() {
    let tmp_dir = setup_git_repo();
    let repo_dir = tmp_dir.path();

    create_commit(repo_dir, "init.txt", "feat: add init");
    // Not conventional.
    create_commit(repo_dir, "a.txt", "this is not conventional");
    // Conventional but non-imperative verb.
    create_commit(repo_dir, "b.txt", "fix: fixed the parser");
    // Conventional but ends with a period.
    create_commit(repo_dir, "c.txt", "feat: add new feature.");

    let output = run_isolated_git_sumi("")
        .current_dir(repo_dir)
        .args(["--from", "HEAD~3", "--to", "HEAD", "-C", "-I", "-P"])
        .output()
        .unwrap();

    assert!(!output.status.success());

    let stderr = String::from_utf8(output.stderr).unwrap();
    // Each commit triggers its own error type.
    assert!(
        stderr.contains("Failed to parse as a conventional commit"),
        "Expected conventional parse error in stderr"
    );
    assert!(
        stderr.contains("non-imperative verb"),
        "Expected imperative error in stderr"
    );
    assert!(
        stderr.contains("Header must not end with a period"),
        "Expected period error in stderr"
    );
    assert!(
        stderr.contains("3 out of 3 commits failed linting"),
        "Expected final summary in stderr"
    );
}

/// Mix of passing and failing commits — only failures appear in output.
#[test]
fn error_mixed_valid_and_invalid_with_multiple_rules() {
    let tmp_dir = setup_git_repo();
    let repo_dir = tmp_dir.path();

    create_commit(repo_dir, "init.txt", "feat: add init");
    create_commit(repo_dir, "a.txt", "feat: add feature a"); // valid
    create_commit(repo_dir, "b.txt", "Added something wrong"); // non-conventional + non-imperative
    create_commit(repo_dir, "c.txt", "fix: correct the build"); // valid
    create_commit(repo_dir, "d.txt", "docs: update readme."); // ends with period

    let output = run_isolated_git_sumi("")
        .current_dir(repo_dir)
        .args(["--from", "HEAD~4", "--to", "HEAD", "-C", "-I", "-P"])
        .output()
        .unwrap();

    assert!(!output.status.success());

    let stderr = String::from_utf8(output.stderr).unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    // Valid commits show success on stdout.
    assert!(
        stdout.contains("All 3 checks passed"),
        "Expected success messages for valid commits on stdout"
    );

    // Invalid commits show their specific errors.
    assert!(
        stderr.contains("Failed to parse as a conventional commit"),
        "Expected conventional parse error"
    );
    assert!(
        stderr.contains("Header must not end with a period"),
        "Expected period error"
    );
    assert!(
        stderr.contains("2 out of 4 commits failed linting"),
        "Expected correct summary count"
    );
}

/// Header length violation caught in range mode.
#[test]
fn error_header_too_long_in_range() {
    let tmp_dir = setup_git_repo();
    let repo_dir = tmp_dir.path();

    create_commit(repo_dir, "init.txt", "feat: init");
    create_commit(
        repo_dir,
        "a.txt",
        "feat: this commit message header is way too long for any reasonable limit we set",
    );

    let output = run_isolated_git_sumi("")
        .current_dir(repo_dir)
        .args([
            "--from",
            "HEAD~1",
            "--to",
            "HEAD",
            "-C",
            "--max-header-length",
            "50",
        ])
        .output()
        .unwrap();

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("is too long"),
        "Expected line-too-long error"
    );
}

/// Whitespace violations in range mode.
#[test]
fn error_whitespace_violations_in_range() {
    let tmp_dir = setup_git_repo();
    let repo_dir = tmp_dir.path();

    create_commit(repo_dir, "init.txt", "feat: init");
    // Git strips trailing whitespace from commit messages, so use double spaces instead.
    create_commit(repo_dir, "a.txt", "feat:  double space");
    create_commit(repo_dir, "b.txt", "fix:  another  double space");

    let output = run_isolated_git_sumi("")
        .current_dir(repo_dir)
        .args(["--from", "HEAD~2", "--to", "HEAD", "-C", "-W"])
        .output()
        .unwrap();

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("Whitespace"),
        "Expected whitespace error in stderr"
    );
    assert!(
        stderr.contains("2 out of 2 commits failed linting"),
        "Expected both commits to fail"
    );
}

/// Description case violation: uppercase required but lowercase given.
#[test]
fn error_description_case_in_range() {
    let tmp_dir = setup_git_repo();
    let repo_dir = tmp_dir.path();

    create_commit(repo_dir, "init.txt", "feat: Init");
    create_commit(repo_dir, "a.txt", "feat: add lowercase start");

    let output = run_isolated_git_sumi("")
        .current_dir(repo_dir)
        .args(["--from", "HEAD~1", "--to", "HEAD", "-C", "-E", "upper"])
        .output()
        .unwrap();

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("Description must start with a capital letter"),
        "Expected uppercase case error"
    );
}

/// Allowed types restriction catches forbidden type.
#[test]
fn error_disallowed_type_in_range() {
    let tmp_dir = setup_git_repo();
    let repo_dir = tmp_dir.path();

    create_commit(repo_dir, "init.txt", "feat: init");
    create_commit(repo_dir, "a.txt", "feat: add feature");
    create_commit(repo_dir, "b.txt", "chore: update deps");

    let output = run_isolated_git_sumi("")
        .current_dir(repo_dir)
        .args(["--from", "HEAD~2", "--to", "HEAD", "-T", "feat,fix,docs"])
        .output()
        .unwrap();

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("Invalid commit type 'chore'"),
        "Expected disallowed type error"
    );
    assert!(
        stderr.contains("1 out of 2 commits failed linting"),
        "Expected plural 'commits' in summary"
    );
}

/// Allowed scopes restriction catches forbidden scope.
#[test]
fn error_disallowed_scope_in_range() {
    let tmp_dir = setup_git_repo();
    let repo_dir = tmp_dir.path();

    create_commit(repo_dir, "init.txt", "feat(core): init");
    create_commit(repo_dir, "a.txt", "feat(core): add feature");
    create_commit(repo_dir, "b.txt", "fix(ui): fix button");

    let output = run_isolated_git_sumi("")
        .current_dir(repo_dir)
        .args(["--from", "HEAD~2", "--to", "HEAD", "-S", "core,api"])
        .output()
        .unwrap();

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("Invalid commit scope 'ui'"),
        "Expected disallowed scope error"
    );
}

/// Header pattern regex mismatch.
#[test]
fn error_header_pattern_mismatch_in_range() {
    let tmp_dir = setup_git_repo();
    let repo_dir = tmp_dir.path();

    create_commit(repo_dir, "init.txt", "PROJ-1 init");
    create_commit(repo_dir, "a.txt", "PROJ-42 add feature");
    create_commit(repo_dir, "b.txt", "no ticket prefix here");

    let output = run_isolated_git_sumi("")
        .current_dir(repo_dir)
        .args(["--from", "HEAD~2", "--to", "HEAD", "-R", "^PROJ-\\d+"])
        .output()
        .unwrap();

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("Header does not match the required pattern"),
        "Expected header pattern error"
    );
    assert!(
        stderr.contains("1 out of 2 commits failed linting"),
        "Expected one failure"
    );
}

/// Single commit in range — passes.
#[test]
fn success_single_commit_range() {
    let tmp_dir = setup_git_repo();
    let repo_dir = tmp_dir.path();

    create_commit(repo_dir, "init.txt", "feat: init");
    create_commit(repo_dir, "a.txt", "fix: correct the build");

    let mut cmd = run_isolated_git_sumi("");
    cmd.current_dir(repo_dir)
        .args(["--from", "HEAD~1", "--to", "HEAD", "-C", "-I", "-P", "-W"])
        .assert()
        .success();
}

/// Single commit in range — fails multiple rules at once.
#[test]
fn error_single_commit_multiple_violations() {
    let tmp_dir = setup_git_repo();
    let repo_dir = tmp_dir.path();

    create_commit(repo_dir, "init.txt", "feat: init");
    // Non-imperative + period + uppercase required but lowercase.
    create_commit(repo_dir, "a.txt", "feat: fixed the thing.");

    let output = run_isolated_git_sumi("")
        .current_dir(repo_dir)
        .args([
            "--from", "HEAD~1", "--to", "HEAD", "-C", "-I", "-P", "-E", "upper",
        ])
        .output()
        .unwrap();

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("non-imperative verb"),
        "Expected imperative error"
    );
    assert!(
        stderr.contains("Header must not end with a period"),
        "Expected period error"
    );
    assert!(
        stderr.contains("Description must start with a capital letter"),
        "Expected case error"
    );
}

/// All commits in range are valid under strict combined rules.
#[test]
fn success_all_rules_pass_strict() {
    let tmp_dir = setup_git_repo();
    let repo_dir = tmp_dir.path();

    create_commit(repo_dir, "init.txt", "feat: add init");
    create_commit(repo_dir, "a.txt", "feat(core): add parser");
    create_commit(repo_dir, "b.txt", "fix(api): handle timeout");
    create_commit(repo_dir, "c.txt", "docs: update changelog");

    let mut cmd = run_isolated_git_sumi("");
    cmd.current_dir(repo_dir)
        .args([
            "--from",
            "HEAD~3",
            "--to",
            "HEAD",
            "-C",
            "-I",
            "-P",
            "-W",
            "-E",
            "lower",
            "--max-header-length",
            "72",
        ])
        .assert()
        .success();
}

/// Range with quiet flag suppresses per-commit success output.
#[test]
fn success_quiet_mode_suppresses_output() {
    let tmp_dir = setup_git_repo();
    let repo_dir = tmp_dir.path();

    create_commit(repo_dir, "init.txt", "feat: add init");
    create_commit(repo_dir, "a.txt", "fix: correct bug");

    let output = run_isolated_git_sumi("")
        .current_dir(repo_dir)
        .args(["--from", "HEAD~1", "--to", "HEAD", "-C", "-q"])
        .output()
        .unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        !stdout.contains("checks passed"),
        "Quiet mode should suppress success messages"
    );
}

/// Multiline commit messages (with body) are handled correctly in range mode.
#[test]
fn success_multiline_commits_in_range() {
    let tmp_dir = setup_git_repo();
    let repo_dir = tmp_dir.path();

    create_commit(repo_dir, "init.txt", "feat: add init");
    create_commit(
        repo_dir,
        "a.txt",
        "feat: add feature\n\nThis is the body of the commit.",
    );
    create_commit(
        repo_dir,
        "b.txt",
        "fix: correct bug\n\nDetailed explanation here.",
    );

    let mut cmd = run_isolated_git_sumi("");
    cmd.current_dir(repo_dir)
        .args(["--from", "HEAD~2", "--to", "HEAD", "-C", "-W"])
        .assert()
        .success();
}

/// Body line length violation caught in range mode.
#[test]
fn error_body_too_long_in_range() {
    let tmp_dir = setup_git_repo();
    let repo_dir = tmp_dir.path();

    let long_body = "a".repeat(100);
    let message = format!("feat: add feature\n\n{long_body}");

    create_commit(repo_dir, "init.txt", "feat: add init");
    create_commit(repo_dir, "a.txt", &message);

    let output = run_isolated_git_sumi("")
        .current_dir(repo_dir)
        .args([
            "--from",
            "HEAD~1",
            "--to",
            "HEAD",
            "-C",
            "--max-body-length",
            "72",
        ])
        .output()
        .unwrap();

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("is too long"),
        "Expected body line-too-long error"
    );
}

#[test]
fn error_commit_flag_conflicts_with_range() {
    let tmp_dir = setup_git_repo();
    let repo_dir = tmp_dir.path();

    create_commit(repo_dir, "init.txt", "feat: initial commit");
    create_commit(repo_dir, "a.txt", "feat: add feature a");

    let mut cmd = run_isolated_git_sumi("");
    cmd.current_dir(repo_dir)
        .args(["--from", "HEAD~1", "--to", "HEAD", "-C", "--commit"])
        .assert()
        .failure()
        .stderr(contains("cannot be used with"));
}

#[test]
fn error_force_flag_conflicts_with_range() {
    let tmp_dir = setup_git_repo();
    let repo_dir = tmp_dir.path();

    create_commit(repo_dir, "init.txt", "feat: initial commit");
    create_commit(repo_dir, "a.txt", "feat: add feature a");

    let mut cmd = run_isolated_git_sumi("");
    cmd.current_dir(repo_dir)
        .args(["--from", "HEAD~1", "--to", "HEAD", "-C", "--force"])
        .assert()
        .failure()
        .stderr(contains("cannot be used with"));
}

/// `--split-lines` in range mode should lint each non-empty line of
/// each commit message independently, just like it does for single commits.
#[test]
fn success_split_lines_in_range_all_valid() {
    let tmp_dir = setup_git_repo();
    let repo_dir = tmp_dir.path();

    create_commit(repo_dir, "init.txt", "feat: init");
    create_commit(
        repo_dir,
        "a.txt",
        "feat: add feature a\n\nfix: correct typo",
    );
    create_commit(repo_dir, "b.txt", "docs: update readme\n\nchore: clean up");

    let mut cmd = run_isolated_git_sumi("");
    cmd.current_dir(repo_dir)
        .args(["--from", "HEAD~2", "--to", "HEAD", "-C", "--split-lines"])
        .assert()
        .success();
}

/// With `--split-lines`, a commit whose body contains a non-conventional line
/// should fail, because each non-empty line is validated independently.
#[test]
fn error_split_lines_in_range_catches_invalid_body_line() {
    let tmp_dir = setup_git_repo();
    let repo_dir = tmp_dir.path();

    create_commit(repo_dir, "init.txt", "feat: init");
    // The header is valid, but the body is not a conventional commit.
    create_commit(
        repo_dir,
        "a.txt",
        "feat: add feature\n\nnot conventional at all",
    );

    let mut cmd = run_isolated_git_sumi("");
    cmd.current_dir(repo_dir)
        .args(["--from", "HEAD~1", "--to", "HEAD", "-C", "--split-lines"])
        .assert()
        .failure()
        .stderr(contains("Failed to parse as a conventional commit"));
}

/// With `--split-lines`, when some commits have invalid lines and others are
/// fully valid, the summary should report the correct commit failure count
/// and include both the per-line and per-range error messages.
#[test]
fn error_split_lines_in_range_mixed_commits() {
    let tmp_dir = setup_git_repo();
    let repo_dir = tmp_dir.path();

    create_commit(repo_dir, "init.txt", "feat: init");
    // All lines valid.
    create_commit(repo_dir, "a.txt", "feat: add feature\n\nfix: correct typo");
    // Second line is not conventional.
    create_commit(
        repo_dir,
        "b.txt",
        "docs: update readme\n\nnot conventional at all",
    );

    let output = run_isolated_git_sumi("")
        .current_dir(repo_dir)
        .args(["--from", "HEAD~2", "--to", "HEAD", "-C", "--split-lines"])
        .output()
        .unwrap();

    assert!(!output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();
    // The per-commit SplitLinesErrors is logged via error!() to stdout.
    assert!(
        stdout.contains("1 out of 2 lines failed linting"),
        "Expected per-commit split-lines summary on stdout, got: {stdout}"
    );
    // The range-level CommitRangeErrors is printed via eprintln in main().
    assert!(
        stderr.contains("1 out of 2 commits failed linting"),
        "Expected range-level summary on stderr, got: {stderr}"
    );
}

#[test]
fn error_split_lines_quiet_in_range_shows_errors() {
    let tmp_dir = setup_git_repo();
    let repo_dir = tmp_dir.path();

    create_commit(repo_dir, "init.txt", "feat: init");
    create_commit(repo_dir, "a.txt", "feat: add feature\n\nnot conventional");

    let output = run_isolated_git_sumi("")
        .current_dir(repo_dir)
        .args([
            "--from",
            "HEAD~1",
            "--to",
            "HEAD",
            "-C",
            "--split-lines",
            "-q",
        ])
        .output()
        .unwrap();

    assert!(!output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        !stdout.contains("checks passed"),
        "Quiet mode should suppress success messages, got stdout: {stdout}"
    );
    assert!(
        stderr.contains("Failed to parse as a conventional commit"),
        "Errors should still appear in quiet mode, got stderr: {stderr}"
    );
}

/// `--display` in range mode should output parsed commit info for passing commits.
#[test]
fn success_display_in_range_mode() {
    let tmp_dir = setup_git_repo();
    let repo_dir = tmp_dir.path();

    create_commit(repo_dir, "init.txt", "feat: init");
    create_commit(repo_dir, "a.txt", "feat: add parser");

    let output = run_isolated_git_sumi("")
        .current_dir(repo_dir)
        .args(["--from", "HEAD~1", "--to", "HEAD", "-C", "--display"])
        .output()
        .unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains("add parser"),
        "Expected display output for the passing commit, got stdout: {stdout}"
    );
}
