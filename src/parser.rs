mod basic_parser;
mod conventional_commit_parser;
pub mod parsed_commit;

use crate::config::Config;
use crate::SumiError;
use lazy_static::lazy_static;
use regex::Regex;
use std::iter::Peekable;
use std::vec::IntoIter;

use self::basic_parser::BasicCommitParser;
use self::conventional_commit_parser::ConventionalCommitParser;
pub use self::parsed_commit::ParsedCommit;

pub fn handle_parsing(
    commit: &str,
    config: &Config,
    errors: &mut Vec<SumiError>,
) -> Result<ParsedCommit, SumiError> {
    if config.conventional {
        match ConventionalCommitParser.parse(commit, config) {
            Ok(parsed) => Ok(parsed),
            Err(e) => {
                errors.push(SumiError::FailedConventionalParse {
                    reason: format!("{}", e),
                });
                BasicCommitParser.parse(commit, config)
            }
        }
    } else {
        BasicCommitParser.parse(commit, config)
    }
}

trait CommitParser {
    fn parse(&self, commit: &str, config: &Config) -> Result<ParsedCommit, SumiError>;

    fn get_commit_header(&self, commit: &str) -> Result<String, SumiError> {
        let header = commit.lines().next().ok_or(SumiError::EmptyCommitHeader)?;
        Ok(header.to_string())
    }

    fn parse_and_remove_emoji(
        &self,
        commit: &str,
        config: &Config,
    ) -> Result<(Option<Vec<String>>, String), SumiError> {
        let commit_title = self.get_commit_header(commit)?;
        if config.gitmoji {
            if let Some(emojis) = parse_gitmoji(&commit_title) {
                let commit_sans_gitmoji = remove_gitmoji(commit, &emojis);
                return Ok((Some(emojis), commit_sans_gitmoji));
            }
        }
        Ok((None, commit.to_string()))
    }

    /// Returns a list of references (issue/PR number and commit hashes) from a commit.
    fn extract_references(&self, commit: &str) -> Option<Vec<String>> {
        let extracted_references: Vec<String> = commit
            .lines()
            .flat_map(|commit_line| REFERENCE_REGEX.captures_iter(commit_line))
            .map(|capture_group| capture_group[1].to_string())
            .collect();

        // Return None if no references were found.
        Some(extracted_references).filter(|references| !references.is_empty())
    }
}

lazy_static! {
    // Regex: #(issue/PR number) | 7-40 hexadecimal character SHAs.
    static ref REFERENCE_REGEX: Regex = Regex::new(r"(#\d+|\b[0-9a-f]{7,40}\b)").expect("Failed to compile regex");
}

fn parse_gitmoji(commit_title: &str) -> Option<Vec<String>> {
    let emojis = extract_gitmoji(commit_title);
    if emojis.is_empty() {
        return None;
    }
    Some(emojis)
}

fn extract_gitmoji(title: &str) -> Vec<String> {
    // Find all emojis in the title, combining adjacent emojis.
    let emojis: Vec<String> = EMOJI_REGEX
        .find_iter(title)
        .map(|m| m.as_str().to_string())
        .collect();
    combine_adjacent_emojis(emojis)
}

lazy_static! {
    static ref EMOJI_REGEX: Regex =
        Regex::new(r"(:\w+:)|[\p{Emoji_Presentation}\p{Extended_Pictographic}\u{200D}]")
            .expect("Failed to compile regex");
}

// If we only counted the number of emojis without this function, we would mistakenly
// consider emojis that should be treated as a single unit (e.g., emojis combined with
// a zero-width joiner like "🧑‍💻" ) as multiple emojis.
// This function combines such adjacent emojis into single units to accurately count them.
/// e.g., "🧑‍💻" and "👨‍👩‍👧‍👦" become single units, even though they are composed of multiple emojis.
/// Returns the list of emojis with adjacent emojis combined.
fn combine_adjacent_emojis(emojis: Vec<String>) -> Vec<String> {
    let mut emojis = emojis.into_iter().peekable();
    let mut combined_emojis = Vec::new();
    while let Some(emoji) = emojis.next() {
        if emoji == "\u{200D}" {
            handle_zwj(&mut combined_emojis, &mut emojis);
        } else {
            combined_emojis.push(emoji);
        }
    }
    combined_emojis
}

fn combine_emojis(previous_emoji: String, zwj: String, next_emoji: String) -> String {
    format!("{}{}{}", previous_emoji, zwj, next_emoji)
}

fn handle_zwj(combined_emojis: &mut Vec<String>, emojis: &mut Peekable<IntoIter<String>>) {
    if let (Some(previous_emoji), Some(next_emoji)) =
        (combined_emojis.pop(), emojis.peek().cloned())
    {
        let combined = combine_emojis(previous_emoji, "\u{200D}".to_string(), next_emoji);
        combined_emojis.push(combined);
        emojis.next();
    }
}

fn remove_gitmoji(commit: &str, gitmojis: &[String]) -> String {
    let mut commit_sans_gitmoji = String::from(commit);
    for gitmoji in gitmojis {
        let full_gitmoji = format!("{}\\u{{fe0f}}?", regex::escape(gitmoji));
        let re = Regex::new(&format!(r"\s?{}\s?", full_gitmoji)).unwrap();
        commit_sans_gitmoji = re.replace(&commit_sans_gitmoji, "").to_string();
    }
    commit_sans_gitmoji.trim().to_string()
}
