extern crate tempfile;

use super::contains;
use super::run_isolated_git_sumi;

#[test]
fn error_conventional_commit_period() {
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("-PC")
        .arg("docs(Kokoro): Annotate Sensei's last letter.")
        .assert()
        .failure()
        .stderr(contains("Header must not end with a period"));
}

#[test]
fn success_breaking_change_with_gitmoji() {
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("-CGd")
        .arg("--format=toml")
        .arg("✨ feat(theme-switcher)!: respect theme_default when JS is enabled")
        .assert()
        .success()
        .stdout(contains("is_breaking = true"));
}

#[test]
fn success_with_capital_letter_conventional() {
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("-C")
        .arg("-E")
        .arg("upper")
        .arg("fix(A Confederacy of Dunces): Tweak Ignatius' valve settings")
        .assert()
        .success();
}

#[test]
fn error_without_capital_letter_conventional() {
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("-C")
        .arg("-E")
        .arg("upper")
        .arg("chore(SegensderErde): sow Hamsun's earth blessing")
        .assert()
        .failure()
        .stderr(contains(
            "Description must start with a capital letter. Try 'Sow Hamsun's earth blessing'",
        ));
}

#[test]
fn success_capital_letter_conventional_gitmoji() {
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("-C")
        .arg("-G")
        .arg("-E")
        .arg("upper")
        .arg("💄 style: Change something")
        .assert()
        .success()
        .stdout(contains("3 checks passed"));
}

#[test]
fn error_capital_letter_conventional_gitmoji() {
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("-CG")
        .arg("-E")
        .arg("upper")
        .arg("💄 style(language-switcher): update icon to be lighter (#173)")
        .assert()
        .failure()
        .stderr(contains(
            "Description must start with a capital letter. Try 'Update icon to be lighter (#173)'",
        ));
}

#[test]
fn success_conventional_whitespace_multiple_line_commit() {
    let mut cmd = run_isolated_git_sumi("");
    let commit_message = "test: add unit tests\n\nMore details.";

    cmd.arg("-CW")
        .arg(commit_message)
        .assert()
        .success()
        .stdout(contains("2 checks passed"));
}

#[test]
fn error_multiple_errors_in_commit() {
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("-PE")
        .arg("upper")
        .arg("chore: add gitignore.")
        .arg("--types-allowed")
        .arg("feat,fix")
        .assert()
        .failure()
        .stderr(contains("Found 3 linting errors"))
        .stderr(contains("Header must not end with a period"))
        .stderr(contains("Description must start with a capital letter"))
        .stderr(contains("Invalid commit type"));
}

#[test]
fn error_conventional_fails_parsing_proceeds() {
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("--gitmoji")
        .arg("--conventional")
        .arg("test(): add unit tests\nMore details.")
        .assert()
        .failure()
        .stderr(contains("Missing type in the commit summary"))
        .stderr(contains("Found 3 linting errors"))
        .stderr(contains("Separate header from body with a blank line"))
        .stderr(contains("Header must contain exactly 1 emoji, found 0"));
}

#[test]
fn success_valid_conventional_commit_with_gitmoji() {
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("-G")
        .arg("-C")
        .arg("--types-allowed")
        .arg("feat, fix, chore, docs, style, perf, style, refactor, test")
        .arg(":sparkles: feat(LostInTranslation): decihper Bill Murray's whisper")
        .assert()
        .success()
        .stdout(contains("3 checks passed"));
}

#[test]
fn error_invalid_conventional_commit_title_length() {
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("-C")
        .arg("--max-header-length")
        .arg("10")
        .arg("docs(Camus): Clarify The Stranger's ending")
        .assert()
        .failure()
        .stderr(contains("Line number 1 is too long (42 > 10)"));
}

#[test]
fn error_invalid_conventional_commit_body_length() {
    let mut cmd = run_isolated_git_sumi("");
    let full_message =
        "chore(HeroOfOurTime): Standardize Lermontov's poetic forms\n\nThis is an extremely long body description...";
    cmd.arg("-C")
        .arg("--max-body-length=10")
        .arg(full_message)
        .assert()
        .failure()
        .stderr(contains("Line number 3 is too long (45 > 10)"));
}

#[test]
fn error_invalid_conventional_commit_invalid_gitmoji() {
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("-GC")
        .arg(":invalidmoji: style(ConfederacyOfDunces): Refactor Ignatius' wardrobe")
        .assert()
        .failure()
        .stderr(contains("Invalid emoji"));
}

