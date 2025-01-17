use crate::run_isolated_git_sumi;
use predicates::str::contains;
use tempfile::tempdir;

#[test]
fn success_read_from_file() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("commit-msg.txt");
    std::fs::write(&file_path, "feat: add new feature").unwrap();
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("--file")
        .arg(file_path)
        .arg("-C")
        .assert()
        .success();
}

#[test]
fn error_nonexistent_file() {
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("--file")
        .arg("nonexistent-file.txt")
        .arg("-C")
        .assert()
        .failure()
        .stderr(contains(
            "Could not read commit message from 'nonexistent-file.txt'",
        ));
}

#[test]
fn error_empty_file() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("empty.txt");
    std::fs::write(&file_path, "").unwrap();
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("--file")
        .arg(file_path)
        .arg("-C")
        .assert()
        .failure()
        .stderr(contains("Header must not be empty"));
}

#[test]
fn error_conflict_file_and_message() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("commit.txt");
    std::fs::write(&file_path, "feat: test").unwrap();

    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("--file")
        .arg(file_path)
        .arg("direct message")
        .assert()
        .failure()
        .stderr(contains("cannot be used with"));
}

#[test]
fn success_file_with_comments() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("commit-with-comments.txt");
    std::fs::write(&file_path, "feat: add feature\n# This is a comment\n").unwrap();
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("--file")
        .arg(file_path)
        .arg("-C")
        .assert()
        .success();
}

#[test]
fn success_file_with_multiline() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("multiline.txt");
    std::fs::write(&file_path, "feat: add feature\n\nDetailed description").unwrap();
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("--file")
        .arg(file_path)
        .arg("-C")
        .assert()
        .success();
}
