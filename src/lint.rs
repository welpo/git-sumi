pub mod constants;
mod display;

use crate::config::{self, count_active_rules, DescriptionCase};
use crate::errors;
use crate::errors::{pluralize, SumiError};
use crate::parser::{handle_parsing, ParsedCommit};

use config::Config;
use constants::gitmoji::{STRING_EMOJIS, UNICODE_EMOJIS};
use constants::non_imperative_verbs::NON_IMPERATIVE_VERBS;
use display::display_parsed_commit;
use lazy_static::lazy_static;
use log::{error, info};
use regex::Regex;

pub fn run_lint_on_each_line(
    commit_message: &str,
    config: &Config,
) -> Result<Vec<ParsedCommit>, SumiError> {
    let non_empty_lines = commit_message.lines().filter(|line| !line.is_empty());
    let mut parsed_commits = Vec::new();
    let mut errors = Vec::new();

    for line in non_empty_lines.clone() {
        match run_lint(line, config) {
            Ok(parsed_commit) => parsed_commits.push(parsed_commit),
            Err(error) => {
                error!("{}", error);
                errors.push(error);
            }
        }
    }

    if errors.is_empty() {
        Ok(parsed_commits)
    } else {
        let lines_with_errors = errors.len();
        let line_plural_suffix = pluralize(lines_with_errors, "line", "lines");
        Err(SumiError::SplitLinesErrors {
            lines_with_errors,
            total_lines: non_empty_lines.count(),
            line_or_lines: line_plural_suffix.to_string(),
        })
    }
}

/// Lints and parses the given commit message.
/// Returns a `ParsedCommit` struct if the commit is valid, or an error message if it is not.
pub fn run_lint(raw_commit: &str, config: &Config) -> Result<ParsedCommit, SumiError> {
    let commit = preprocess_commit_message(raw_commit);
    info!("💬 Input: \"{}\"", commit);
    let mut non_fatal_errors: Vec<SumiError> = Vec::new();
    let parsed_commit = handle_parsing(&commit, config, &mut non_fatal_errors)?;
    let errors = validate_commit(&commit, &parsed_commit, config);
    non_fatal_errors.extend(errors);
    if non_fatal_errors.is_empty() {
        handle_success(&parsed_commit, config)?;
        return Ok(parsed_commit);
    }
    handle_failure(&non_fatal_errors)
}

fn preprocess_commit_message(commit: &str) -> String {
    // Remove comments.
    commit
        .lines()
        .filter(|line| !line.trim_start().starts_with('#'))
        .collect::<Vec<&str>>()
        .join("\n")
}

fn validate_commit(
    commit: &String,
    parsed_commit: &ParsedCommit,
    config: &Config,
) -> Vec<SumiError> {
    let mut errors = validate_whitespace_and_length(commit.to_string(), config);
    if let Some(validation_errors) = validate_parsed_commit(parsed_commit, config) {
        errors.extend(validation_errors);
    }
    errors
}

fn validate_whitespace_and_length(commit: String, config: &Config) -> Vec<SumiError> {
    let mut errors = Vec::new();
    let mut lines = commit.lines();
    let header_line = lines.next().unwrap_or("");
    errors.extend(validate_header_line(header_line, config));
    errors.extend(validate_body_lines(lines, config));
    errors
}

fn validate_header_line(header_line: &str, config: &Config) -> Vec<SumiError> {
    let mut errors = Vec::new();
    if let Err(err) = validate_whitespace(header_line, config) {
        errors.push(err);
    }
    if let Err(actual_length) = validate_line_length(header_line, config.max_header_length) {
        errors.push(SumiError::LineTooLong {
            line_number: 1,
            line_length: actual_length,
            max_length: config.max_header_length,
        });
    }
    errors
}

fn validate_line_length(line: &str, max_length: usize) -> Result<(), usize> {
    if max_length == 0 {
        return Ok(());
    }
    let actual_length = line.chars().count();
    if actual_length > max_length {
        return Err(actual_length);
    }
    Ok(())
}

fn validate_body_lines(lines: std::str::Lines, config: &Config) -> Vec<SumiError> {
    let mut errors = Vec::new();
    for (line_number, line) in lines.enumerate() {
        if line_number == 0 && !line.is_empty() {
            errors.push(SumiError::SeparateHeaderFromBody);
            continue;
        }
        if let Err(err) = validate_whitespace(line, config) {
            errors.push(err);
        }
        if let Err(actual_length) = validate_line_length(line, config.max_body_length) {
            errors.push(SumiError::LineTooLong {
                line_number: line_number + 2,
                line_length: actual_length,
                max_length: config.max_body_length,
            });
        }
    }
    errors
}

