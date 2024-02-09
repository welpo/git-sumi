use crate::lint::constants::config_descriptions::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{self, BufReader, Read, Write};
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use strum::IntoEnumIterator;
use strum_macros::{AsRefStr, EnumIter};

use super::SumiError;
use crate::args::Opt;

const CONFIG_FILE_NAME: &str = "sumi.toml";

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(default)]
pub struct Config {
    pub quiet: bool,
    pub display: bool,
    pub format: ParsedCommitDisplayFormat,
    pub split_lines: bool,
    pub gitmoji: bool,
    pub description_case: DescriptionCase,
    pub imperative: bool,
    pub no_period: bool,
    pub max_header_length: usize,
    pub max_body_length: usize,
    pub whitespace: bool,
    pub conventional: bool,
    pub scopes_allowed: Vec<String>,
    pub types_allowed: Vec<String>,
    pub header_pattern: String,
}

pub trait Configurable {
    fn configure(&self, config: &mut Config);
}

/// Display format for the parsed commit message.
#[derive(Debug, Clone, Serialize, Deserialize, EnumIter, AsRefStr, Default)]
#[serde(rename_all = "lowercase")]
pub enum ParsedCommitDisplayFormat {
    #[default]
    Cli,
    Json,
    Table,
    Toml,
}

#[derive(Debug, Clone, Serialize, Deserialize, EnumIter, AsRefStr, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum DescriptionCase {
    #[default]
    Any,
    Lower,
    Upper,
}

impl FromStr for ParsedCommitDisplayFormat {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ParsedCommitDisplayFormat::iter()
            .find(|variant| s.eq_ignore_ascii_case(variant.as_ref()))
            .ok_or_else(|| {
                format!(
                    "Unknown format '{}'. Supported formats: {}",
                    s,
                    ParsedCommitDisplayFormat::iter()
                        .map(|v| v.as_ref().to_lowercase())
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            })
    }
}

impl FromStr for DescriptionCase {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        DescriptionCase::iter()
            .find(|variant| s.eq_ignore_ascii_case(variant.as_ref()))
            .ok_or_else(|| {
                format!(
                    "Unknown case '{}'. Supported cases: {}",
                    s,
                    DescriptionCase::iter()
                        .map(|v| v.as_ref().to_lowercase())
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            })
    }
}

/// Options to initialise git-sumi config.
#[derive(Debug, Clone, Serialize, Deserialize, EnumIter, AsRefStr)]
pub enum InitOption {
    #[strum(serialize = "commit-msg")]
    CommitMsg,
    Config,
    Hooks,
    #[strum(serialize = "prepare-commit-msg")]
    PrepareCommitMsg,
}

impl FromStr for InitOption {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        InitOption::iter()
            .find(|variant| s.eq_ignore_ascii_case(variant.as_ref()))
            .ok_or_else(|| {
                format!(
                    "Unknown option '{}'. Supported options: {}",
                    s,
                    InitOption::iter()
                        .map(|v| v.as_ref().to_lowercase())
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            })
    }
}

type IsModifiedFn<'a> = Box<dyn Fn(&Config, &Config) -> bool + 'a>;
type CurrentValueFn<'a> = Box<dyn Fn(&Config) -> String + 'a>;

struct RuleMeta<'a> {
    is_modified: IsModifiedFn<'a>,
    description: &'a str,
    current_value: CurrentValueFn<'a>,
}

