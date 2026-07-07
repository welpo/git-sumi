//! Tests for git comment handling: comment line stripping, custom
//! `core.commentChar`, and the verbose-commit scissors line.

extern crate assert_cmd;
extern crate tempfile;

use super::contains;
use super::run_isolated_git_sumi;
use super::{create_and_stage_file, setup_git_repo};
use assert_cmd::Command;
use std::path::Path;
use tempfile::tempdir;

fn set_commentchar(repo_dir: &Path, commentchar: &str) {
    Command::new("git")
        .args(["config", "core.commentChar", commentchar])
        .current_dir(repo_dir)
        .assert()
        .success();
}

#[test]
fn success_ignore_comments() {
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("-W")
        // If the comment weren't ignored, git-sumi would complain about the adjacent spaces in the second line.
        .arg("feat: adds feature\n\n#       modified:   src/lib.rs")
        .assert()
        .success();
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

/// Git only treats lines starting with the comment character at column 0
/// as comments; indented lines are kept in the commit message and must be linted.
#[test]
fn error_indented_comment_line_is_linted() {
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("-W")
        .arg("feat: add feature\n\n  # indented, so  git keeps this line")
        .assert()
        .failure()
        .stderr(contains("Whitespace issue"));
}

/// Comment lines starting with a custom `core.commentChar` must be ignored.
#[test]
fn success_custom_commentchar_comments_are_ignored() {
    let tmp_dir = setup_git_repo();
    set_commentchar(tmp_dir.path(), ";");

    let mut cmd = run_isolated_git_sumi("");
    cmd.current_dir(tmp_dir.path())
        .arg("-W")
        // If the comment weren't ignored, the adjacent spaces would fail linting.
        .arg("feat: add feature\n\n;       modified:   src/lib.rs")
        .assert()
        .success();
}

/// With a custom `core.commentChar`, '#' lines are regular content and must be linted.
#[test]
fn error_hash_line_is_content_with_custom_commentchar() {
    let tmp_dir = setup_git_repo();
    set_commentchar(tmp_dir.path(), ";");

    let mut cmd = run_isolated_git_sumi("");
    cmd.current_dir(tmp_dir.path())
        .arg("-W")
        .arg("feat: add feature\n\n# this  hash line  is content")
        .assert()
        .failure()
        .stderr(contains("Whitespace issue"));
}

#[test]
fn test_verbose_git_commit() {
    let tmp_dir = setup_git_repo();
    let repo_dir = tmp_dir.path();

    create_and_stage_file(repo_dir, "new_file.txt", "New content");

    let msg = r#"feat: Add new feature
# Please enter the commit message for your changes. Lines starting
# with '#' will be ignored, and an empty message aborts the commit.
#
# On branch verbose
# Changes to be committed:
#	new file:   new_file.txt
#
# Changes not staged for commit:
#	modified:   src/lib.rs
#
# ------------------------ >8 ------------------------
# Do not modify or remove the line above.
# Everything below it will be ignored.
diff --git a/new_file.txt b/new_file.txt
new file mode 100644
index 0000000..a11f211
--- /dev/null
+++ b/new_file.txt
@@ -0,0 +1 @@
+New content
    "#;

    let mut cmd = run_isolated_git_sumi("");
    cmd.current_dir(repo_dir)
        .arg("-C")
        .arg("--commit")
        .arg(msg)
        .assert()
        .success();
}

#[test]
fn test_verbose_git_commit_custom_commentchar() {
    let tmp_dir = setup_git_repo();
    let repo_dir = tmp_dir.path();

    set_commentchar(repo_dir, ";");
    create_and_stage_file(repo_dir, "new_file.txt", "New content");

    // The text after the scissor line has no blank line separator, which would
    // trigger "Separate header from body" if the scissor line weren't recognised.
    let msg = "feat: Add new feature\n\
; ------------------------ >8 ------------------------\n\
this line has no blank separator from the header";

    let mut cmd = run_isolated_git_sumi("");
    cmd.current_dir(repo_dir)
        .arg("-C")
        .arg("--whitespace")
        .arg("--commit")
        .arg(msg)
        .assert()
        .success();
}

#[test]
fn test_verbose_git_commit_diff_does_not_trigger_line_length() {
    let tmp_dir = setup_git_repo();
    let repo_dir = tmp_dir.path();

    create_and_stage_file(repo_dir, "new_file.txt", "New content");

    // The diff contains a line longer than 50 chars which would fail --header-length 50 if it weren't stripped.
    let long_line = "a".repeat(200);
    let msg = format!(
        "feat: Add feature\n\
# ------------------------ >8 ------------------------\n\
+{}",
        long_line
    );

    let mut cmd = run_isolated_git_sumi("");
    cmd.current_dir(repo_dir)
        .arg("-C")
        .arg("--max-header-length")
        .arg("50")
        .arg("--commit")
        .arg(&msg)
        .assert()
        .success();
}
