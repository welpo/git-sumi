use crate::config::{DescriptionCase, InitOption, ParsedCommitDisplayFormat};
use clap::{builder::ArgPredicate, Parser};
use clap_complete::Shell;

#[derive(Parser, Debug)]
#[command(
    author = env!("CARGO_PKG_AUTHORS"),
    version = env!("CARGO_PKG_VERSION"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    after_help = "Visit https://sumi.rs to learn more about git-sumi",
)]

pub struct Opt {
    /// Commit message to lint. Alternatively, read from STDIN.
    #[arg(index = 1)]
    pub commit_message: Option<String>,

    /// Initialize the default configuration ("config", default value) or commit-msg hook ("hook").
    #[arg(
        long,
        value_name = "OPTION",
        num_args = 0..=1,
        required = false,
        default_missing_value = "config"
    )]
    pub init: Option<InitOption>,

    /// Generate shell completion script for the specified shell.
    #[arg(long, value_enum, required = false, value_name = "SHELL")]
    pub generate_shell_completion: Option<Shell>,

    /// Path to a TOML configuration file.
    #[arg(long, env = "GIT_SUMI_CONFIG")]
    pub config: Option<String>,

    /// Suppress progress messages.
    #[arg(
        short = 'q',
        num_args = 0,
        default_missing_value = "true",
        long,
        env = "GIT_SUMI_QUIET"
    )]
    pub quiet: Option<bool>,

    /// Process each non-empty line as an individual commit.
    #[arg(
        short = 's',
        long,
        env = "GIT_SUMI_SPLIT_LINES",
        num_args = 0,
        default_missing_value = "true"
    )]
    pub split_lines: Option<bool>,

    /// Display the parsed commit message.
    #[arg(
        short = 'd',
        long,
        env = "GIT_SUMI_DISPLAY",
        num_args = 0,
        default_missing_value = "true"
    )]
    pub display: Option<bool>,

    /// Specify the output format for displaying the parsed commit message.
    /// Options: "cli", "table", "json", "toml". Default: "cli"
    #[arg(short = 'f', long, env = "GIT_SUMI_FORMAT")]
    pub format: Option<ParsedCommitDisplayFormat>,

    /// Commit the message after successful linting.
    #[arg(short = 'c', long)]
    pub commit: bool,

    /// Force a commit, regardless of linting errors.
    #[arg(long)]
    pub force: bool,

    /// Follow Conventional Commits format.
    #[arg(short = 'C',
        long,
        env = "GIT_SUMI_CONVENTIONAL",
        num_args = 0,
        default_missing_value = "true",
        default_value_ifs([
            ("types_allowed", ArgPredicate::IsPresent, Some("true")),
            ("scopes_allowed", ArgPredicate::IsPresent, Some("true")),
            ]),
        help_heading = "Rules")]
    pub conventional: Option<bool>,

    /// Use the imperative mood in the description.
    #[arg(
        short = 'I',
        long,
        env = "GIT_SUMI_IMPERATIVE",
        num_args = 0,
        default_missing_value = "true"
    )]
    pub imperative: Option<bool>,

    /// Include one valid Gitmoji.
    #[arg(
        short = 'G',
        long,
        env = "GIT_SUMI_GITMOJI",
        num_args = 0,
        default_missing_value = "true"
    )]
    pub gitmoji: Option<bool>,

    /// Disallow leading/trailing whitespace and consecutive spaces.
    #[arg(
        short = 'W',
        long,
        env = "GIT_SUMI_WHITESPACE",
        num_args = 0,
        default_missing_value = "true"
    )]
    pub whitespace: Option<bool>,

    /// Commit description must start with the selected case.
    /// Options: "lower", "upper", "any". Default: "any".
    #[arg(
        short = 'E',
        long,
        env = "GIT_SUMI_DESCRIPTION_CASE",
        value_name = "CASE",
        help_heading = "Rules"
    )]
    pub description_case: Option<DescriptionCase>,

    /// Do not end commit header with a period.
    #[arg(
        short = 'P',
        long,
        env = "GIT_SUMI_NO_PERIOD",
        num_args = 0,
        default_missing_value = "true"
    )]
    pub no_period: Option<bool>,

    /// Limit the header to the specified length.
    #[arg(short = 'H', long, env = "GIT_SUMI_MAX_HEADER_LENGTH", value_parser = clap::value_parser!(usize), help_heading = "Rules")]
    pub max_header_length: Option<usize>,

    /// Wrap the body at the specified length.
    #[arg(short = 'B', long, env = "GIT_SUMI_MAX_BODY_LENGTH", value_parser = clap::value_parser!(usize), help_heading = "Rules")]
    pub max_body_length: Option<usize>,

    /// Only allow the specified, comma-separated commit scopes.
    #[arg(
        short = 'S',
        long,
        env = "GIT_SUMI_SCOPES_ALLOWED",
        value_name = "SCOPES",
        help_heading = "Rules"
    )]
    pub scopes_allowed: Vec<String>,

    /// Only allow the specified, comma-separated commit types.
    #[arg(
        short = 'T',
        long,
        env = "GIT_SUMI_TYPES_ALLOWED",
        value_name = "TYPES",
        help_heading = "Rules"
    )]
    pub types_allowed: Vec<String>,

    /// Commit header must match the specified (regex) pattern.
    #[arg(
        short = 'R',
        long,
        env = "GIT_SUMI_HEADER_PATTERN",
        value_name = "PATTERN",
        help_heading = "Rules"
    )]
    pub header_pattern: Option<String>,
}