fn validate_whitespace(line: &str, config: &Config) -> Result<(), SumiError> {
    if !config.whitespace {
        return Ok(());
    }

    let mut issues = Vec::new();
    let highlighted_line = WHITESPACE_REGEX.replace_all(line, |caps: &regex::Captures| {
        let len = caps[0].len();
        let start = caps.get(0).unwrap().start();
        let end = caps.get(0).unwrap().end();

        if start == 0 {
            issues.push("Leading space".to_owned());
        } else if end == line.len() {
            issues.push("Trailing space".to_owned());
        } else {
            issues.push(format!("{} adjacent spaces", len));
        }

        "🟥️".repeat(len)
    });

    if !issues.is_empty() {
        let issue_count = issues.len();
        let issues_list = issues
            .iter()
            .map(|issue| format!("  - {}: \"{}\"", issue, highlighted_line))
            .collect::<Vec<String>>()
            .join("\n");

        return Err(SumiError::GeneralError {
            details: format!(
                "Whitespace {} detected:\n{}",
                pluralize(issue_count, "issue", "issues"),
                issues_list
            ),
        });
    }

    Ok(())
}

lazy_static! {
    // This regex has three capturing groups:
    // - ^\s+ captures leading spaces.
    // - \s+$ captures trailing spaces.
    // - \s{2,} captures adjacent spaces.
    static ref WHITESPACE_REGEX: Regex = Regex::new(r"(^\s+|\s+$|\s{2,})").unwrap();
}

fn validate_parsed_commit(parsed_commit: &ParsedCommit, config: &Config) -> Option<Vec<SumiError>> {
    let mut errors: Vec<SumiError> = Vec::new();

    if config.gitmoji {
        if let Err(err) = validate_gitmoji(&parsed_commit.gitmoji) {
            errors.push(err);
        }
    }

    if let Some(err) = validate_description_case(parsed_commit, config) {
        errors.push(err);
    }

    if config.imperative {
        if let Err(err) = is_imperative(&parsed_commit.description) {
            errors.push(err);
        }
    }

    if config.no_period {
        if let Err(err) = validate_no_period(&parsed_commit.header) {
            errors.push(err);
        }
    }

    if config.conventional {
        if let Err(err) = validate_commit_type_and_scope(parsed_commit, config) {
            errors.push(err);
        }
    }

    if !config.header_pattern.is_empty() {
        if let Err(err) = validate_header_pattern(&parsed_commit.header, &config.header_pattern) {
            errors.push(err);
        }
    }

    Some(errors)
}

/// Validates that the commit title contains exactly one gitmoji.
/// Returns the gitmoji if it is valid, or an error message if it is not.
/// Validates that the commit title contains exactly one gitmoji.
/// Returns the normalised gitmoji if it is valid, or an error message if it is not.
fn validate_gitmoji(emojis: &Option<Vec<String>>) -> Result<(), SumiError> {
    match emojis {
        Some(gitmojis) if gitmojis.len() != 1 => Err(SumiError::IncorrectEmojiCount {
            found: gitmojis.len(),
        }),
        Some(gitmojis) => {
            let gitmoji = &gitmojis[0];
            let normalised_gitmoji = normalise_emoji(gitmoji);
            if !UNICODE_EMOJIS.contains(normalised_gitmoji.as_str())
                && !STRING_EMOJIS.contains(&gitmoji.as_str())
            {
                Err(SumiError::InvalidEmoji {
                    emoji: gitmoji.clone(),
                })
            } else {
                Ok(())
            }
        }
        None => Err(SumiError::IncorrectEmojiCount { found: 0 }),
    }
}

// Function to normalise emojis, removing variation selectors.
fn normalise_emoji(emoji: &str) -> String {
    emoji.replace('\u{fe0f}', "")
}

fn validate_description_case(parsed_commit: &ParsedCommit, config: &Config) -> Option<SumiError> {
    match config.description_case {
        DescriptionCase::Lower => validate_lowercase(&parsed_commit.description).err(),
        DescriptionCase::Upper => validate_upper_case(&parsed_commit.description).err(),
        DescriptionCase::Any => None,
    }
}

