use std::fmt::Display;

fn bail() -> ! {
    std::process::exit(1);
}

fn main() {
    let args = std::env::args();
    if args.len() != 2 {
        println!("usage: git-shorthand <SHORTHAND>");
        std::process::exit(1);
    }
    let shorthand = args.skip(1).next().unwrap();
    let Some(result) = expand(&shorthand) else {
        bail();
    };
    println!("{result}");
}

fn expand(shorthand: &str) -> Option<String> {
    let (cmd, flags, target) = split_shorthand(shorthand)?;
    let cmd = Command::parse(cmd)?;
    let target = parse_target(target);
    let flags = cmd.expand_flags(flags, target);
    Some(format!("{cmd}{flags}"))
}

// TODO: Ensure that real git commands aren't being replaced.

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum Command {
    Add, //
    Blame,
    Branch, //
    Checkout,
    Clean,
    Clone,
    Commit, //
    Diff,   //
    Fetch,  //
    Init,   //
    Log,    //
    Merge,  //
    Pull,   //
    Push,   //
    Rebase,
    Reflog,
    Reset,
    Restore,
    Show,
    Stash,
    Status,
    Switch,
    Tag,      //
    Worktree, //
}

impl Command {
    fn parse(cmd: char) -> Option<Self> {
        Some(match cmd {
            'a' => Self::Add,
            'b' => Self::Branch,
            'c' => Self::Commit,
            'd' => Self::Diff,
            'e' => todo!(),
            'f' => Self::Fetch,
            'g' => todo!(),
            'h' => Self::Show,
            'i' => Self::Init,
            'j' => todo!(),
            'l' => Self::Log,
            'm' => Self::Merge,
            'n' => todo!(),
            'o' => todo!(),
            'p' => Self::Push,
            'q' => Self::Pull,
            'r' => Self::Reset,
            's' => Self::Status, // this could also be 'show' or 'switch'
            't' => Self::Tag,
            'u' => todo!(), // this could be reset (undo)
            'v' => todo!(), // this could be show (view)
            'w' => Self::Worktree,
            'x' => todo!(),
            'y' => todo!(),
            'z' => todo!(),
            _ => return None,
        })
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
                Command::Clone => todo!(),
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
                Command::Init => todo!(),
                Command::Log => todo!(),
                Command::Merge => todo!(),
                Command::Pull => todo!(),
                Command::Push => todo!(),
                Command::Rebase => todo!(),
                Command::Reflog => todo!(),
                Command::Reset => match flag {
                    'q' => "--quiet",
                    's' => "--soft",
                    'h' => "--hard",
                    'm' => "--merge",
                    'k' => "--keep",
                    'r' => "--recurse-submodules",
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

pub use flags::FLAGS;
mod flags {
    use super::Command::{self, *};
    pub const FLAGS: &[(Command, &[(char, &str)])] = &[];
}

fn split_shorthand(shorthand: &str) -> Option<(char, &str, &str)> {
    let cmd: char = shorthand.chars().next()?;
    let tail = shorthand.get(1..).unwrap_or("");
    let split_idx = tail.find('-');
    let (flags, target) = match split_idx {
        Some(i) => tail.split_at(i),
        None => (tail, ""),
    };
    Some((cmd, flags, target))
}

fn parse_target(target: &str) -> Option<String> {
    // Empty target is no target
    if target.is_empty() {
        return None;
    }

    // Parent of HEAD, multiple tildes
    if target.chars().all(|c| c == '-') {
        let count = target.chars().count();
        return Some(format!("HEAD~{count}"));
    }

    // Parent of HEAD, numbered
    let first_is_minus = target.chars().next().unwrap() == '-';
    let rest_is_digits = target.chars().skip(1).all(|c| c.is_ascii_digit());
    if first_is_minus && rest_is_digits {
        let number: u32 = str::parse(&target[1..]).ok()?;
        return Some(format!("HEAD~{number}"));
    } else {
        return None;
    }
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
