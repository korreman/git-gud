use anyhow::{Context, Result, anyhow, bail};
use std::fmt::Display;

const INSTALLER_SCRIPT: &str = include_str!("git_expand.fish.template");

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
        let last_char = args.next().context("missing second argument")?;
        let result = expand(&arg, last_char != " ")?;
        println!("{result}");
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

fn expand(shorthand: &str, terminate: bool) -> Result<String> {
    let (cmd, rest) = Command::parse(shorthand)?;
    let (flags, target) = split_flags_from_target(rest);
    let target = parse_target(target)?;
    if cmd == Command::Add
        && let Some(idx) = flags.find('c')
    {
        let (add_flags, tail) = flags.split_at(idx);
        let flags = cmd.expand_flags(add_flags, target, true)?;
        let cmd2 = expand(tail, terminate)?;
        Ok(format!("{cmd}{flags}; git {cmd2}"))
    } else {
        let flags = cmd.expand_flags(flags, target, terminate)?;
        Ok(format!("{cmd}{flags}"))
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum Command {
    Add,
    Blame,
    Branch,
    Checkout,
    Clean,
    Clone,
    Commit,
    Diff,
    Fetch,
    Init,
    Log,
    Merge,
    Pull,
    Push,
    Rebase,
    Reflog,
    Reset,
    Restore,
    Show,
    Stash,
    Status,
    Switch,
    Tag,
    Worktree,
}

impl Command {
    const COMMAND_PREFIX: &[(&str, Command)] = &[
        ("a", Self::Add),
        ("bl", Self::Blame),
        ("b", Self::Branch),
        ("c", Self::Commit),
        ("d", Self::Diff),
        ("e", Self::Rebase), // r__e__base
        ("f", Self::Fetch),
        ("g", Self::Checkout), // goto
        //("h", _),
        ("i", Self::Init),
        //("j", _),
        ("k", Self::Clone),
        ("l", Self::Log),
        ("m", Self::Merge),
        //("n", _),
        //("o", _),
        ("p", Self::Push),
        ("q", Self::Pull), // visually reversed 'p', on the opposite end of keyboard
        ("rl", Self::Reflog), // __r__ef__l__og
        ("r", Self::Reset),
        ("st", Self::Status),
        ("s", Self::Switch),
        ("t", Self::Tag),
        ("u", Self::Restore), // undo
        ("v", Self::Show),    // view
        ("w", Self::Worktree),
        ("x", Self::Clean),
        //("y", _),
        ("z", Self::Stash), // ztash, similar to marks in modal editors
    ];

    fn parse(shortcmd: &str) -> Result<(Command, &str)> {
        for (prefix, cmd) in Self::COMMAND_PREFIX {
            if let Some(rest) = shortcmd.strip_prefix(prefix) {
                return Ok((*cmd, rest));
            }
        }
        Err(anyhow!("doesn't match any command prefix"))
    }

    fn expand_flags(
        &self,
        flags: &str,
        mut target: Option<String>,
        terminate: bool,
    ) -> Result<String> {
        let mut body = Vec::new();
        let mut end = Vec::new();
        for flag in flags.chars() {
            let flag_err = format!("unrecognized flag '{flag}' for '{self}' command");
            let expanded_flag = match self {
                Command::Add => match flag {
                    'a' => "--all",
                    'e' => "--edit",
                    'f' => "--force",
                    'i' => "--interactive",
                    'n' => "--dry-run",
                    'N' => "--intent-to-add",
                    'p' => "--patch",
                    'u' => "--update",
                    'v' => "--verbose",
                    '.' => {
                        if target.replace(String::from(".")).is_some() {
                            bail!("both a target and '.' specified in add command")
                        }
                        continue;
                    }
                    _ => bail!(flag_err),
                },
                Command::Blame => match flag {
                    'l' => {
                        end.push("-L %");
                        continue;
                    }
                    _ => bail!(flag_err),
                },
                Command::Branch => match flag {
                    'd' => "--delete",
                    'D' => "--delete --force",
                    'f' => "--force",
                    'm' => "--move",
                    'M' => "--move --force",
                    'c' => "--copy",
                    'C' => "--copy --force",
                    'i' => "--ignore-case",
                    'r' => "--remotes",
                    'a' => "--all",
                    'l' => "--list",
                    'v' => "--verbose",
                    'q' => "--quiet",
                    't' => "--track",
                    'e' => "--edit-description",
                    _ => bail!(flag_err),
                },
                Command::Checkout => match flag {
                    _ => bail!(flag_err),
                },
                Command::Clean => match flag {
                    _ => bail!(flag_err),
                },
                Command::Clone => match flag {
                    'b' => "--bare",
                    'd' => "--dissociate",
                    'h' => "--shared",
                    'l' => "--local",
                    'm' => "--mirror",
                    'q' => "--quiet",
                    's' => "--sparse",
                    'v' => "--verbose",
                    'r' => {
                        end.push("--reference=%");
                        continue;
                    }
                    _ => bail!(flag_err),
                },
                Command::Commit => match flag {
                    'a' => "--amend",
                    'i' => "--include",
                    'm' => {
                        end.push("--message=\"%\"");
                        continue;
                    }
                    'n' => "--no-verify",
                    'o' => "--only",
                    'q' => "--quiet",
                    's' => "--signoff",
                    't' => "--template=%",
                    'v' => "--verbose",
                    _ => bail!(flag_err),
                },
                Command::Diff => match flag {
                    _ => bail!(flag_err),
                },
                Command::Fetch => match flag {
                    '4' => "--ipv4",
                    '6' => "--ipv6",
                    'a' => "--all",
                    'd' => "--dry-run",
                    'f' => "--force",
                    'p' => "--prune",
                    'P' => "--prune-tags",
                    'q' => "--quiet",
                    't' => "--tags",
                    'u' => {
                        /* TODO: Get name of branch */
                        continue;
                    }
                    'v' => "--verbose",
                    _ => bail!(flag_err),
                },
                Command::Init => match flag {
                    'b' => "--bare",
                    'h' => "--object-format=sha256",
                    'i' => {
                        end.push("--initial-branch=%");
                        continue;
                    }
                    'q' => "--quiet",
                    's' => {
                        end.push("--separate-git-dir=%");
                        continue;
                    }
                    _ => bail!(flag_err),
                },
                Command::Log => match flag {
                    'o' => "--oneline",
                    'f' => "--follow",
                    'd' => "--decorate",
                    's' => "--sparse",
                    'm' => "--merges",
                    'a' => "--all",
                    '1' => "--first-parent",
                    'b' => "--bisect",
                    'g' => "--graph",
                    'D' => "--date-order",
                    'r' => "--reverse",
                    'l' => "--no-abrev-commit",
                    'p' => "--parents",
                    'c' => "--children",
                    _ => bail!(flag_err),
                },
                Command::Merge => match flag {
                    _ => bail!(flag_err),
                },
                Command::Pull => match flag {
                    '4' => "--ipv4",
                    '6' => "--ipv6",
                    'a' => "--all",
                    'd' => "--dry-run",
                    'e' => "--edit",
                    'f' => "--force",
                    'p' => "--prune",
                    'q' => "--quiet",
                    'r' => "--recurse-submodules",
                    't' => "--tags",
                    'u' => {
                        /* TODO: Get name of the current branch */
                        continue;
                    }
                    'v' => "--verbose",
                    _ => bail!(flag_err),
                },
                Command::Push => match flag {
                    '4' => "--ipv4",
                    '6' => "--ipv6",
                    'a' => "--all",
                    'A' => "--atomic",
                    'd' => "--delete",
                    'f' => "--force",
                    'm' => "--mirror",
                    'p' => "--prune",
                    'q' => "--quiet",
                    'r' => "--recurse-submodules",
                    't' => "--tags",
                    'u' => {
                        /* TODO: Get name of the current branch */
                        continue;
                    }
                    'v' => "--verbose",
                    _ => bail!(flag_err),
                },
                Command::Rebase => match flag {
                    'a' => "--abort",
                    'c' => "--continue",
                    'e' => "--edit-todo",
                    'h' => "--show-current-patch",
                    'i' => "--interactive",
                    'k' => "--keep-base",
                    'o' => {
                        end.push("--onto=%");
                        continue;
                    }
                    'q' => "--quiet",
                    'Q' => "--quit",
                    'r' => "--root",
                    's' => "--skip",
                    't' => "--stat",
                    'u' => "--uprate-refs",
                    'v' => "--verbose",
                    'x' => {
                        end.push("--exec='%'");
                        continue;
                    }
                    _ => bail!(flag_err),
                },
                Command::Reflog => match flag {
                    _ => bail!(flag_err),
                },
                Command::Reset => match flag {
                    'h' => "--hard",
                    'k' => "--keep",
                    'm' => "--merge",
                    'q' => "--quiet",
                    'r' => "--recurse-submodules",
                    's' => "--soft",
                    _ => bail!(flag_err),
                },
                Command::Restore => match flag {
                    _ => bail!(flag_err),
                },
                Command::Show => match flag {
                    _ => bail!(flag_err),
                },
                Command::Stash => match flag {
                    _ => bail!(flag_err),
                },
                Command::Status => match flag {
                    'l' => "--long",
                    's' => "--short",
                    _ => bail!(flag_err),
                },
                Command::Switch => match flag {
                    _ => bail!(flag_err),
                },
                Command::Tag => match flag {
                    _ => bail!(flag_err),
                },
                Command::Worktree => match flag {
                    _ => bail!(flag_err),
                },
            };
            body.push(expanded_flag);
        }

        let mut result = String::new();
        if terminate && matches!(self, Command::Add) && target.is_none() {
            if !body.contains(&"--all") {
                body.push("--all");
            }
            target = Some(":/".to_string());
        }
        if let Some(target) = &target
            && (matches!(self, Command::Reset) || matches!(self, Command::Add))
        {
            body.push(target);
        }
        for flag in body.iter().chain(end.iter()) {
            result.push(' ');
            result.push_str(flag);
        }
        Ok(result)
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Command::Add => "add",
            Command::Blame => "blame",
            Command::Branch => "branch",
            Command::Checkout => "checkout",
            Command::Clean => "clean",
            Command::Clone => "clone",
            Command::Commit => "commit",
            Command::Diff => "diff",
            Command::Fetch => "fetch",
            Command::Init => "init",
            Command::Log => "log",
            Command::Merge => "merge",
            Command::Pull => "pull",
            Command::Push => "push",
            Command::Rebase => "rebase",
            Command::Reflog => "reflog",
            Command::Reset => "reset",
            Command::Restore => "restore",
            Command::Show => "show",
            Command::Stash => "stash",
            Command::Status => "status",
            Command::Switch => "switch",
            Command::Tag => "tag",
            Command::Worktree => "worktree",
        })
    }
}

