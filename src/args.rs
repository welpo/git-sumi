use crate::config::{DescriptionCase, InitOption, ParsedCommitDisplayFormat};
use crate::lint::constants::config_descriptions;
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
    #[arg(index = 1, help = config_descriptions::COMMIT_MESSAGE)]
    pub commit_message: Option<String>,

    #[arg(
        long,
        value_name = "OPTION",
        num_args = 0..=1,
        required = false,
        default_missing_value = "config",
        help = config_descriptions::INIT
    )]
    pub init: Option<InitOption>,

    /// Outputs enabled rules' description as bash comments for the prepare-commit-msg hook.
    #[arg(long, num_args = 0, hide = true)]
    pub prepare_commit_message: bool,

    #[arg(long,
        value_enum,
        required = false,
        value_name = "SHELL",
        help = config_descriptions::GENERATE_SHELL_COMPLETION
    )]
    pub generate_shell_completion: Option<Shell>,

    #[arg(long, env = "GIT_SUMI_CONFIG", help = config_descriptions::CONFIG)]
    pub config: Option<String>,

    /// Suppress progress messages.
    #[arg(
        short = 'q',
        num_args = 0,
        default_missing_value = "true",
        long,
        env = "GIT_SUMI_QUIET",
        help = config_descriptions::QUIET.short
    )]
    pub quiet: Option<bool>,

    /// Process each non-empty line as an individual commit.
    #[arg(
        short = 's',
        long,
        env = "GIT_SUMI_SPLIT_LINES",
        num_args = 0,
        default_missing_value = "true",
        help = config_descriptions::SPLIT_LINES.short
    )]
    pub split_lines: Option<bool>,

    /// Display the parsed commit message.
    #[arg(
        short = 'd',
        long,
        env = "GIT_SUMI_DISPLAY",
        num_args = 0,
        default_missing_value = "true",
        help = config_descriptions::DISPLAY.short
    )]
    pub display: Option<bool>,

    /// Specify the output format for displaying the parsed commit message.
    /// Options: "cli", "table", "json", "toml". Default: "cli"
    #[arg(short = 'f',
        long,
        env = "GIT_SUMI_FORMAT",
        help = config_descriptions::FORMAT.short
    )]
    pub format: Option<ParsedCommitDisplayFormat>,

    /// Commit the message after successful linting.
    #[arg(short = 'c', long, help=config_descriptions::COMMIT)]
    pub commit: bool,

    /// Force a commit, regardless of linting errors.
    #[arg(long, help=config_descriptions::FORCE)]
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
        help_heading = "Rules",
        help = config_descriptions::CONVENTIONAL.short
    )]
    pub conventional: Option<bool>,

    /// Use the imperative mood in the description.
    #[arg(
        short = 'I',
        long,
        env = "GIT_SUMI_IMPERATIVE",
        num_args = 0,
        default_missing_value = "true",
        help_heading = "Rules",
        help = config_descriptions::IMPERATIVE.short
    )]
    pub imperative: Option<bool>,

    /// Include one valid Gitmoji.
    #[arg(
        short = 'G',
        long,
        env = "GIT_SUMI_GITMOJI",
        num_args = 0,
        default_missing_value = "true",
        help_heading = "Rules",
        help = config_descriptions::GITMOJI.short
    )]
    pub gitmoji: Option<bool>,

    /// Disallow leading/trailing whitespace and consecutive spaces.
    #[arg(
        short = 'W',
        long,
        env = "GIT_SUMI_WHITESPACE",
        num_args = 0,
        default_missing_value = "true",
        help_heading = "Rules",
        help = config_descriptions::WHITESPACE.short
    )]
    pub whitespace: Option<bool>,

    /// Commit description must start with the selected case.
    /// Options: "lower", "upper", "any". Default: "any".
    #[arg(
        short = 'E',
        long,
        env = "GIT_SUMI_DESCRIPTION_CASE",
        value_name = "CASE",
        help_heading = "Rules",
        help = config_descriptions::DESCRIPTION_CASE.short
    )]
    pub description_case: Option<DescriptionCase>,

    /// Do not end commit header with a period.
    #[arg(
        short = 'P',
        long,
        env = "GIT_SUMI_NO_PERIOD",
        num_args = 0,
        default_missing_value = "true",
        help_heading = "Rules",
        help = config_descriptions::NO_PERIOD.short
    )]
    pub no_period: Option<bool>,

    /// Limit the header to the specified length.
    #[arg(short = 'H',
        long,
        env = "GIT_SUMI_MAX_HEADER_LENGTH",
        value_parser = clap::value_parser!(usize),
        help_heading = "Rules",
        help = config_descriptions::MAX_HEADER_LENGTH.short
    )]
    pub max_header_length: Option<usize>,

    /// Wrap the body at the specified length.
    #[arg(short = 'B',
        long,
        env = "GIT_SUMI_MAX_BODY_LENGTH",
        value_parser = clap::value_parser!(usize),
        help_heading = "Rules",
        help = config_descriptions::MAX_BODY_LENGTH.short
    )]
    pub max_body_length: Option<usize>,

    /// Only allow the specified, comma-separated commit scopes.
    #[arg(
        short = 'S',
        long,
        env = "GIT_SUMI_SCOPES_ALLOWED",
        value_name = "SCOPES",
        help_heading = "Rules",
        help = config_descriptions::SCOPES_ALLOWED.short
    )]
    pub scopes_allowed: Vec<String>,

    /// Only allow the specified, comma-separated commit types.
    #[arg(
        short = 'T',
        long,
        env = "GIT_SUMI_TYPES_ALLOWED",
        value_name = "TYPES",
        help_heading = "Rules",
        help = config_descriptions::TYPES_ALLOWED.short
    )]
    pub types_allowed: Vec<String>,

    /// Commit header must match the specified (regex) pattern.
    #[arg(
        short = 'R',
        long,
        env = "GIT_SUMI_HEADER_PATTERN",
        value_name = "PATTERN",
        help_heading = "Rules",
        help = config_descriptions::HEADER_PATTERN.short
    )]
    pub header_pattern: Option<String>,
}
