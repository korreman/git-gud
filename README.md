# git-gud
__A shorthand syntax for git commands that expands directly in your prompt.__

___This project is very much a WIP.
All of the shorthand syntax is unstable and___ __will__ ___be changed on a whim.
I'm making it public so that I can easily install it through home-manager without setting up secrets.___

Using the [abbreviation](https://fishshell.com/docs/current/cmds/abbr.html) feature of the Fish shell,
`git-gud` lets you quickly type complex git commands using a custom shorthand syntax.
Typing `g ` in an empty prompt is automatically expanded to `git `,
while the first argument is parsed as shorthand syntax and expanded to a valid subcommand.
Subsequent arguments are not changed by `git-gud`.

## Syntax
The shorthand syntax is a sequence of letters and "words" (like `rl`) with no separators between them.
With no separators, we disambiguate tokenization by always eating the longest matching applicable word.
These are parsed into an AST that corresponds to valid git subcommand,
and the long form of this subcommand is generated.

## Examples
In these examples:
- `%` is where the cursor will be placed after the command is expanded after pressing space.
- `MAIN_BRANCH` is the branch tracking the `HEAD` of the main remote
  (the checkout default, origin, or first remote listed).
  This will typically resolve to something like `main`, `master`, or `trunk`.

| Shorthand | Command |
|---|---|
| `g a` | `git add` |
| `g cm` | `git commit --message="%"` |
| `g enum` | `git rebase --no-update-refs <MAIN_BRANCH>` |
| `g kmd20b` | `git clone --mirror --depth=20 --branch=%` |

## How to escape and type normal subcommands
Valid subcommands that aren't supported by `git-gud` will intentionally not be substituted.
Fish abbreviations can always be nullified by surrounding the thing you're typing with `'`.
Examples:
- `git log` expands to `git log --oneline --graph`.
  You can always just type `g l` to run a regular `git log`.
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
