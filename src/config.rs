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
#[serde(rename_all = "lowercase")]
pub enum InitOption {
    Config,
    Hook,
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

pub fn count_active_rules(config: &Config) -> usize {
    let mut count = 0;
    count += config.whitespace as usize;
    count += (config.description_case != DescriptionCase::Any) as usize;
    count += config.no_period as usize;
    count += config.gitmoji as usize;
    count += config.imperative as usize;
    count += config.conventional as usize;
    count += (config.max_header_length > 0) as usize;
    count += (config.max_body_length > 0) as usize;
    count += (!config.scopes_allowed.is_empty()) as usize;
    count += (!config.types_allowed.is_empty()) as usize;
    count += (!config.header_pattern.is_empty()) as usize;

    count
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
        InitOption::Hook => {
            init_commit_hook()?;
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

fn init_commit_hook() -> Result<(), SumiError> {
    let git_dir = Path::new(".git");
    ensure_git_repository(git_dir)?;
    let hooks_dir = git_dir.join("hooks");
    create_directory_if_not_exists(&hooks_dir)?;
    let hook_path = hooks_dir.join("commit-msg");
    write_commit_hook_if_needed(&hook_path)?;
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

fn write_commit_hook_if_needed(hook_path: &Path) -> Result<(), SumiError> {
    if hook_path.exists() {
        match prompt_overwrite(hook_path.to_str().unwrap()) {
            Err(e) => {
                return Err(SumiError::GeneralError {
                    details: e.to_string(),
                })
            }
            Ok(true) => {}
            Ok(false) => return Ok(()),
        }
    }
    fs::write(hook_path, COMMIT_MSG_HOOK).map_err(|e| SumiError::GeneralError {
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

        let config_comments = HashMap::from([
            ("quiet", "Suppress progress messages."),
            ("display", "Shows the parsed commit message post-linting. See 'format' for options."),
            ("format", "Output format for the parsed commit message. Options: \"cli\", \"json\", \"table\", \"toml\"."),
            ("split_lines", "Process each non-empty line in the commit message as an individual commit."),
            ("gitmoji", "Rule: include one valid Gitmoji: https://gitmoji.dev/"),
            ("conventional", "Rule: follow Conventional Commits format: https://www.conventionalcommits.org/"),
            ("imperative", "Rule: use the imperative mood in the description (e.g. \"Fix bug\" instead of \"Fixed bug\")."),
            ("whitespace", "Rule: disallow leading/trailing whitespace and consecutive spaces."),
            ("description_case", "Rule: commit description must start with the specified case. Options: \"any\", \"lower\", \"upper\"."),
            ("no_period", "Rule: do not end commit header with a period."),
            ("max_header_length", "Rule: limit the header to the specified length. A value of 0 disables this rule."),
            ("max_body_length", "Rule: wrap the body at the specified length. A value of 0 disables this rule."),
            ("scopes_allowed", "Rule: only allow the specified commit scopes. Example: [\"docs\", \"cli\"]. An empty list allows any scope."),
            ("types_allowed", "Rule: only allow the specified commit types. Example: [\"feat\", \"fix\"]. An empty list allows any type."),
            ("header_pattern", "Rule: commit header must match the specified (regex) pattern. Example: '^JIRA-\\d+:'"),
        ]);

        let mut toml_with_comments = String::new();
        for line in toml.lines() {
            if let Some((key, _)) = line.split_once('=') {
                if let Some(comment) = config_comments.get(key.trim()) {
                    toml_with_comments.push_str(&format!("\n# {}\n", comment));
                }
            }
            toml_with_comments.push_str(line);
            toml_with_comments.push('\n');
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