fn split_flags_from_target(input: &str) -> (&str, &str) {
    let split_idx = input.find(['-', '/']);
    let (flags, target) = match split_idx {
        Some(i) => input.split_at(i),
        None => (input, ""),
    };
    (flags, target)
}

fn parse_target(target: &str) -> Result<Option<String>> {
    // Empty target is no target
    if target.is_empty() {
        return Ok(None);
    }

    // Parent of HEAD, multiple tildes
    if target.chars().all(|c| c == '-') {
        let mut result = "HEAD".to_string();
        for _ in target.chars() {
            result.push('~');
        }
        return Ok(Some(result));
    }

    let first = target
        .chars()
        .next()
        .expect("couldn't get first char despite target not empty");

    // Parent of HEAD, numbered
    let rest_is_digits = target.chars().skip(1).all(|c| c.is_ascii_digit());
    if first == '-' && rest_is_digits {
        let number: u32 = str::parse(&target[1..])?;
        return Ok(Some(format!("HEAD~{number}")));
    }

    // Reflog entry, numbered
    if first == '/' && rest_is_digits {
        let number: u32 = str::parse(&target[1..])?;
        return Ok(Some(format!("HEAD@{{{number}}}")));
    }

    Err(anyhow!("target '{target}' is invalid"))
}