fn validate_lowercase(description: &str) -> Result<(), SumiError> {
    let first_char = description.chars().next();
    match first_char {
        Some(c) if c.is_uppercase() => {
            let corrected_description = c.to_lowercase().to_string() + &description[1..];
            Err(SumiError::DescriptionNotLowercase {
                lowercase_header: corrected_description,
            })
        }
        Some(_) => Ok(()),
        None => Err(SumiError::EmptyCommitHeader),
    }
}

fn validate_upper_case(description: &str) -> Result<(), SumiError> {
    let first_char = description.chars().next().unwrap();
    if is_lowercase_letter(first_char) {
        let capitalized_title = capitalize_title(first_char, &description[1..]);
        return Err(SumiError::DescriptionNotTitleCase {
            capitalized_description: capitalized_title,
        });
    }
    Ok(())
}

// This is a best-effort heuristic, and will not catch all non-imperative messages.
fn is_imperative(description: &str) -> Result<(), SumiError> {
    let first_word = description
        .split_whitespace()
        .next()
        .unwrap_or("")
        .to_string();
    let first_word_lower = first_word.to_lowercase();
    if NON_IMPERATIVE_VERBS.contains(first_word_lower.as_str()) {
        return Err(SumiError::NonImperativeVerb { verb: first_word });
    }
    Ok(())
}

fn is_lowercase_letter(character: char) -> bool {
    character.is_alphabetic() && !character.is_uppercase()
}

fn capitalize_title(first_char: char, rest: &str) -> String {
    let capitalized_first_char = first_char.to_uppercase().collect::<String>();
    format!("{}{}", capitalized_first_char, rest)
}

fn validate_no_period(header: &str) -> Result<(), SumiError> {
    if header.ends_with('.') {
        return Err(SumiError::HeaderEndsWithPeriod);
    }
    Ok(())
}

fn validate_commit_type_and_scope(
    parsed_commit: &ParsedCommit,
    config: &Config,
) -> Result<(), SumiError> {
    let types_allowed = split_and_trim_list(&config.types_allowed);
    let scopes_allowed = split_and_trim_list(&config.scopes_allowed);

    // Empty lists mean all types/scopes are allowed.
    if types_allowed.is_empty() && scopes_allowed.is_empty() {
        return Ok(());
    }

    if let Some(commit_type) = &parsed_commit.commit_type {
        if !types_allowed.is_empty() && !types_allowed.contains(commit_type) {
            return Err(SumiError::InvalidCommitType {
                type_found: commit_type.clone(),
                allowed_types: types_allowed.join(", "),
            });
        }
    }

    if let Some(scope) = &parsed_commit.scope {
        if !scopes_allowed.is_empty() && !scopes_allowed.contains(scope) {
            return Err(SumiError::InvalidCommitScope {
                scope_found: scope.clone(),
                allowed_scopes: scopes_allowed.join(", "),
            });
        }
    }

    Ok(())
}

fn validate_header_pattern(header: &str, pattern: &str) -> Result<(), SumiError> {
    let re = Regex::new(pattern).map_err(|_| SumiError::InvalidRegexPattern {
        pattern: pattern.to_string(),
    })?;
    if !re.is_match(header) {
        return Err(SumiError::HeaderPatternMismatch {
            pattern: pattern.to_string(),
        });
    }
    Ok(())
}

// Helper function to process allowed types and scopes.
fn split_and_trim_list(list: &[String]) -> Vec<String> {
    list.iter()
        .flat_map(|s| s.split(',').map(|item| item.trim().to_string()))
        .filter(|x| !x.is_empty())
        .collect()
}

fn handle_success(parsed_commit: &ParsedCommit, config: &Config) -> Result<(), SumiError> {
    if config.display {
        display_parsed_commit(parsed_commit, &config.format)?;
    }
    if !config.quiet {
        let active_rule_count = count_active_rules(config);
        if !config.quiet && active_rule_count > 0 {
            info!(
                "✅ All {} {} passed.",
                active_rule_count,
                pluralize(active_rule_count, "check", "checks")
            );
        }
    }
    Ok(())
}

fn handle_failure(errors: &[SumiError]) -> Result<ParsedCommit, SumiError> {
    display_errors(errors);
    Err(SumiError::GeneralError {
        details: format!(
            "Found {} linting {}",
            errors.len(),
            pluralize(errors.len(), "error", "errors")
        ),
    })
}

fn display_errors(errors: &[SumiError]) {
    for err in errors.iter() {
        eprintln!("️❗ {}", err);
    }
}
