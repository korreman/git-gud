# git-gud (WIP)
__A shorthand syntax for git commands that expands directly in your prompt.__

Using the [abbreviation](https://fishshell.com/docs/current/cmds/abbr.html) feature of the Fish shell,
`git-gud` lets you quickly type complex git commands using a custom shorthand syntax.
Typing `g ` in an empty prompt is automatically expanded to `git `,
while the first argument is parsed as shorthand syntax and expanded to a valid subcommand.
Subsequent arguments are not changed by `git-gud`.

## Examples
| Shorthand | Expanded |
|-|-|
| `g c` | `git commit` |
| `g q` | `git pull` |
| `g pu` | `git push --set-upstream <DEFAULT_REMOTE> <CURRENT_BRANCH_NAME>` |
| `g a.can` | `git add . && git commit --amend --no-edit` |
| `g aec` | `git add --all :/ && git rebase --continue` |
| `g rs-3` | `git reset --soft HEAD~3` |

## Syntax
The shorthand syntax is a sequence of letters and "words" (like `rl`) with no separators between them.
With no separators, we disambiguate tokenization by always eating the longest matching applicable word.
These are parsed into an AST that corresponds to valid git subcommand,
and the long form of this subcommand can be generated.

## How to escape and type normal subcommands
Valid subcommands that aren't supported by `git-gud` will intentionally not be substituted.
Fish abbreviations can always be nullified by surrounding the thing you're typing with `'`.
Examples:
- `git log` expands to `git log --oneline --graph`,
  since you can just type `g l`, which expands to `git log`.
- `git 'log'` is escaped and will run as normal `git log` command.

## Advantages over aliases
- Abbreviations can be expanded before you run them,
  and they can be undone to modify your shorthand expression before expanding again.
- Commands are composed from a syntax, not just matched against a set of pre-defined aliases.
  A much wider range of commands can be expanded.
- `git-gud` expands the first argument provided to a git command rather than the first command you typed.
  This avoids polluting your command entire.

## Non-goals
- `git-gud` doesn't try to match the syntax of common aliases.
  Those are made without abbreviations and shorthand notation in mind.
- The shorthand doesn't try to cover all possible git commands and flags, there are simply too many.
- Matching shorthand letters with the short forms of flags is not the first priority.
- Typing speed is often prioritized over good mnemonics.
