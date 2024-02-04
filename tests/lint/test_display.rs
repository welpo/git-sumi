use super::contains;
use super::run_isolated_git_sumi;
use assert_cmd::Command;
use predicates::prelude::*;
use serde_json::Value;

const COMMIT_MESSAGE: &str = "🐛 fix(Scope)!: short description\n\nLonger body description\n\nBREAKING CHANGE: breaking description\nFooter1: value1\nFooter2: value2\n\n#123, ce6df36";

fn assert_table_output(cmd: &mut Command) {
    cmd.assert()
        .success()
        .stdout(contains("Gitmoji              │ 🐛"))
        .stdout(contains("Commit type          │ fix"))
        .stdout(contains("Scope                │ Scope"))
        .stdout(contains("Description          │ short description"))
        .stdout(contains("Body                 │ Longer body description"))
        .stdout(contains("Footers              │ BREAKING CHANGE:breaking description, Footer1:value1, Footer2:value2"))
        .stdout(contains("Is breaking          │ true"))
        .stdout(contains("Breaking description │ breaking description"))
        .stdout(contains("References           │ #123, ce6df36"))
        // No headers.
        .stdout(contains("Key").not())
        .stdout(contains("Value").not());
}

#[test]
fn success_display_format_cli() {
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("-dCG").arg("-f").arg("cli").arg(COMMIT_MESSAGE);
    assert_table_output(&mut cmd);
}

#[test]
fn success_display_format_cli_as_default() {
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("-dCG").arg(COMMIT_MESSAGE);
    assert_table_output(&mut cmd);
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
fn success_display_format_json() {
    let mut cmd = run_isolated_git_sumi("");

    let commit_message = "✨ feat(CrimeAndPunishment): Add Raskolnikov's internal monologue\n\nIntroduces the new chapter where Raskolnikov wrestles with his conscience.\n\nBREAKING CHANGE: New perspective on morality introduced.\nTranslator: Garnett\nPublisher: Penguin\n#1866";

    let output = cmd
        .arg("-dCGqf")
        .arg("json")
        .arg(commit_message)
        .unwrap()
        .stdout;

    let output_str = std::str::from_utf8(&output).unwrap();

    // Validate that the output is proper JSON.
    let parsed: Value = serde_json::from_str(output_str).expect("Output is not valid JSON");

    assert_eq!(parsed["gitmoji"], "✨");
    assert_eq!(parsed["commit_type"], "feat");
    assert_eq!(parsed["scope"], "CrimeAndPunishment");
    assert_eq!(
        parsed["description"],
        "Add Raskolnikov's internal monologue"
    );
    assert_eq!(
        parsed["body"],
        "Introduces the new chapter where Raskolnikov wrestles with his conscience."
    );
    let expected_footers: Value = serde_json::json!([
        "BREAKING CHANGE:New perspective on morality introduced.",
        "Translator:Garnett",
        "Publisher:Penguin\n#1866"
    ]);
    assert_eq!(parsed["footers"], expected_footers);
    assert_eq!(parsed["is_breaking"], true);
    assert_eq!(
        parsed["breaking_description"],
        "New perspective on morality introduced."
    );
    let expected_references: Value = serde_json::json!(["#1866"]);
    assert_eq!(parsed["references"], expected_references);
}

#[test]
fn success_contains_body() {
    let full_message = ("bought wards\n\nPlease commend.").to_string();
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("-df")
        .arg("toml")
        .arg(full_message)
        .assert()
        .success()
        .stdout(contains("body = \"Please commend.\""));
}

#[test]
fn success_does_not_contain_header() {
    let full_message = (" ✨feat: support privacy-respecting analytics (#193)").to_string();
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("-df")
        .arg("toml")
        .arg(full_message)
        .assert()
        .success()
        .stdout(contains("header = ").not());
}

#[test]
fn success_display_format_markdown() {
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("-dCG").arg("-f").arg("table").arg(COMMIT_MESSAGE);
    cmd.assert()
        .success()
        .stdout(contains("| Key"))
        .stdout(contains("| Value"))
        .stdout(contains("| Gitmoji              | 🐛"))
        .stdout(contains("|----------------------|----------------------------------------------------------------------|"))
        .stdout(contains("| Scope                | Scope"))
        .stdout(contains("| Description          | short description"))
        .stdout(contains("| Body                 | Longer body description "))
        .stdout(contains(
            "| Footers              | BREAKING CHANGE:breaking description, Footer1:value1, Footer2:value2 |",
        ))
        .stdout(contains("| Is breaking          | true "))
        .stdout(contains("| Breaking description | breaking description "))
        .stdout(contains("| References           | #123, ce6df36 "));
}
