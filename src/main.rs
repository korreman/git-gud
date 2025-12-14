use anyhow::{Context, Result, bail};
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
        cli::Sub::Installer => {
            let executable = std::env::current_exe().context("couldn't get own executable path")?;
            let with_executable = INSTALLER_SCRIPT.replace(
                "${GIT_GUD}",
                executable.to_str().context("executable path isn't UTF-8")?,
            );
            print!("{with_executable}");
        }
        cli::Sub::Expand { expr } => {
            let ast = grammar::ast();
            debug!("{ast:#?}");
            let mut result = String::new();
            if let Some(tail) = ast.expand(&expr, &mut result)
                && tail.is_empty()
            {
                println!("{}", result.trim());
            } else {
                std::process::exit(1);
            }
        }
        cli::Sub::Grammar => {
            todo!("showing the grammar is not supported yet");
        }
    }
    Ok(())
}
