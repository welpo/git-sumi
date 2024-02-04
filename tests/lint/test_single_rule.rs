extern crate tempfile;

use super::contains;
use super::run_isolated_git_sumi;

#[test]
fn success_with_capital_letter_non_conventional() {
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("-E")
        .arg("upper")
        .arg("Replant metaphors in Umbral's garden")
        .assert()
        .success();
}

#[test]
fn error_capital_letter_non_conventional() {
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("-E")
        .arg("upper")
        .arg("add gitignore")
        .assert()
        .failure()
        .stderr(contains(
            "Description must start with a capital letter. Try 'Add gitignore'",
        ));
}

#[test]
fn error_invalid_description_case() {
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("-E")
        .arg("title") // "title" is not a valid option
        .arg("add gitignore")
        .assert()
        .failure()
        .stderr(contains("Unknown case"));
}

#[test]
fn error_bad_output_format() {
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("-f")
        .arg("refactor(TheYellowArrow): Implement one-way journey")
        .assert()
        .failure()
        .stderr(contains("Unknown format"));
}

#[test]
fn success_lowercase_description() {
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("-E")
        .arg("lower")
        .arg("add unit tests for timeline consistency")
        .assert()
        .success();
}

#[test]
fn error_catch_conventional_body_errror() {
    let mut cmd = run_isolated_git_sumi("");
    let commit_message = "feat: adds feature\nThis feature…";

    cmd.arg("-C")
        .arg(commit_message)
        .assert()
        .failure()
        .stderr(contains("Separate header from body with a blank line"));
}

#[test]
fn success_valid_whitespace() {
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("-WC")
        .arg("test: add unit tests")
        .assert()
        .success();
}

#[test]
fn success_valid_whitespace_gitmoji() {
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("-GCW")
        .arg("✅ test: add unit tests")
        .assert()
        .success();
}

#[test]
fn error_invalid_whitespace() {
    let test_cases = [
        " problem leading space",
        "problem trailing space ",
        "problem  double space",
    ];

    for &test_case in test_cases.iter() {
        let mut cmd = run_isolated_git_sumi("");
        cmd.arg("--whitespace")
            .arg(test_case)
            .assert()
            .failure()
            .stderr(contains("Whitespace issue"));
    }
}

#[test]
fn success_non_letter_cant_be_capitalised() {
    let test_cases = [" space", "✅ emoji", "1 number"];
    for &test_case in test_cases.iter() {
        let mut cmd = run_isolated_git_sumi("");
        cmd.arg("-E").arg("upper").arg(test_case).assert().success();
    }
}

#[test]
fn error_body_whitespace_issues() {
    let test_cases = [
        (
            "Dark Souls:  Blighttown\n\nBeware of toxic",
            "Dark Souls:🟥️🟥️Blighttown",
        ),
        (
            "Elden Ring: Gatefront Ruins \n\nNew vistas await",
            "Elden Ring: Gatefront Ruins🟥️",
        ),
        (
            "Dark Souls: Anor Londo  \n\nHome of the gods",
            "Dark Souls: Anor Londo🟥️🟥️",
        ),
        (
            "Elden Ring: The Tree of Life\n\n Uncover  its secrets",
            "🟥️Uncover🟥️🟥️its secrets",
        ),
    ];
    for &(test_case, expected_issues) in &test_cases {
        let mut cmd = run_isolated_git_sumi("");
        cmd.arg("-W")
            .arg(test_case)
            .assert()
            .failure()
            .stderr(contains("Whitespace issue"))
            .stderr(contains(expected_issues));
    }
}

#[test]
fn error_whitespace_issue_pluralization() {
    let test_cases = [
        (
            "Put  Dwight's stuff in vending machine",
            "Whitespace issue detected:",
        ),
        (
            "Michael Scott: Life lessons \n\nI declare bankruptcy!",
            "Whitespace issue detected:",
        ),
        ("Put the  stapler in jello ", "Whitespace issues detected:"),
    ];

    for &(test_case, expected_issues) in &test_cases {
        let mut cmd = run_isolated_git_sumi("");
        cmd.arg("-W")
            .arg(test_case)
            .assert()
            .failure()
            .stderr(contains(expected_issues));
    }
}

#[test]
fn success_whitespace_single_line_commit() {
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("-W").arg("test: add unit tests").assert().success();
}

#[test]
fn error_not_lowercase_description() {
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("--description-case")
        .arg("lower")
        .arg("-C")
        .arg("fix: Correct minor typos in code")
        .assert()
        .failure()
        .stderr(contains(
            "Description must start in lowercase. Try 'correct minor typos in code'",
        ));
}

#[test]
fn error_not_using_imperative() {
    let test_cases = vec![
        "Adds a feature",
        "fixes a bug",
        "Implements a cool thing",
        "Updated the README",
        "Refactored the utils class",
        "Deleting unused files",
        "Merging branch 'main'",
        "Optimizing image size",
        "Reverting the previous commit",
        "created a new component",
        "Documented the main function",
        "Deprecating the old API",
        "Translated comments to English",
        "Linted the source code",
        "Tested the new feature",
        "Validating the input",
        "Released version 1.0",
        "Introduced breaking changes",
        "Synchronized the database",
        "Deploying to production",
        "Initializing the variables",
        "Automated the tests",
        "Improving performance",
        "Deleted unused files",
        "Optimised image size",
        "Reverted the previous commit",
        "Deprecated the old API",
        "Translating comments to English",
        "Translated the strings to Spanish",
        "Styled the header",
        "Validated the input",
        "Introducing breaking changes",
        "Built the project",
        "Deployed to production",
        "Initialized the variables",
        "Improved performance",
    ];

    for commit_message in test_cases {
        let mut cmd = run_isolated_git_sumi("");
        cmd.arg("-I")
            .arg(commit_message)
            .assert()
            .failure()
            .stderr(contains("Description starts with a non-imperative verb"));
    }
}

