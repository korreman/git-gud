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
    Installer,
    /// Expand a shorthand expression to a subcommand.
    Expand { expr: String },
    /// Show the shorthand grammar.
    Grammar,
}