fn rules_metadata<'a>() -> Vec<RuleMeta<'a>> {
    vec![
        RuleMeta {
            is_modified: Box::new(|c, d| c.conventional != d.conventional),
            description: CONVENTIONAL.short,
            current_value: Box::new(|c| c.conventional.to_string()),
        },
        RuleMeta {
            is_modified: Box::new(|c, d| c.imperative != d.imperative),
            description: IMPERATIVE.short,
            current_value: Box::new(|c| c.imperative.to_string()),
        },
        RuleMeta {
            is_modified: Box::new(|c, d| c.gitmoji != d.gitmoji),
            description: GITMOJI.short,
            current_value: Box::new(|c| c.gitmoji.to_string()),
        },
        RuleMeta {
            is_modified: Box::new(|c, d| c.whitespace != d.whitespace),
            description: WHITESPACE.short,
            current_value: Box::new(|c| c.whitespace.to_string()),
        },
        RuleMeta {
            is_modified: Box::new(|c, d| c.no_period != d.no_period),
            description: NO_PERIOD.short,
            current_value: Box::new(|c| c.no_period.to_string()),
        },
        RuleMeta {
            is_modified: Box::new(|c, d| c.description_case != d.description_case),
            description: DESCRIPTION_CASE.short,
            current_value: Box::new(|c| c.description_case.as_ref().to_string()),
        },
        RuleMeta {
            is_modified: Box::new(|c, d| c.max_header_length != d.max_header_length),
            description: MAX_HEADER_LENGTH.short,
            current_value: Box::new(|c| c.max_header_length.to_string()),
        },
        RuleMeta {
            is_modified: Box::new(|c, d| c.max_body_length != d.max_body_length),
            description: MAX_BODY_LENGTH.short,
            current_value: Box::new(|c| c.max_body_length.to_string()),
        },
        RuleMeta {
            is_modified: Box::new(|c, d| c.scopes_allowed != d.scopes_allowed),
            description: SCOPES_ALLOWED.short,
            current_value: Box::new(|c| c.scopes_allowed.join(", ")),
        },
        RuleMeta {
            is_modified: Box::new(|c, d| c.types_allowed != d.types_allowed),
            description: TYPES_ALLOWED.short,
            current_value: Box::new(|c| c.types_allowed.join(", ")),
        },
        RuleMeta {
            is_modified: Box::new(|c, d| c.header_pattern != d.header_pattern),
            description: HEADER_PATTERN.short,
            current_value: Box::new(|c| c.header_pattern.clone()),
        },
    ]
}

pub fn count_active_rules(config: &Config) -> usize {
    let default_config = Config::default();
    let rules_meta = rules_metadata();
    rules_meta
        .into_iter()
        .filter(|rule_meta| (rule_meta.is_modified)(config, &default_config))
        .count()
}

/// - For boolean flags, sets the `Config` field to `true` if the field is `true`.
/// - For optional values, updates the `Config` field only if the field has a value.
/// - For lists, updates the `Config` field only if the list is not empty.
macro_rules! update_field {
    ($config_field:expr, $self_field:expr) => {
        if let Some(val) = $self_field {
            $config_field = val;
        }
    };
    ($config_field:expr, $self_field:expr, option) => {
        if let Some(val) = &$self_field {
            $config_field = val.clone();
        }
    };
    ($config_field:expr, $self_field:expr, list) => {
        if !$self_field.is_empty() {
            $config_field = $self_field.clone();
        }
    };
}

impl Configurable for Opt {
    fn configure(&self, config: &mut Config) {
        update_field!(config.gitmoji, self.gitmoji);
        update_field!(config.conventional, self.conventional);
        update_field!(config.split_lines, self.split_lines);
        update_field!(config.quiet, self.quiet);
        update_field!(config.display, self.display);
        update_field!(config.format, self.format, option);
        update_field!(config.description_case, self.description_case, option);
        update_field!(config.imperative, self.imperative);
        update_field!(config.whitespace, self.whitespace);
        update_field!(config.no_period, self.no_period);
        update_field!(config.max_body_length, self.max_body_length, option);
        update_field!(config.max_header_length, self.max_header_length, option);
        update_field!(config.scopes_allowed, self.scopes_allowed, list);
        update_field!(config.types_allowed, self.types_allowed, list);
        update_field!(config.header_pattern, self.header_pattern, option);
    }
}

pub fn init_config(init_option: InitOption) -> Result<(), SumiError> {
    match init_option {
        InitOption::Config => {
            Config::init_config()?;
        }
        InitOption::PrepareCommitMsg => {
            init_prepare_commit_msg_hook()?;
        }
        InitOption::CommitMsg => {
            init_commit_msg_hook()?;
        }
        InitOption::Hooks => {
            init_commit_msg_hook()?;
            init_prepare_commit_msg_hook()?;
        }
    }
    Ok(())
}

