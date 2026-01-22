use anyhow::{Context, Result};
use clap::Parser;
use log::debug;

const INSTALLER_SCRIPT: &str = include_str!("git_expand.fish.template");

mod cli;
mod grammar;
mod helpers;
mod tree;

fn main() {
    env_logger::init();

    if let Err(e) = run() {
        println!("err: {e}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = cli::Cli::parse();
    match cli.subcommand {
        cli::Sub::Installer { default_command } => {
            let executable = std::env::current_exe().context("couldn't get own executable path")?;
            let with_executable = INSTALLER_SCRIPT.replace(
                "${GIT_GUD}",
                executable.to_str().context("executable path isn't UTF-8")?,
            );
            let with_default = with_executable.replace("${DEFAULT_COMMAND}", &default_command);
            print!("{with_default}");
        }
        cli::Sub::Expand { expr, cursor_char } => {
            let ast = grammar::ast();
            debug!("{ast:#?}");
            let mut result = String::from("git ");
            let eol = cursor_char != ' ';
            if expr.starts_with('a')
                && let Some(idx) = expr.find(['c', 'e'])
            {
                let (first, second) = expr.split_at(idx);
                let tail = ast.expand(first, true, &mut result);
                if tail != Some("") {
                    std::process::exit(1);
                }
                result.push_str(" && git ");

                let tail = ast.expand(second, eol, &mut result);
                if tail != Some("") {
                    std::process::exit(1);
                }
            } else {
                let tail = ast.expand(&expr, eol, &mut result);
                if tail != Some("") {
                    std::process::exit(1);
                }
            }
            println!("{}", result.trim());
        }
        cli::Sub::Grammar => {
            todo!("showing the grammar is not supported yet");
        }
    }
    Ok(())
}
