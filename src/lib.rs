extern crate git_conventional;
extern crate lazy_static;
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

    let commit_message = get_commit_from_arg_or_stdin(args.commit_message)?;

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

fn get_commit_from_arg_or_stdin(commit: Option<String>) -> Result<String, SumiError> {
    if let Some(commit) = commit {
        Ok(commit)
    } else {
        get_commit_from_stdin()
    }
}

fn get_commit_from_stdin() -> Result<String, SumiError> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    Ok(buffer.trim().to_string())
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
                "🚨 Forced commit with lint errors: {}. Force flag is set, committing despite errors.",
                lint_errors
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
    std::process::Command::new("git")
        .args(["commit", "-m", commit_message])
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
        format!("git output:\n{}", git_error)
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
