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
        /// Set the command to expand to when entering only `g` and submitting the command.
        #[arg(long, default_value = "status")]
        default_command: String,
    },
    /// Expand a shorthand expression to a subcommand.
    Expand { expr: String, cursor_char: char },
    /// Show the shorthand grammar.
    Grammar,
}
