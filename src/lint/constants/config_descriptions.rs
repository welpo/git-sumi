// CLI-exclusive --help descriptions.
pub const COMMIT_MESSAGE: &str = "Commit message to lint. Alternatively, read from STDIN";
pub const INIT: &str =
    "Initialize the default configuration ('config'), commit-msg hook ('commit-msg'), prepare-commit-msg hook ('prepare-commit-msg') or both hooks ('hooks')";
pub const GENERATE_SHELL_COMPLETION: &str =
    "Generate shell completion script for the specified shell";
pub const CONFIG: &str = "Path to a TOML configuration file";
pub const COMMIT: &str = "Commit the message after successful linting";
pub const FORCE: &str = "Force the commit even if linting fails";

// Structure for rule descriptions.
// The 'short' version is used in --help.
// The 'extra' version is used to create the default config.
pub struct RuleDescription {
    pub short: &'static str,
    pub extra: Option<&'static str>,
}

// General configuration.
pub const QUIET: RuleDescription = RuleDescription {
    short: "Suppresses progress messages",
    extra: None,
};
pub const DISPLAY: RuleDescription = RuleDescription {
    short: "Displays parsed commit message",
    extra: None,
};
pub const FORMAT: RuleDescription = RuleDescription {
    short: "Sets display format: cli, json, table, toml",
    extra: None,
};
pub const SPLIT_LINES: RuleDescription = RuleDescription {
    short: "Processes each non-empty line as an individual commit",
    extra: None,
};

// Rules.
pub const GITMOJI: RuleDescription = RuleDescription {
    short: "Include one valid Gitmoji",
    extra: Some("See https://gitmoji.dev/"),
};

pub const CONVENTIONAL: RuleDescription = RuleDescription {
    short: "Follow Conventional Commits format",
    extra: Some("See https://www.conventionalcommits.org/"),
};

pub const IMPERATIVE: RuleDescription = RuleDescription {
    short: "Use the imperative mood in the description",
    extra: Some("Example: 'Fix bug' instead of 'Fixed bug'"),
};

pub const WHITESPACE: RuleDescription = RuleDescription {
    short: "No leading, trailing, or consecutive spaces",
    extra: None,
};

pub const DESCRIPTION_CASE: RuleDescription = RuleDescription {
    short: "Description must start with the specified case",
    extra: Some("Options: 'any', 'lower', 'upper'"),
};

pub const NO_PERIOD: RuleDescription = RuleDescription {
    short: "Do not end commit header with a period",
    extra: None,
};

pub const MAX_HEADER_LENGTH: RuleDescription = RuleDescription {
    short: "Header length limit",
    extra: Some("A value of 0 disables the rule"),
};

pub const MAX_BODY_LENGTH: RuleDescription = RuleDescription {
    short: "Body line length limit",
    extra: Some("A value of 0 disables the rule"),
};

pub const SCOPES_ALLOWED: RuleDescription = RuleDescription {
    short: "List of allowed commit scopes",
    extra: Some("An empty list allows all scopes. Example: [\"docs\", \"cli\"]"),
};

pub const TYPES_ALLOWED: RuleDescription = RuleDescription {
    short: "List of allowed commit types",
    extra: Some("An empty list allows all types. Example: [\"feat\", \"fix\", \"docs\"]"),
};

pub const HEADER_PATTERN: RuleDescription = RuleDescription {
    short: "Header must match regex pattern",
    extra: Some("Example: '^JIRA-\\d+:'"),
};