#[test]
fn success_quiet() {
    let mut cmd = run_isolated_git_sumi("");
    let assert = cmd
        .arg("--quiet")
        .arg("--conventional")
        .arg("chore(Sartre): Update dependency existentialism to v4.2")
        .assert()
        .success();

    let stdout_str = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    assert!(!stdout_str.contains("checks passed"));
}

#[test]
fn error_conventional_gitmoji_imperative() {
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("-GCI")
        .arg(":recycle: refactor: changes the code")
        .assert()
        .failure()
        .stderr(contains(
            "Description starts with a non-imperative verb: 'changes'",
        ));
}

#[test]
fn success_conventional_commits_and_emoji_formats() {
    const CONVENTIONAL_COMMITS_AND_EMOJI_FORMATS: [&str; 7] = [
        "{emoji} feat: awesome feature",
        "{emoji}fix: awesome feature",
        "{emoji} refactor:awesome feature",
        "{emoji}docs:awesome feature",
        "style{emoji}: awesome feature",
        "perf: awesome feature{emoji}",
        "test: {emoji}awesome feature",
    ];

    const EMOJIS: [&str; 10] = [
        "🚀",
        "✨",
        "🔧",
        ":memo:",
        ":lock:",
        ":lipstick:",
        "♻️",
        ":heavy_minus_sign:",
        "🔀",
        "👽️",
    ];

    for &emoji in &EMOJIS {
        for &format in &CONVENTIONAL_COMMITS_AND_EMOJI_FORMATS {
            let test_case = format.replace("{emoji}", emoji);
            let mut cmd = run_isolated_git_sumi("");
            cmd.arg("-GC").arg(&test_case).assert().success();
        }
    }
}

#[test]
fn error_imperative_and_gitmoji() {
    let mut cmd = run_isolated_git_sumi("");
    let commit_message = "✨ updates feature\n\nThis feature…";

    cmd.arg("--gitmoji")
        .arg("--imperative")
        .arg(commit_message)
        .assert()
        .failure()
        .stderr(contains("Description starts with a non-imperative verb"));
}

#[test]
fn error_custom_title_too_long_gitmoji() {
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("--max-header-length")
        .arg("1")
        .arg("-G")
        .arg("✨a")
        .assert()
        .failure()
        .stderr(contains("Line number 1 is too long (2 > 1)"));
}

#[test]
fn error_catch_multiple_commit_errors_with_split_lines() {
    let commit_message = "fix: Fix issue with login.\nfix: resolve database deadlock issue";

    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("--split-lines")
        .arg("--conventional")
        .arg("--description-case")
        .arg("upper")
        .arg("--no-period")
        .arg(commit_message)
        .assert()
        .failure()
        .stderr(contains("Description must start with a capital letter"))
        .stderr(contains("Header must not end with a period"))
        .stderr(contains("2 out of 2 lines failed linting"));
}

#[test]
fn success_empty_line_skipped_with_split_lines() {
    let commit_message = "🔨 chore: add pull request template\n\n📝 chore: update contact e-mail";

    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("--split-lines")
        .arg("--conventional")
        .arg("--description-case")
        .arg("lower")
        .arg("--gitmoji")
        .arg(commit_message)
        .assert()
        .success();
}

#[test]
fn error_whitespace_gitmoji() {
    let commit_message = " 🔨 add pull request template";

    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("--gitmoji")
        .arg("--whitespace")
        .arg(commit_message)
        .assert()
        .failure()
        .stderr(contains("Leading space"));
}

#[test]
fn success_with_lowercase_description_and_pattern() {
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("--description-case")
        .arg("lower")
        .arg("--header-pattern")
        .arg("^[a-z\\s]+$") // Backslashes must be escaped.
        .arg("the air is hangs in the air")
        .assert()
        .success()
        .stdout(contains("✅ All 2 checks passed"));
}

#[test]
fn success_gitmoji_with_pattern() {
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("--gitmoji")
        .arg("--header-pattern")
        .arg("^(?:[\\p{Emoji_Presentation}\\p{Extended_Pictographic}\\u{200D}])")
        .arg("⬆️ chore(deps): update Rust crate foo to v4.2")
        .assert()
        .success()
        .stdout(contains("✅ All 2 checks passed"));
}
