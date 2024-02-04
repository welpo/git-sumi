mod test_combined_rules;
mod test_commit_changes;
mod test_config;
mod test_conventional_commits;
mod test_display;
mod test_gitmoji;
mod test_single_rule;

use super::contains;
use super::run_isolated_git_sumi;
use super::Command;

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
