use clap::{Parser, Subcommand};

#[derive(Debug, Clone, Parser)]
#[command(version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub subcommand: Sub,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Sub {
    /// Generate an installer script that can be sourced during shell init.
    Installer {
        /// Also expand the command field, like `gca` to `git commit --amend`,
        /// when it doesn't conflict with existing commands.
        #[arg(long, conflicts_with = "both")]
        no_space: bool,
    },
    /// Expand a shorthand expression to a subcommand.
    Expand {
        /// Do not refuse to expand expressions that are unsupposrted valid git commands.
        #[arg(short, long)]
        force: bool,
        expr: String,
    },
    /// Show the shorthand grammar.
    Grammar,
}
