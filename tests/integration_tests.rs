use std::sync::LazyLock;

use assert_cmd::{cargo::cargo_bin_cmd, Command};
use predicates::str::contains;
use tempfile::tempdir;

#[cfg(test)]
mod lint;

static TEST_DIR: LazyLock<tempfile::TempDir> =
    LazyLock::new(|| tempdir().expect("Failed to create temporary directory"));

fn run_isolated_git_sumi(subcommand: &str) -> Command {
    let mut cmd = cargo_bin_cmd!();
    if !subcommand.is_empty() {
        cmd.arg(subcommand);
    }
    cmd.current_dir(TEST_DIR.path()); // Change dir to avoid loading the project's config.
    cmd
}

#[test]
fn success_shell_completion_generation() {
    for shell in ["bash", "elvish", "fish", "powershell", "zsh"] {
        let mut cmd = run_isolated_git_sumi("");
        let output = cmd
            .arg("--generate-shell-completion")
            .arg(shell)
            .output()
            .unwrap();

        assert!(output.status.success());
        assert!(!output.stdout.is_empty());
    }
}
