use assert_cmd::Command;
use lazy_static::lazy_static;
use predicates::str::contains;
use tempfile::tempdir;

#[cfg(test)]
mod lint;

lazy_static! {
    static ref TEST_DIR: tempfile::TempDir = tempdir().unwrap();
}

fn run_isolated_git_sumi(subcommand: &str) -> Command {
    let mut cmd = Command::cargo_bin("git-sumi").unwrap();
    if !subcommand.is_empty() {
        cmd.arg(subcommand);
    }
    cmd.current_dir(TEST_DIR.path()); // Change dir to avoid loading the project's config.
    cmd
}

#[test]
fn success_bash_completion() {
    let mut cmd = run_isolated_git_sumi("");
    let output = cmd
        .arg("--generate-shell-completion")
        .arg("bash")
        .output()
        .unwrap();

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(stdout.contains("_git-sumi() {"));
    assert!(stdout.contains("COMPREPLY=()"));
    assert!(stdout.contains("complete -F _git-sumi -o nosort -o bashdefault -o default git-sumi"));
}
