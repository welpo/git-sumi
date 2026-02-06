extern crate git_conventional;
extern crate prettytable;
extern crate serde;
extern crate toml;

mod args;
mod config;
mod errors;
pub mod lint;
mod parser;

use crate::errors::SumiError;
use args::Opt;
use clap::{CommandFactory, Parser};
use config::{
    assemble_config, count_active_rules, generate_commit_msg_hook_content, init_config, Config,
};
use env_logger::Builder;
use lint::{run_lint, run_lint_on_each_line};
use log::{error, info, LevelFilter};
use parser::ParsedCommit;
use std::io::{self, Read, Write};

pub fn run() -> Result<(), SumiError> {
    let args = Opt::parse();

    if let Some(init_option) = args.init {
        init_config(init_option)?;
        return Ok(());
    }

    if let Some(shell_type) = args.generate_shell_completion {
        generate_shell_completion(shell_type);
        return Ok(());
    }

    let config = assemble_config(&args)?;
    init_logger_from_config(&config);

    if args.prepare_commit_message {
        generate_commit_msg_hook_content(&config)?;
        return Ok(());
    }

    // It's Lintin' Time.
    if !args.commit && !config.display && count_active_rules(&config) == 0 {
        return Err(SumiError::NoRulesEnabled);
    }

    let commit_message = get_commit_from_arg_or_stdin(args.commit_message, args.commit_file)?;

    let lint_result = if config.split_lines {
        run_lint_on_each_line(&commit_message, &config)
    } else {
        run_lint(&commit_message, &config).map(|pc| vec![pc])
    };

    if args.commit {
        handle_commit_based_on_lint(lint_result, &commit_message, args.force)?;
    } else {
        lint_result?;
    }

    Ok(())
}

fn init_logger_from_config(config: &Config) {
    Builder::new()
        .format(|buf, record| {
            if record.level() == log::Level::Error {
                writeln!(buf, "❌ Error: {}", record.args())
            } else {
                writeln!(buf, "{}", record.args())
            }
        })
        .filter(
            None,
            if config.quiet {
                LevelFilter::Error
            } else {
                LevelFilter::Info
            },
        )
        .target(env_logger::Target::Stdout)
        .init();
}

fn get_commit_from_arg_or_stdin(
    commit: Option<String>,
    commit_file: Option<String>,
) -> Result<String, SumiError> {
    let msg = match (commit, commit_file) {
        (Some(message), _) => message,
        (None, Some(path)) => get_commit_from_file(&path)?,
        (None, None) => get_commit_from_stdin()?,
    };

    remove_verbose_output(&msg)
}

fn get_commit_from_file(path: &str) -> Result<String, SumiError> {
    std::fs::read_to_string(path)
        .map(|content| content.trim().to_string())
        .map_err(|e| SumiError::GeneralError {
            details: format!("Could not read commit message from '{path}': {e}"),
        })
}

fn get_commit_from_stdin() -> Result<String, SumiError> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    Ok(buffer.trim().to_string())
}

fn remove_verbose_output(commit_message: &str) -> Result<String, SumiError> {
    let commentchar = get_git_commentchar()?;
    let cutline = format!(
        "{} ------------------------ >8 ------------------------",
        commentchar
    );

    match commit_message.lines().position(|line| line == cutline) {
        Some(i) => {
            let truncated: Vec<&str> = commit_message.lines().take(i).collect();
            Ok(truncated.join("\n"))
        }
        None => Ok(commit_message.to_string()),
    }
}

fn get_git_commentchar() -> Result<String, SumiError> {
    let output = std::process::Command::new("git")
        .arg("config")
        .arg("--get")
        .arg("core.commentchar")
        .output()?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        if stderr.is_empty() {
            // commentchar isn't set, so fallback to #
            Ok("#".to_string())
        } else {
            Err(SumiError::GeneralError {
                details: format!("Failed to get git comment character: {}", stderr.trim()),
            })
        }
    }
}

