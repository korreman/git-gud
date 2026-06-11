use clap::{Parser, Subcommand};

#[derive(Debug, Clone, Parser)]
#[command(version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Command,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Command {
    /// Generate an installer script that can be sourced during shell init.
    Installer {
        /// Set the command to expand to when entering only `g` and submitting the command.
        #[arg(long, default_value = "status")]
        default_command: String,
    },
    /// Expand a shorthand expression to a subcommand.
    Expand { expr: String, cursor_char: char },
    /// Start an interactive completion prompt, showing you options as you type.
    Complete,
    /// Shaw.
    #[command(hide = true)]
    Shaw,
}

pub const GIT_GUD: &'static str = "  |\\ |\\
  | \\| \\   PHRASE
  \\ O O/    
 _ \\  / _ 
  \\/  \\/
  /____\\
    | \\
   /  /
";

pub const GIT_GUD_POSHANKA: &'static str = "  |\\ |\\
  | \\| \\   PHRASE
  \\ O O/    
 O \\  / O 
  \\/  \\/
  /____\\
    | \\
   /  /
";

pub const HORNET_PHRASES: &[&'static str] = &[
    "Garama",
    "Fuedastama",
    "Vennefrein",
    "Yirenare",
    "Kadestre",
    "Yennada",
    "La, fenistra",
    "Hasvien",
    "Gueneera",
    "Nejinafore",
    "Nesvire",
    "Temirayen",
    "Mihrfehne",
    "Poshanka!",
];