const COMMIT_MSG_HOOK: &str = r#"#!/usr/bin/env bash

# Commit-msg hook generated by git-sumi.
# For more information and documentation, visit: https://sumi.rs

set -e  # Exit on any error.

# Check if git-sumi is installed.
if ! command -v git-sumi &> /dev/null
then
    echo "git-sumi is not installed. Please install it. See https://sumi.rs for instructions."
    echo "Alternatively, edit or remove the commit-msg hook in .git/hooks/commit-msg."
    exit 1
fi

git-sumi -- "$(cat $1)"  # Exit with error if linting fails.
"#;

fn init_commit_msg_hook() -> Result<(), SumiError> {
    let git_dir = Path::new(".git");
    ensure_git_repository(git_dir)?;
    let hooks_dir = git_dir.join("hooks");
    create_directory_if_not_exists(&hooks_dir)?;
    let hook_path = hooks_dir.join("commit-msg");
    write_commit_hook_if_needed(&hook_path, COMMIT_MSG_HOOK)?;
    #[cfg(unix)]
    set_executable_permission(&hook_path)?;
    Ok(())
}

fn ensure_git_repository(git_dir: &Path) -> Result<(), SumiError> {
    if !git_dir.exists() {
        return Err(SumiError::GeneralError {
            details: "No .git directory found. Are you in a Git repository?".to_string(),
        });
    }
    Ok(())
}

fn create_directory_if_not_exists(dir: &Path) -> Result<(), SumiError> {
    if !dir.exists() {
        fs::create_dir_all(dir).map_err(|e| SumiError::GeneralError {
            details: e.to_string(),
        })?;
    }
    Ok(())
}

fn write_commit_hook_if_needed(hook_path: &Path, hook_content: &str) -> Result<(), SumiError> {
    if hook_path.exists() {
        let overwrite = prompt_overwrite(hook_path.to_str().unwrap())?;
        if !overwrite {
            return Ok(());
        }
    }
    fs::write(hook_path, hook_content).map_err(|e| SumiError::GeneralError {
        details: e.to_string(),
    })?;
    Ok(())
}

fn prompt_overwrite(filename: &str) -> Result<bool, SumiError> {
    let mut input = String::new();
    print!("File '{}' already exists. Overwrite? (y/n) [n] ", filename);
    io::stdout().flush()?;
    input.clear();
    io::stdin().read_line(&mut input)?;
    match input.trim().to_lowercase().as_str() {
        "y" => Ok(true),
        _ => Ok(false),
    }
}

fn set_executable_permission(file_path: &Path) -> Result<(), SumiError> {
    let mut permissions = fs::metadata(file_path)
        .map_err(|e| SumiError::GeneralError {
            details: e.to_string(),
        })?
        .permissions();
    #[cfg(unix)]
    permissions.set_mode(0o755);
    fs::set_permissions(file_path, permissions).map_err(|e| SumiError::GeneralError {
        details: e.to_string(),
    })?;
    Ok(())
}

const PREPARE_COMMIT_MSG_HOOK: &str = r#"#!/usr/bin/env bash
set -euo pipefail

COMMIT_MSG_FILE="${1}"
COMMIT_SOURCE="${2:-}"

# Do nothing if the message was created with `git commit -m`.
if [ "${COMMIT_SOURCE}" = "message" ]; then
    exit 0
fi

CUSTOM_MSG="$(git-sumi --prepare-commit-message)"

# Prepend the rules to the commit message template.
TEMP_FILE="$(mktemp)"
echo "${CUSTOM_MSG}" > "${TEMP_FILE}"
cat "${COMMIT_MSG_FILE}" >> "${TEMP_FILE}"
mv "${TEMP_FILE}" "${COMMIT_MSG_FILE}"
"#;

