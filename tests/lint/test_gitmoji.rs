use super::contains;
use super::run_isolated_git_sumi;
use super::{create_and_stage_file, git_command, setup_git_repo};
use git_sumi::lint::constants::gitmoji::{STRING_EMOJIS, UNICODE_EMOJIS};

#[test]
fn error_two_unicode_emojis_2() {
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("-G")
        .arg("✨💇 test: dance to the beat of absurdity")
        .assert()
        .failure()
        .stderr(contains("Header must contain exactly 1 emoji"));
}

#[test]
fn error_two_string_emojis() {
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("-G")
        .arg(":sparkles: feat: :sparkles: explain Inland Empire")
        .assert()
        .failure()
        .stderr(contains("Header must contain exactly 1 emoji"));
}

#[test]
fn error_two_unicode_emojis() {
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("-G")
        .arg("✨ feat: ✨ turn the Cappella Sistina ceiling into a 3D experience")
        .assert()
        .failure()
        .stderr(contains("Header must contain exactly 1 emoji"));
}

#[test]
fn success_joined_emoji() {
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("-G")
        .arg("🧑‍💻 decipher Rosetta Stone")
        .assert()
        .success()
        .stdout(contains("All 1 check passed"));
}

#[test]
fn error_no_emojis() {
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("--gitmoji")
        .arg("understand life backwards")
        .assert()
        .failure()
        .stderr(contains("Header must contain exactly 1 emoji"));
}

#[test]
fn success_no_emojis() {
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("-I").arg("add something cool").assert().success();
}

#[test]
fn success_one_unicode_emoji() {
    for &emoji in UNICODE_EMOJIS.iter() {
        let test_case = format!("{emoji} fix: mend Kintsugi on Ming vase");
        let mut cmd = run_isolated_git_sumi("");
        cmd.arg("--gitmoji").arg(&test_case).assert().success();
    }
}

#[test]
fn success_one_string_emoji() {
    for &emoji in STRING_EMOJIS.iter() {
        let test_case = format!("{emoji} fix: iron out Bladee's auto-tune");
        let mut cmd = run_isolated_git_sumi("");
        cmd.arg("-G").arg(&test_case).assert().success();
    }
}

#[test]
fn error_invalid_emoji_unicode() {
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("--gitmoji")
        .arg("🌟 style: drape UI in Sakamoto minimalism")
        .assert()
        .failure()
        .stderr(contains("Invalid emoji"));
}

#[test]
fn error_invalid_emoji_string() {
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("--gitmoji")
        .arg(":star: feat: awesome feature")
        .assert()
        .failure()
        .stderr(contains("Invalid emoji"));
}

#[test]
fn error_invalid_emoji_unicode_2() {
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("--gitmoji")
        .arg("🐵 feat: awesome feature")
        .assert()
        .failure()
        .stderr(contains("Invalid emoji"));
}

#[test]
fn error_invalid_emoji_string_2() {
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("--gitmoji")
        .arg(":monkey: feat: awesome feature")
        .assert()
        .failure()
        .stderr(contains("Invalid emoji"));
}

#[test]
fn success_mid_header_emoji_removal_keeps_words_separated() {
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("-G")
        .arg("-d")
        .arg("-f")
        .arg("json")
        .arg("fix 🐛 the bug")
        .assert()
        .success()
        .stdout(contains(r#""description": "fix the bug""#));
}

#[test]
fn error_non_imperative_verb_before_mid_header_emoji() {
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("-G")
        .arg("-I")
        .arg("fixed 🐛 the bug")
        .assert()
        .failure()
        .stderr(contains("non-imperative verb: 'fixed'"));
}

#[test]
fn success_trailing_header_emoji_keeps_body_separation() {
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("-G")
        .arg("-C")
        .arg("fix: squash the bug 🐛\n\nExplanatory body.")
        .assert()
        .success()
        .stdout(contains("All 2 checks passed"));
}

#[test]
fn error_gitmoji_only_header_with_body_stays_empty() {
    let repo = setup_git_repo();
    create_and_stage_file(repo.path(), "initial.txt", "initial");
    git_command()
        .args(["commit", "--message", "initial"])
        .current_dir(repo.path())
        .assert()
        .success();
    create_and_stage_file(repo.path(), "body.txt", "body");
    git_command()
        .args(["commit", "--message", "📝", "--message", "This is the body"])
        .current_dir(repo.path())
        .assert()
        .success();

    let mut cmd = run_isolated_git_sumi("");
    cmd.current_dir(repo.path())
        .args(["--gitmoji", "--from", "HEAD~1", "--to", "HEAD"])
        .assert()
        .failure()
        .stdout(contains("Header must not be empty"));
}

#[test]
fn success_emoji_without_gitmoji_whitespace() {
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("-dW")
        .arg("test: 🧪 add unit tests for timeline consistency")
        .assert()
        .success();
}