fn handle_commit_based_on_lint(
    lint_result: Result<Vec<ParsedCommit>, SumiError>,
    commit_message: &str,
    force: bool,
) -> Result<(), SumiError> {
    match lint_result {
        Ok(_) => commit_with_message(commit_message),
        Err(lint_errors) if force => {
            error!(
                "🚨 Forced commit with lint errors: {lint_errors}. Force flag is set, committing despite errors."
            );
            commit_with_message(commit_message)
        }
        Err(lint_errors) => {
            info!("💡 Use the --force flag to commit despite errors.");
            Err(lint_errors)
        }
    }
}

fn commit_with_message(commit_message: &str) -> Result<(), SumiError> {
    info!("🚀 Running git commit…");
    let commit_result = execute_git_commit(commit_message)?;

    if commit_result.status.success() {
        info!("🎉 Commit successful!");
        Ok(())
    } else {
        Err(construct_commit_error(commit_result))
    }
}

fn execute_git_commit(commit_message: &str) -> Result<std::process::Output, std::io::Error> {
    use std::env;
    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;

    // Create a temporary file to store the full commit message.
    // Using `git commit -m` with multiline messages can lead to malformed commits
    // or truncated bodies, so we use `-F` to pass the message via file instead.
    let mut path: PathBuf = env::temp_dir();
    path.push("sumi_commit_msg.txt");

    let mut file = File::create(&path)?;
    file.write_all(commit_message.as_bytes())?;

    // Use `git commit -F` to ensure the full commit message (including body)
    // is preserved exactly as provided.
    std::process::Command::new("git")
        .args(["commit", "-F", path.to_str().unwrap()])
        .output()
}

fn construct_commit_error(commit_output: std::process::Output) -> SumiError {
    let git_output = [&commit_output.stderr[..], &commit_output.stdout[..]].concat();
    let git_error_message = extract_error_message(&git_output);
    SumiError::ErrorWhileCommitting(git_error_message)
}

fn extract_error_message(git_output: &[u8]) -> String {
    let git_error_cow = String::from_utf8_lossy(git_output);
    let git_error = git_error_cow.trim();

    if git_error.is_empty() {
        "Commit failed. No additional error information available.".to_string()
    } else {
        format!("git output:\n{git_error}")
    }
}

fn generate_shell_completion(shell: clap_complete::Shell) {
    let cmd = &mut Opt::command();
    clap_complete::generate(
        shell,
        cmd,
        cmd.get_name().to_string(),
        &mut std::io::stdout(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;
    use std::process::Command;

    #[test]
    #[serial_test::serial]
    fn multiline_commit_body_is_preserved() {
        let original_dir = std::env::current_dir().unwrap();

        // temp repo
        let tmp = env::temp_dir().join("git_sumi_test_repo");
        let _ = fs::remove_dir_all(&tmp);
        fs::create_dir_all(&tmp).unwrap();

        // init git repo
        Command::new("git")
            .arg("init")
            .current_dir(&tmp)
            .output()
            .unwrap();

        // required git identity (CI-safe)
        Command::new("git")
            .args(["config", "user.email", "test@example.com"])
            .current_dir(&tmp)
            .output()
            .unwrap();

        Command::new("git")
            .args(["config", "user.name", "Test"])
            .current_dir(&tmp)
            .output()
            .unwrap();

        // create file
        fs::write(tmp.join("file.txt"), "hello").unwrap();

        Command::new("git")
            .args(["add", "."])
            .current_dir(&tmp)
            .output()
            .unwrap();

        let msg = "feat: test\n\nbody line 1\nbody line 2\nfooter";

        // switch into temp repo for commit
        std::env::set_current_dir(&tmp).unwrap();

        // ensure staged (CI-safe)
        Command::new("git").args(["add", "."]).output().unwrap();

        let commit = super::execute_git_commit(msg).unwrap();
        assert!(commit.status.success(), "commit failed");

        // read commit back
        let log = Command::new("git")
            .args(["log", "-1", "--pretty=%B"])
            .current_dir(&tmp)
            .output()
            .unwrap();

        let result = String::from_utf8_lossy(&log.stdout);

        assert!(
            result.contains("body line 1") && result.contains("body line 2"),
            "multiline body was NOT preserved:\n{}",
            result
        );

        // restore cwd so other tests don't break
        std::env::set_current_dir(original_dir).unwrap();
    }
}
