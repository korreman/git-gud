# Useful shorthand commands

## Commit

| Shorthand | Command | Explainer |
|---|---|---|
| `gacm` | `git add --all && git commit --message="%"` | Commit all changes with a message. |
| `gacan` | `git add --all && git commit --amend --no-edit` | Amend latest commit with all current changes, without editing the message. |

## Diff

| Shorthand | Command | Explainer |
|---|---|---|
| `gdmbm` | `git diff --merge-base <MAIN_BRANCH>` | Diff current branch with the branch-off point from the main branch. |

## Fetch & push

| Shorthand | Command | Explainer |
|---|---|---|
| `gpuoc` | `git push --set-upstream <MAIN_REMOTE> <CURRENT_BRANCH>` | Push while setting up the current branch to track the same on remote. |

## Rebase

| Shorthand | Command | Explainer |
|---|---|---|
| `gei-10` | `git rebase --interactive HEAD~10` | Interactive rebase of the latest 10 commits. |
| `geuri-10` | `git rebase --interactive HEAD~10` | Interactive rebase of the latest 10 commits, updating refs as well. |
| `gaec` | `git add --all && git rebase --continue` | Add all changes and continue the rebase. |
