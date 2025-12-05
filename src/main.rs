use anyhow::{Context, Result, bail};

const INSTALLER_SCRIPT: &str = include_str!("git_expand.fish.template");

mod ast;
mod nodes;

fn main() {
    if let Err(e) = run() {
        println!("err: {e}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let mut args = std::env::args();
    args.next(); // skip binary name
    let arg = args.next().context("missing first argument")?;
    if is_real_command(&arg)? {
        bail!("'{arg}' is a real git command");
    } else if arg == "--generate-installer" {
        let executable = std::env::current_exe().context("couldn't get own executable path")?;
        let replaced = INSTALLER_SCRIPT.replace(
            "${GIT_SHORTHAND}",
            executable.to_str().context("executable path isn't UTF-8")?,
        );
        print!("{replaced}");
    } else if arg == "--generic-installer" {
        print!("{INSTALLER_SCRIPT}");
    } else {
        let ast = ast::ast();
        let mut result = String::new();
        if let Some(tail) = ast.expand(&arg, &mut result)
            && tail.is_empty()
        {
            println!("{}", result.trim());
        } else {
            std::process::exit(1);
        }
    }
    Ok(())
}

fn is_real_command(shorthand: &str) -> Result<bool> {
    let mut cmd = std::process::Command::new("git");
    cmd.args(["help", "--all"]);
    let output = String::try_from(cmd.output()?.stdout)?;
    for line in output.lines() {
        if line.starts_with(&["   ", shorthand, " "].join("")) {
            return Ok(true);
        }
    }
    Ok(false)
}
