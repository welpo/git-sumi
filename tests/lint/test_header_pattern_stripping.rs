extern crate tempfile;

use super::contains;
use super::run_isolated_git_sumi;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_header_pattern_with_strip_disabled_backwards_compatibility() {
    // Test backwards compatibility - old behaviour should be preserved
    let config = r#"
header_pattern = "^JIRA-\\d+ "
whitespace = true
imperative = true
"#;
    let tmp_dir = tempdir().unwrap();
    let config_path = tmp_dir.path().join("sumi.toml");
    fs::write(&config_path, config).unwrap();
    // With strip_header_pattern disabled (default), this should pass
    // because whitespace and imperative checks run on full message
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("--config")
        .arg(config_path)
        .arg("JIRA-123 fixed the bug")
        .assert()
        .success();
}

#[test]
fn test_header_pattern_with_stripping_enabled_imperative_fails() {
    let config = r#"
header_pattern = "^JIRA-\\d+ "
whitespace = true
imperative = true
strip_header_pattern = true
"#;
    let tmp_dir = tempdir().unwrap();
    let config_path = tmp_dir.path().join("sumi.toml");
    fs::write(&config_path, config).unwrap();
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("--config")
        .arg(config_path)
        .arg("JIRA-123 fixed the bug")
        .assert()
        .failure()
        .stderr(contains("non-imperative verb: 'fixed'"));
}

#[test]
fn test_header_pattern_with_stripping_enabled_whitespace_fails() {
    let config = r#"
header_pattern = "^JIRA-\\d+ "
whitespace = true
strip_header_pattern = true
"#;
    let tmp_dir = tempdir().unwrap();
    let config_path = tmp_dir.path().join("sumi.toml");
    fs::write(&config_path, config).unwrap();
    // With stripping enabled, extra spaces after pattern removal should fail
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("--config")
        .arg(config_path)
        .arg("JIRA-123  fixed the bug") // double space after JIRA-123
        .assert()
        .failure()
        .stderr(contains("Leading space"));
}

#[test]
fn test_header_pattern_imperative_validation_after_stripping() {
    let config = r#"
header_pattern = "^JIRA-\\d+ "
imperative = true
strip_header_pattern = true
"#;
    let tmp_dir = tempdir().unwrap();
    let config_path = tmp_dir.path().join("sumi.toml");
    fs::write(&config_path, config).unwrap();
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("--config")
        .arg(config_path)
        .arg("JIRA-123 fixed the bug")
        .assert()
        .failure()
        .stderr(contains("non-imperative verb: 'fixed'"));
}

#[test]
fn test_header_pattern_with_stripping_enabled_valid_case() {
    let config = r#"
header_pattern = "^JIRA-\\d+ "
whitespace = true
imperative = true
strip_header_pattern = true
"#;
    let tmp_dir = tempdir().unwrap();
    let config_path = tmp_dir.path().join("sumi.toml");
    fs::write(&config_path, config).unwrap();
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("--config")
        .arg(config_path)
        .arg("JIRA-123 fix the bug")
        .assert()
        .success();
}

#[test]
fn test_header_pattern_validation_works() {
    let config = r#"
header_pattern = "^JIRA-\\d+ "
"#;
    let tmp_dir = tempdir().unwrap();
    let config_path = tmp_dir.path().join("sumi.toml");
    fs::write(&config_path, config).unwrap();
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("--config")
        .arg(config_path)
        .arg("XYZ-123 fix the bug")
        .assert()
        .failure()
        .stderr(contains("Header does not match the required pattern"));
}

#[test]
fn test_strip_with_no_header_pattern_does_nothing() {
    let config = r#"
whitespace = true
imperative = true
strip_header_pattern = true
"#;
    let tmp_dir = tempdir().unwrap();
    let config_path = tmp_dir.path().join("sumi.toml");
    fs::write(&config_path, config).unwrap();
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("--config")
        .arg(config_path)
        .arg("  fixed the bug")
        .assert()
        .failure()
        .stderr(contains("Leading space"));
}

#[test]
fn test_header_pattern_with_multiline_commit() {
    let config = r#"
header_pattern = "^JIRA-\\d+ "
whitespace = true
strip_header_pattern = true
"#;
    let tmp_dir = tempdir().unwrap();
    let config_path = tmp_dir.path().join("sumi.toml");
    fs::write(&config_path, config).unwrap();
    let commit_msg =
        "JIRA-123  fix the bug\n\nThis is a detailed description\nwith multiple lines.";
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("--config")
        .arg(config_path)
        .arg(commit_msg)
        .assert()
        .failure()
        .stderr(contains("Leading space"));
}

#[test]
fn test_header_length_validation_uses_full_header_with_pattern() {
    let config = r#"
header_pattern = "^JIRA-\\d+ "
max_header_length = 20
strip_header_pattern = true
"#;
    let tmp_dir = tempdir().unwrap();
    let config_path = tmp_dir.path().join("sumi.toml");
    fs::write(&config_path, config).unwrap();
    // Full header is 25 chars: "JIRA-123 fix the bug here"
    // Should fail length check on full header even though stripped version would be shorter
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("--config")
        .arg(config_path)
        .arg("JIRA-123 fix the bug here")
        .assert()
        .failure()
        .stderr(contains("too long"));
}

#[test]
fn test_header_length_validation_passes_when_full_header_within_limit() {
    let config = r#"
header_pattern = "^JIRA-\\d+ "
max_header_length = 30
imperative = true
strip_header_pattern = true
"#;
    let tmp_dir = tempdir().unwrap();
    let config_path = tmp_dir.path().join("sumi.toml");
    fs::write(&config_path, config).unwrap();
    // Full header is 18 chars: "JIRA-123 fix bug" - should pass length
    // Stripped version "fix bug" should pass imperative
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("--config")
        .arg(config_path)
        .arg("JIRA-123 fix bug")
        .assert()
        .success();
}
