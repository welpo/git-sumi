use super::{CommitParser, Config, ParsedCommit, SumiError};

pub struct ConventionalCommitParser;

impl CommitParser for ConventionalCommitParser {
    fn parse(&self, commit: &str, config: &Config) -> Result<ParsedCommit, SumiError> {
        let header = self.get_commit_header(commit)?;
        let (gitmoji, commit) = self.parse_and_remove_emoji(commit, config)?;
        // Parse the commit message sans gitmoji using the git_conventional crate.
        let conventional_commit = match git_conventional::Commit::parse(&commit) {
            Ok(commit) => commit,
            Err(err) => match err.kind() {
                git_conventional::ErrorKind::InvalidBody => {
                    return Err(SumiError::SeparateHeaderFromBody);
                }
                _ => {
                    return Err(err.into());
                }
            },
        };

        let footers = conventional_commit
            .footers()
            .iter()
            .map(|footer| footer.to_string())
            .collect::<Vec<String>>();

        let references = self.extract_references(&commit);

        let parsed_commit = ParsedCommit {
            header,
            gitmoji,
            commit_type: Some(conventional_commit.type_().to_string()),
            scope: conventional_commit.scope().map(|s| s.to_string()),
            description: conventional_commit.description().to_string(),
            body: conventional_commit.body().map(|s| s.to_string()),
            footers: Some(footers).filter(|f| !f.is_empty()),
            is_breaking: Some(conventional_commit.breaking()),
            breaking_description: conventional_commit
                .breaking_description()
                .map(|s| s.to_string()),
            references,
        };

        Ok(parsed_commit)
    }
}
