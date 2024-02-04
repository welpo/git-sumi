use super::errors::SumiError;
use crate::config::ParsedCommitDisplayFormat;
use crate::parser::ParsedCommit;
use prettytable::{format, Cell, Row, Table};
use serde_json::Value;

pub fn display_parsed_commit(
    commit: &ParsedCommit,
    format: &ParsedCommitDisplayFormat,
) -> Result<(), SumiError> {
    match format {
        ParsedCommitDisplayFormat::Json => display_parsed_commit_as_json(commit)?,
        ParsedCommitDisplayFormat::Table => display_parsed_commit_as_table(commit)?,
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

fn display_parsed_commit_as_table(commit: &ParsedCommit) -> Result<(), SumiError> {
    let mut table = Table::new();
    add_row_for_vector(&mut table, "Gitmoji", &commit.gitmoji);
    add_row_for_string(&mut table, "Commit type", &commit.commit_type);
    add_row_for_string(&mut table, "Scope", &commit.scope);
    add_row_for_string(&mut table, "Description", &Some(commit.description.clone()));
    add_row_for_string(&mut table, "Body", &commit.body);
    add_row_for_vector(&mut table, "Footers", &commit.footers);
    add_row_for_bool(&mut table, "Is breaking", &commit.is_breaking);
    add_row_for_string(
        &mut table,
        "Breaking description",
        &commit.breaking_description,
    );
    add_row_for_vector(&mut table, "References", &commit.references);
    table.set_format(*format::consts::FORMAT_BOX_CHARS);
    table.printstd();
    Ok(())
}

fn add_row_for_string(table: &mut Table, label: &str, value: &Option<String>) {
    if let Some(v) = value {
        table.add_row(Row::new(vec![Cell::new(label), Cell::new(v)]));
    }
}

fn add_row_for_bool(table: &mut Table, label: &str, value: &Option<bool>) {
    if let Some(v) = value {
        let display_value = if *v { "true" } else { "false" };
        table.add_row(Row::new(vec![Cell::new(label), Cell::new(display_value)]));
    }
}

fn add_row_for_vector(table: &mut Table, label: &str, vec: &Option<Vec<String>>) {
    if let Some(v) = vec {
        if !v.is_empty() {
            let joined = v.join(", ");
            table.add_row(Row::new(vec![Cell::new(label), Cell::new(&joined)]));
        }
    }
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
