use thiserror::Error;

#[derive(Error, Debug)]
pub enum SumiError {
    #[error("{details}")]
    GeneralError { details: String },

    #[error("{lines_with_errors} out of {total_lines} {line_or_lines} failed linting. See the errors above")]
    SplitLinesErrors {
        lines_with_errors: usize,
        total_lines: usize,
        line_or_lines: String,
    },

    #[error("Failed to parse as a conventional commit: '{reason}'")]
    FailedConventionalParse { reason: String },

    #[error("Separate header from body with a blank line")]
    SeparateHeaderFromBody,

    #[error("Header must not end with a period")]
    HeaderEndsWithPeriod,

    #[error("Description must start in lowercase. Try '{lowercase_header}'")]
    DescriptionNotLowercase { lowercase_header: String },

    #[error("Description must start with a capital letter. Try '{capitalized_description}'")]
    DescriptionNotTitleCase { capitalized_description: String },

    #[error("Line number {line_number} is too long ({line_length} > {max_length})")]
    LineTooLong {
        line_number: usize,
        line_length: usize,
        max_length: usize,
    },

    #[error("Header must contain exactly 1 emoji, found {found}")]
    IncorrectEmojiCount { found: usize },

    #[error("Invalid emoji: '{emoji}'. See the full list https://gitmoji.dev/")]
    InvalidEmoji { emoji: String },

    #[error("Description starts with a non-imperative verb: '{verb}'. Use an imperative verb like 'fix', instead of 'fixes' or 'fixing'")]
    NonImperativeVerb { verb: String },

    #[error("Invalid commit type '{type_found}'. Allowed types are: [{allowed_types}]")]
    InvalidCommitType {
        type_found: String,
        allowed_types: String,
    },

    #[error("Invalid commit scope '{scope_found}'. Allowed scopes are: [{allowed_scopes}]")]
    InvalidCommitScope {
        scope_found: String,
        allowed_scopes: String,
    },

    #[error(
        "No rules enabled. Enable at least one rule with --conventional, --imperative, --whitespace… Or use --commit or --display. Try `git-sumi --help`"
    )]
    NoRulesEnabled,

    #[error("Incompatible rules enabled: '{rule1}' and '{rule2}'")]
    IncompatibleRules { rule1: String, rule2: String },

    #[error("Header must not be empty")]
    EmptyCommitHeader,

    #[error("Header regex pattern '{pattern}' is invalid")]
    InvalidRegexPattern { pattern: String },

    #[error("Header does not match the required pattern: '{pattern}'")]
    HeaderPatternMismatch { pattern: String },

    #[error(transparent)]
    InvalidConventionalCommit(#[from] git_conventional::Error),

    #[error("Could not serialize to {format}: {detail}")]
    SerializationError { format: String, detail: String },

    #[error("Failed to commit changes. {0}")]
    ErrorWhileCommitting(String),

    #[error(transparent)]
    InputOutputError(#[from] std::io::Error),

    #[error(transparent)]
    ConfigDumpError(#[from] toml::ser::Error),

    #[error(transparent)]
    ConfigLoadError(#[from] toml::de::Error),

    #[error(
        "Configuration file '{path}' not found. Make sure the file exists at the specified path"
    )]
    ConfigFileNotFound { path: String },

    #[error("The specified path '{path}' is a directory, not a configuration file")]
    PathIsDirectory { path: String },
}

pub fn pluralize<'a>(count: usize, singular: &'a str, plural: &'a str) -> &'a str {
    if count == 1 {
        singular
    } else {
        plural
    }
}