#[test]
fn success_imperative_verbs() {
    let test_cases = vec![
        "Add a new feature",
        "Fix bug",
        "Implement a cool thing",
        "Update the README",
        "Refactor the utils class",
        "Delete unused files",
        "Merge branch 'main'",
        "Revert the previous commit",
        "Create a new component",
        "Document the main function",
        "Deprecate the old API",
        "Translate comments to English",
        "Lint the source code",
        "Test the new feature",
        "Style the header",
        "Validate the input",
        "Release version 1.0",
        "Introduce breaking changes",
        "Synchronize the database",
        "Build the project",
        "Deploy to production",
        "Initialise the variables",
        "Automate the tests",
        "Improve performance",
        "Clarify the documentation",
        "Secure the application",
        "Redefine the constants",
        "Finalize the implementation",
        "Format the code",
    ];

    for commit_message in test_cases {
        let mut cmd = run_isolated_git_sumi("");
        cmd.arg("--imperative")
            .arg(commit_message)
            .assert()
            .success();
    }
}

#[test]
fn error_body_too_long_custom() {
    let mut cmd = run_isolated_git_sumi("");
    let long_body = "a".repeat(51);
    let full_message = format!("stare into the void\n\n{}", long_body);

    cmd.arg("--max-body-length")
        .arg("50")
        .arg(full_message)
        .assert()
        .failure()
        .stderr(contains("Line number 3 is too long (51 > 50)"));
}

#[test]
fn success_body_long_custom() {
    let mut cmd = run_isolated_git_sumi("");
    let long_body = "a".repeat(50);
    let full_message = format!("style: nuance Yeat's vocal fry\n\n{}", long_body);

    cmd.arg("--max-body-length")
        .arg("50")
        .arg(full_message)
        .assert()
        .success();
}

#[test]
fn success_body_long_custom_title_long_custom() {
    let mut cmd = run_isolated_git_sumi("");
    let long_body = "a".repeat(50);
    let full_message = format!("rehearse Kamasi Washington's scales\n\n{}", long_body);

    cmd.arg("--max-body-length")
        .arg("50")
        .arg("-H")
        .arg("50")
        .arg(full_message)
        .assert()
        .success();
}

#[test]
fn error_ends_with_period() {
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("--no-period")
        .arg("hack the mainframe.")
        .assert()
        .failure()
        .stderr(contains("Header must not end with a period"));
}

#[test]
fn success_ends_with_period() {
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("--no-period")
        .arg("hack the mainframe")
        .assert()
        .success();
}

#[test]
fn error_whitespace_body_missing_newline() {
    let mut cmd = run_isolated_git_sumi("");
    let commit_message =
        "unleash robot unicorns\nThey're now galloping in the meadows of our code.";

    cmd.arg("-W")
        .arg(commit_message)
        .assert()
        .failure()
        .stderr(contains("Separate header from body with a blank line"));
}

#[test]
fn error_whitespace_body_missing_newline_conventional() {
    let mut cmd = run_isolated_git_sumi("");
    let full_message = "feat: awesome feature\nThis feature…";

    cmd.arg("-C")
        .arg(full_message)
        .assert()
        .failure()
        .stderr(contains("Separate header from body with a blank line"));
}

#[test]
fn success_body_has_newline() {
    let mut cmd = run_isolated_git_sumi("");
    let full_message = "feat: awesome feature\n\nThe body";

    cmd.arg("-W").arg(full_message).assert().success();
}

#[test]
fn error_empty_header() {
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("-G")
        .arg("")
        .assert()
        .failure()
        .stderr(contains("Header must not be empty"));
}

#[test]
fn success_stdin_conventional() {
    let input = "refactor: relocate Starry Night to Impressionist wing";
    let mut cmd = run_isolated_git_sumi("");
    cmd.write_stdin(input)
        .arg("-C")
        .assert()
        .success()
        .stdout(contains("All 1 check passed"));
}

#[test]
fn error_custom_title_too_long() {
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("--max-header-length")
        .arg("50")
        .arg("a".repeat(51))
        .assert()
        .failure()
        .stderr(contains("Line number 1 is too long (51 > 50)"));
}

#[test]
fn error_when_header_does_not_match_jira_pattern() {
    let test_cases = ["Fix issue with user login", "JIRA1234: Missing hyphen"];

    for &test_case in test_cases.iter() {
        let mut cmd = run_isolated_git_sumi("");
        cmd.arg("--header-pattern")
            .arg("^JIRA-\\d+:")
            .arg(test_case)
            .assert()
            .failure()
            .stderr(contains("Header does not match the required pattern"));
    }
}

#[test]
fn success_with_valid_jira_ticket_in_header() {
    let test_case = "JIRA-1234: Fix issue with user login";
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("--header-pattern")
        .arg("^JIRA-\\d+:")
        .arg(test_case)
        .assert()
        .success();
}

#[test]
fn error_when_using_unsupported_regex_feature() {
    let mut cmd = run_isolated_git_sumi("");
    cmd.arg("--header-pattern")
        .arg("(?!.*WIP)")
        .arg("WIP: Start project documentation")
        .assert()
        .failure()
        .stderr(contains("Header regex pattern '(?!.*WIP)' is invalid"));
}