fn init_prepare_commit_msg_hook() -> Result<(), SumiError> {
    let git_dir = Path::new(".git");
    ensure_git_repository(git_dir)?;
    let hooks_dir = git_dir.join("hooks");
    create_directory_if_not_exists(&hooks_dir)?;
    let hook_path = hooks_dir.join("prepare-commit-msg");
    write_commit_hook_if_needed(&hook_path, PREPARE_COMMIT_MSG_HOOK)?;
    #[cfg(unix)]
    set_executable_permission(&hook_path)?;
    Ok(())
}

pub fn generate_commit_msg_hook_content(config: &Config) -> Result<(), SumiError> {
    let metadata_list = rules_metadata();

    let template_content = metadata_list
        .into_iter()
        .filter(|meta| (meta.is_modified)(config, &Config::default()))
        .map(|meta| {
            let description_str = meta.description;
            let value_str = (meta.current_value)(config);
            if value_str == "true" {
                // We don't need to show ": true" for booleans.
                format!("# {}.\n", description_str)
            } else {
                // Show description + value.
                format!("# {}: {}\n", description_str, value_str)
            }
        })
        .collect::<Vec<String>>()
        .join("");

    let header_comment = format!(
        "# git-sumi rules enabled:\n\
         {}",
        template_content
    );

    // Print to stdout.
    println!("{}", header_comment);
    Ok(())
}

impl Config {
    pub fn new() -> Self {
        Config::default()
    }

    pub fn update_from<T: Configurable>(&mut self, args: &T) {
        args.configure(self);
    }

    pub fn init_config() -> Result<(), SumiError> {
        let default_config = Self::default();
        let toml = toml::to_string(&default_config)?;

        let config_keys_and_rules = [
            ("quiet", (&QUIET, false)),
            ("display", (&DISPLAY, false)),
            ("format", (&FORMAT, false)),
            ("split_lines", (&SPLIT_LINES, false)),
            ("gitmoji", (&GITMOJI, true)),
            ("description_case", (&DESCRIPTION_CASE, true)),
            ("imperative", (&IMPERATIVE, true)),
            ("no_period", (&NO_PERIOD, true)),
            ("whitespace", (&WHITESPACE, true)),
            ("max_header_length", (&MAX_HEADER_LENGTH, true)),
            ("max_body_length", (&MAX_BODY_LENGTH, true)),
            ("conventional", (&CONVENTIONAL, true)),
            ("scopes_allowed", (&SCOPES_ALLOWED, true)),
            ("types_allowed", (&TYPES_ALLOWED, true)),
            ("header_pattern", (&HEADER_PATTERN, true)),
        ];

        let config_comments: HashMap<&str, String> = config_keys_and_rules
            .iter()
            .map(|&(key, (rule, is_rule))| (key, format_description(rule, is_rule)))
            .collect();

        fn format_description(description: &RuleDescription, is_rule: bool) -> String {
            let prefix = if is_rule { "Rule: " } else { "" };
            let mut formatted_description = format!("\n# {}{}.\n", prefix, description.short);
            if let Some(extra) = description.extra {
                formatted_description += &format!("# {}.\n", extra);
            }
            formatted_description
        }

        let mut toml_with_comments = String::new();
        for line in toml.lines() {
            if let Some((key, value)) = line.split_once('=') {
                if let Some(comment) = config_comments.get(key.trim()) {
                    toml_with_comments.push_str(comment);
                }
                toml_with_comments.push_str(&format!("{}={}\n", key, value));
            }
        }

        let toml_with_comments = format!(
            "# git-sumi ~ configuration file\n\
            # Config: https://sumi.rs/docs/configuration\n\
            # Rules: https://sumi.rs/docs/rules\n\
            {}",
            toml_with_comments
        );

        let current_dir = std::env::current_dir()?;
        let default_path = current_dir.join(CONFIG_FILE_NAME);
        if default_path.exists() {
            match prompt_overwrite(default_path.to_str().unwrap()) {
                Err(e) => return Err(e),
                Ok(true) => {}
                Ok(false) => return Ok(()),
            }
        }

        std::fs::write(default_path, toml_with_comments)?;

        Ok(())
    }
}

