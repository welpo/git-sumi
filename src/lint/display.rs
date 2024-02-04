use super::errors::SumiError;
use crate::config::ParsedCommitDisplayFormat;
use crate::parser::ParsedCommit;
use serde_json::Value;
use tabled::{
    settings::{object::Rows, Disable, Style},
    Table, Tabled,
};

pub fn display_parsed_commit(
    commit: &ParsedCommit,
    format: &ParsedCommitDisplayFormat,
) -> Result<(), SumiError> {
    match format {
        ParsedCommitDisplayFormat::Cli => {
            display_parsed_commit_as_table(commit, ParsedCommitDisplayFormat::Cli)?
        }
        ParsedCommitDisplayFormat::Json => display_parsed_commit_as_json(commit)?,
        ParsedCommitDisplayFormat::Table => {
            display_parsed_commit_as_table(commit, ParsedCommitDisplayFormat::Table)?
        }
        ParsedCommitDisplayFormat::Toml => display_parsed_commit_as_toml(commit)?,
    }
    Ok(())
}

fn display_parsed_commit_as_json(commit: &ParsedCommit) -> Result<(), SumiError> {
    let mut commit_value =
        serde_json::to_value(commit).map_err(|err| SumiError::SerializationError {
            format: "JSON".to_string(),
            detail: err.to_string(),
        })?;

    if let Value::Object(ref mut m) = commit_value {
        m.retain(|_, v| !v.is_null());
    }

    let serialized = serde_json::to_string_pretty(&commit_value).map_err(|err| {
        SumiError::SerializationError {
            format: "JSON".to_string(),
            detail: err.to_string(),
        }
    })?;
    println!("{}", serialized);

    Ok(())
}

#[derive(Tabled)]
#[tabled(rename_all = "PascalCase")]
struct CommitRow {
    key: &'static str,
    value: String,
}

fn display_parsed_commit_as_table(
    commit: &ParsedCommit,
    format: ParsedCommitDisplayFormat,
) -> Result<(), SumiError> {
    let mut rows = Vec::new();
    let fields = [
        ("Gitmoji", commit.gitmoji.as_ref().map(|g| g.join(", "))),
        ("Commit type", commit.commit_type.clone()),
        ("Scope", commit.scope.clone()),
        // "Description" is the only field that is guaranteed to be present.
        ("Description", Some(commit.description.clone())),
        ("Body", commit.body.clone()),
        ("Footers", commit.footers.as_ref().map(|f| f.join(", "))),
        (
            "Is breaking",
            Some(format!("{}", commit.is_breaking.unwrap_or(false))),
        ),
        ("Breaking description", commit.breaking_description.clone()),
        (
            "References",
            commit.references.as_ref().map(|r| r.join(", ")),
        ),
    ];

    for (key, value) in fields.iter() {
        if let Some(val) = value {
            rows.push(CommitRow {
                key,
                value: val.clone(),
            });
        }
    }

    let mut table = Table::new(rows);
    match format {
        ParsedCommitDisplayFormat::Cli => {
            // Cute table for terminal; no header.
            table.with(Style::modern());
            table.with(Disable::row(Rows::first()));
        }
        ParsedCommitDisplayFormat::Table => {
            // Markdown table with header.
            table.with(Style::markdown());
        }
        _ => {}
    }

    println!("{}", table);
    Ok(())
}

fn display_parsed_commit_as_toml(commit: &ParsedCommit) -> Result<(), SumiError> {
    let serialized = serialize_to_toml(commit)?;
    print!("{}", serialized);
    Ok(())
}

fn serialize_to_toml(commit: &ParsedCommit) -> Result<String, SumiError> {
    let serialized = toml::to_string(&commit).map_err(|err| SumiError::SerializationError {
        format: String::from("TOML"),
        detail: err.to_string(),
    })?;
    Ok(serialized)
}
