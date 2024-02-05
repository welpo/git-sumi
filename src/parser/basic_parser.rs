use super::{CommitParser, Config, ParsedCommit, SumiError};

pub struct BasicCommitParser;

impl CommitParser for BasicCommitParser {
    fn parse(&self, commit: &str, config: &Config) -> Result<ParsedCommit, SumiError> {
        let header = self.get_commit_header(commit)?;
        let (gitmoji, commit) = self.parse_and_remove_emoji(commit, config)?;
        let description = self.get_commit_header(&commit)?;
        let lines: Vec<&str> = commit.lines().collect();
        let body_start_index = self.find_body_start_index(&lines);
        let parsed_body = self.extract_body(&lines, body_start_index);
        let references = self.extract_references(&commit);
        Ok(ParsedCommit {
            header: header.clone(),
            gitmoji,
            description,
            body: parsed_body,
            references,
            ..Default::default()
        })
    }
}

impl BasicCommitParser {
    fn find_body_start_index(&self, lines: &[&str]) -> usize {
        lines
            .iter()
            .skip(1) // Skip the header.
            .position(|&line| !line.is_empty())
            .unwrap_or(0)
            + 1 // Add 1 to compensate for the header we skipped.
    }

    fn extract_body(&self, lines: &[&str], start_index: usize) -> Option<String> {
        lines
            .get(start_index..)
            .filter(|line| !line.is_empty())
            .map(|line| line.join("\n").trim_start_matches('\n').to_string())
    }
}
