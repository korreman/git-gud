use std::process::Command;

/// Run `git $args`, and return the trimmed stdout.
fn git_query_command(args: &[&str]) -> Option<String> {
    let output = Command::new("git").args(args).output().ok()?;
    if !output.status.success() {
        return None;
    }
    let result = str::from_utf8(&output.stdout).ok()?.trim();
    Some(result.to_owned())
}

/// Get the currently checked out branch.
pub fn current_branch() -> Option<String> {
    git_query_command(&["branch", "--show-current"])
}

/// Get the tracked branch of the current branch.
pub fn current_upstream() -> Option<String> {
    upstream(current_branch()?)
}

/// Get the main branch (HEAD) of the upstream remote of the current branch.
pub fn current_remote_head() -> Option<String> {
    remote_head(&main_remote()?)
}

/// Get the main branch (HEAD) of the main remote.
pub fn main_remote_head() -> Option<String> {
    remote_head(&main_remote()?)
}

/// Get the (first) branch tracking the HEAD of the main remote.
/// This can commonly be interpreted as your "main" branch of the local repository.
pub fn main_branch() -> Option<String> {
    let main_remote_head = main_remote_head()?;
    let branches = git_query_command(&[
        "for-each-ref",
        "--format=%(refname:short) %(upstream:short)",
    ])?;
    //println!("mrh: {main_remote_head}");
    for line in branches.lines() {
        //println!("{line}");
        if let Some((branch, upstream)) = line.split_once(' ') {
            if upstream == main_remote_head {
                return Some(branch.to_owned());
            }
        }
    }
    None
}

/// Get the upstream remote of the current branch.
pub fn current_remote() -> Option<String> {
    tracked_remote(&current_branch()?)
}

/// Get the "main" remote of the repository, first of the following priorities:
/// 1. The `checkout.defaultRemote` variable if available.
/// 2. `origin` if such a named remote exists.
/// 3. The first remote returned by `git remote`.
/// 4. `origin` if no remotes exist.
pub fn main_remote() -> Option<String> {
    if let Some(r) = git_query_command(&["config", "get", "checkout.defaultRemote"]) {
        //println!("default remote: {r}");
        return Some(r);
    }
    let remotes = remotes()?;
    //println!("{remotes:?}");
    if remotes.iter().any(|r| r == "origin") {
        Some(String::from("origin"))
    } else {
        remotes.into_iter().next()
    }
}

fn remote_head(remote: &str) -> Option<String> {
    let remote_head = String::from("refs/remotes/") + remote + "/HEAD";
    git_query_command(&["symbolic-ref", "--short", &remote_head])
}

/// Get the remotes of the repository.
fn remotes() -> Option<Vec<String>> {
    let output = git_query_command(&["remote"])?;
    if output.is_empty() {
        return Some(Vec::new());
    }
    let res = output.lines().map(str::to_owned).collect();
    Some(res)
}

/// Get the upstream tracked branch of `branch`.
fn upstream(branch: String) -> Option<String> {
    git_query_command(&["rev-parse", "--abbrev-ref", &(branch + "@{upstream}")])
}

// Get the tracked remote of a branch.
fn tracked_remote(branch: &str) -> Option<String> {
    git_query_command(&[
        "branch",
        "--list",
        "--format=%(upstream:remotename)",
        branch,
    ])
}