/// Initialises a `Config` object and loads values into it.
/// It first creates a default `Config`, then attempts to find and load a config file.
/// Then, it updates the `Config` with CLI arguments, overriding any values from the config file.
/// Finally, it validates the `Config` and returns it, if valid.
/// If the special value "none" is passed as the config path, returns a default config.
pub fn assemble_config(args: &Opt) -> Result<Config, SumiError> {
    let mut config = if args.config.as_deref() == Some("none") {
        Config::new()
    } else if let Some(custom_path) = args.config.as_ref() {
        load_config(custom_path)?
    } else if let Some(default_path) = find_config_file()? {
        load_config(default_path)?
    } else {
        Config::new()
    };
    config.update_from(args);
    adjust_config(&mut config);
    Ok(config)
}

/// Finds and returns the path of the configuration file.
///
/// The function searches for `sumi.toml` or `.sumi.toml`:
///  - In the current directory and its parent directories.
///  - In the user's home directory.
///  - In a "sumi" folder within the user's home directory.
///
/// If set to "none", returns the default configuration.
fn find_config_file() -> Result<Option<PathBuf>, SumiError> {
    let filenames = ["sumi.toml", ".sumi.toml"];
    let current_dir = fs::canonicalize(std::env::current_dir()?)?;

    if let Some(path) = find_config_in_ancestors(current_dir.clone(), &filenames) {
        return Ok(Some(path));
    }
    if let Some(path) = find_config_in_home(&filenames) {
        return Ok(Some(path));
    }

    Ok(None)
}

fn find_config_in_ancestors(starting_dir: PathBuf, filenames: &[&str]) -> Option<PathBuf> {
    let mut current = starting_dir;
    loop {
        if let Some(path) = find_in_dir(current.clone(), filenames) {
            return Some(path);
        }
        if !current.pop() {
            break;
        }
    }
    None
}

fn find_config_in_home(filenames: &[&str]) -> Option<PathBuf> {
    if let Some(mut home_dir) = dirs::home_dir() {
        if let Some(path) = find_in_dir(home_dir.clone(), filenames) {
            return Some(path);
        }
        home_dir.push("sumi");
        return find_in_dir(home_dir, filenames);
    }
    None
}

fn find_in_dir(directory: PathBuf, filenames: &[&str]) -> Option<PathBuf> {
    for filename in filenames {
        let mut check_path = directory.clone();
        check_path.push(filename);
        if check_path.exists() {
            return Some(check_path);
        }
    }
    None
}

/// Loads the TOML configuration file from the given path.
/// Returns the parsed configuration or an error if the file cannot be found or parsed.
fn load_config<P: AsRef<Path>>(file_path: P) -> Result<Config, SumiError> {
    validate_file_path(&file_path)?;
    let file_contents_as_string = read_file_into_string(&file_path)?;
    let parsed_configuration = toml::from_str(&file_contents_as_string)?;
    Ok(parsed_configuration)
}

fn validate_file_path<P: AsRef<Path>>(path: P) -> Result<(), SumiError> {
    let path_ref = path.as_ref();
    if !path_ref.exists() {
        return Err(SumiError::ConfigFileNotFound {
            path: path_ref.to_string_lossy().into_owned(),
        });
    }
    if path_ref.is_dir() {
        return Err(SumiError::PathIsDirectory {
            path: path_ref.to_string_lossy().into_owned(),
        });
    }
    Ok(())
}

fn read_file_into_string<P: AsRef<Path>>(file_path: P) -> Result<String, std::io::Error> {
    let file_handle = File::open(file_path)?;
    let mut buffered_reader = BufReader::new(file_handle);
    let mut file_contents = String::new();
    buffered_reader.read_to_string(&mut file_contents)?;
    Ok(file_contents)
}

fn adjust_config(config: &mut Config) {
    if !config.types_allowed.is_empty() || !config.scopes_allowed.is_empty() {
        config.conventional = true;
    }
}
