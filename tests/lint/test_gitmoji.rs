use super::contains;
use super::run_isolated_git_sumi;
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
        let test_case = format!("{} fix: mend Kintsugi on Ming vase", emoji);
        let mut cmd = run_isolated_git_sumi("");
        cmd.arg("--gitmoji").arg(&test_case).assert().success();
    }
}

#[test]
fn success_one_string_emoji() {
    for &emoji in STRING_EMOJIS.iter() {
        let test_case = format!("{} fix: iron out Bladee's auto-tune", emoji);
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
fn success_emoji_without_gitmoji_whitespace() {
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("-dW")
        .arg("test: 🧪 add unit tests for timeline consistency")
        .assert()
        .success();
}
