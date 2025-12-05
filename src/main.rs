use std::fmt::Display;

const INSTALLER_SCRIPT: &str = include_str!("git_expand.fish.template");

fn main() {
    let args = std::env::args();
    if args.len() != 2 {
        println!("usage: git-shorthand <SHORTHAND>");
        std::process::exit(1);
    }
    let arg = args.skip(1).next().unwrap();
    if arg == "--generate-installer" {
        let executable = std::env::current_exe().expect("couldn't get own executable");
        let replaced = INSTALLER_SCRIPT.replace("${GIT_SHORTHAND}", executable.to_str().unwrap());
        print!("{replaced}");
    } else if arg == "--generic-installer" {
        print!("{INSTALLER_SCRIPT}");
    } else {
        let Some(result) = expand(&arg) else {
            bail();
        };
        println!("{result}");
    }
}

fn bail() -> ! {
    std::process::exit(1);
}

fn expand(shorthand: &str) -> Option<String> {
    let (cmd, rest) = Command::parse(shorthand)?;
    let (flags, target) = split_flags_from_target(rest)?;
    let target = parse_target(target)?;
    let flags = cmd.expand_flags(flags, target);
    Some(format!("{cmd}{flags}"))
}

// TODO: Ensure that real git commands aren't being replaced.

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
        //("h", todo!()),
        ("i", Self::Init),
        //("j", todo!()),
        ("k", Self::Clone),
        ("l", Self::Log),
        ("m", Self::Merge),
        //("n", todo!()),
        //("o", todo!()),
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
        //("y", todo!()),
        ("z", Self::Stash), // ztash, similar to marks in modal editors
    ];

    fn parse(shortcmd: &str) -> Option<(Command, &str)> {
        for (prefix, cmd) in Self::COMMAND_PREFIX {
            if let Some(rest) = shortcmd.strip_prefix(prefix) {
                return Some((*cmd, rest));
            }
        }
        None
    }

    fn expand_flags(&self, flags: &str, target: Option<String>) -> String {
        let mut body = Vec::new();
        let mut end_flags = Vec::new();
        for flag in flags.chars() {
            let expanded_flag = match self {
                Command::Add => match flag {
                    'a' => {
                        end_flags.push(":/");
                        "--all"
                    }
                    'e' => "--edit",
                    'f' => "--force",
                    'i' => "--interactive",
                    'n' => "--dry-run",
                    'N' => "--intent-to-add",
                    'p' => "--patch",
                    'u' => "--update",
                    'v' => "--verbose",
                    _ => bail(),
                },
                Command::Blame => todo!(),
                Command::Branch => todo!(),
                Command::Checkout => todo!(),
                Command::Clean => todo!(),
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
                        end_flags.push("--reference=%");
                        continue;
                    }
                    _ => bail(),
                },
                Command::Commit => match flag {
                    'a' => "--amend",
                    'i' => "--include",
                    'm' => {
                        end_flags.push("--message=\"%\"");
                        continue;
                    }
                    'n' => "--no-verify",
                    'o' => "--only",
                    'q' => "--quiet",
                    's' => "--signoff",
                    't' => "--template=%",
                    'v' => "--verbose",
                    _ => bail(),
                },
                Command::Diff => todo!(),
                Command::Fetch => todo!(),
                Command::Init => match flag {
                    'b' => "--bare",
                    'h' => "--object-format=sha256",
                    'i' => {
                        end_flags.push("--initial-branch=%");
                        continue;
                    }
                    'q' => "--quiet",
                    's' => {
                        end_flags.push("--separate-git-dir=%");
                        continue;
                    }
                    _ => bail(),
                },
                Command::Log => todo!(),
                Command::Merge => todo!(),
                Command::Pull => todo!(),
                Command::Push => todo!(),
                Command::Rebase => todo!(),
                Command::Reflog => todo!(),
                Command::Reset => match flag {
                    'h' => "--hard",
                    'k' => "--keep",
                    'm' => "--merge",
                    'q' => "--quiet",
                    'r' => "--recurse-submodules",
                    's' => "--soft",
                    _ => bail(),
                },
                Command::Restore => todo!(),
                Command::Show => todo!(),
                Command::Stash => todo!(),
                Command::Status => todo!(),
                Command::Switch => todo!(),
                Command::Tag => todo!(),
                Command::Worktree => todo!(),
            };
            body.push(expanded_flag);
        }
        let mut result = String::new();
        for flag in body.iter().chain(end_flags.iter()) {
            result.push(' ');
            result.push_str(flag);
        }
        if let Some(target) = target
            && matches!(self, Command::Reset)
        {
            result.push(' ');
            result.push_str(&target);
        }
        result
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

fn split_flags_from_target(input: &str) -> Option<(&str, &str)> {
    let split_idx = input.find(['-', '/']);
    let (flags, target) = match split_idx {
        Some(i) => input.split_at(i),
        None => (input, ""),
    };
    Some((flags, target))
}

fn parse_target(target: &str) -> Option<Option<String>> {
    // Empty target is no target
    if target.is_empty() {
        return Some(None);
    }

    // Parent of HEAD, multiple tildes
    if target.chars().all(|c| c == '-') {
        let mut result = "HEAD".to_string();
        for _ in target.chars() {
            result.push('~');
        }
        return Some(Some(result));
    }

    // Parent of HEAD, numbered
    let first_is_minus = target.chars().next().unwrap() == '-';
    let rest_is_digits = target.chars().skip(1).all(|c| c.is_ascii_digit());
    if first_is_minus && rest_is_digits {
        let number: u32 = str::parse(&target[1..]).ok()?;
        return Some(Some(format!("HEAD~{number}")));
    }

    // Reflog entry, numbered
    let first_is_slash = target.chars().next().unwrap() == '/';
    if first_is_slash && rest_is_digits {
        let number: u32 = str::parse(&target[1..]).ok()?;
        return Some(Some(format!("HEAD@{{{number}}}")));
    }

    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_shorthand(input: &str, sub: char, flags: &str, target: &str) {
        assert_eq!(split_shorthand(input), Some((sub, flags, target)));
    }

    #[test]
    fn test_split_shorthand() {
        test_shorthand("c", 'c', "", "");
        test_shorthand("cabc", 'c', "abc", "");
        test_shorthand("ccc", 'c', "cc", "");
        test_shorthand("ccch", 'c', "cc", "h");
        test_shorthand("ccc-", 'c', "cc", "-");
        test_shorthand("ccc-asdlkjf", 'c', "cc", "-asdlkjf");
        test_shorthand("ccc---", 'c', "cc", "---");
        test_shorthand("h", 'h', "", "");
        test_shorthand("-", '-', "", "");
        test_shorthand("--", '-', "", "-");
        test_shorthand("b-23", 'b', "", "-23");
        test_shorthand("bfeu-23", 'b', "feu", "-23");

        assert_eq!(split_shorthand(""), None);
    }
}
