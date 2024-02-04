extern crate tempfile;

use super::contains;
use super::run_isolated_git_sumi;

#[test]
fn success_with_empty_types_and_scopes() {
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("-C")
        .arg("--types-allowed")
        .arg("")
        .arg("--scopes-allowed")
        .arg("")
        .arg("feat(SegensDerErde): Add Hamsun's agrarian cycles")
        .assert()
        .success();
}

#[test]
fn success_breaking_change() {
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("-Cd")
        .arg("--format=toml")
        .arg("feat(theme-switcher)!: respect `theme_default` when JS is enabled")
        .assert()
        .success()
        .stdout(contains("is_breaking = true"));
}

#[test]
fn success_display_format_toml() {
    let mut cmd = run_isolated_git_sumi("");

    let commit_message = "📝 docs(cleanliness): Update mop instructions\n\nMop in circular motions for best results.\n\nBREAKING CHANGE: No more mopping in squares!\nMopBrand: WhirlMop\nSurface: Tile\nRelated: #987, d9e8f1b";

    cmd.arg("-CdG")
        .arg("--format")
        .arg("toml")
        .arg(commit_message)
        .assert()
        .success()
        .stdout(contains("gitmoji = \"📝\""))
        .stdout(contains("commit_type = \"docs\""))
        .stdout(contains("scope = \"cleanliness\""))
        .stdout(contains("description = \"Update mop instructions\""))
        .stdout(contains("body = \"Mop in circular motions for best results.\""))
        .stdout(contains("footers = [\"BREAKING CHANGE:No more mopping in squares!\", \"MopBrand:WhirlMop\", \"Surface:Tile\", \"Related:#987, d9e8f1b\"]"))
        .stdout(contains("is_breaking = true"))
        .stdout(contains("breaking_description = \"No more mopping in squares!\""))
        .stdout(contains("references = [\"#987\", \"d9e8f1b\"]"));
}

#[test]
fn success_with_user_specified_type() {
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("--conventional")
        .arg("--types-allowed")
        .arg("feat,custom")
        .arg("custom(YellowArrow): fine-tune the train's timing")
        .assert()
        .success()
        .stdout(contains("All 2 checks passed"));
}

#[test]
fn error_unallowed_type() {
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("-C")
        .arg("--types-allowed")
        .arg("feat,custom")
        .arg("penguin(PingvinZabludilsya): resolve penguin's directional issues")
        .assert()
        .failure()
        .stderr(contains("Invalid commit type"));
}

#[test]
fn success_allowed_scope() {
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("-C")
        .arg("--scopes-allowed")
        .arg("niebla")
        .arg("feat(niebla): Add Unamuno's existential layer")
        .assert()
        .success();
}

#[test]
fn error_unallowed_scope() {
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("-C")
        .arg("--scopes-allowed")
        .arg("users")
        .arg("feat(admin): add new admin")
        .assert()
        .failure()
        .stderr(contains("Invalid commit scope"));
}

#[test]
fn success_valid_conventional_commit() {
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("-C")
        .arg("docs(BladeRunner): annotate Deckard's ambiguity (#2049)")
        .assert()
        .success();
}

#[test]
fn error_conventional_commit_empty_scope() {
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("-C")
        .arg("bugfix(): fix some bug")
        .assert()
        .failure()
        .stderr(contains(
            "Missing type in the commit summary, expected `type: description`",
        ));
}

#[test]
fn success_conventional_commit() {
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("--conventional")
        .arg("style(NoLongerHuman): refine Dazai's self-reflection")
        .assert()
        .success();
}
