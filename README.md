# git-gud
__A Git shorthand syntax for the Fish shell that expands directly in the prompt.__

___This project is very much a WIP.
All of the shorthand syntax is unstable and___ __will__ ___be changed on a whim.___

Using the
[abbreviation](https://fishshell.com/docs/current/cmds/abbr.html)
feature of the Fish shell,
`git-gud` lets you quickly type complex git commands using a custom shorthand syntax.
Commands (that aren't bound to anything else) starting with `g` get expanded,
potentially placing the cursor in a specific position to type a commit message
or similar parameter.

## Syntax
The shorthand syntax is a sequence of letters and "words" with no separators between them.
With no separators, we disambiguate tokenization by
always eating the longest matching applicable shortstring.
For example, the string `grldr` is split and expanded as follows:
- `g`: `git`
- `rl`: `reflog`
- `d`: `delete`
- `r`: `--rewrite`

The resulting command, `git reflog delete --rewrite`,
is substituted directly into the terminal while typing.

## Examples
In these examples:
- `%` is where the cursor will be placed after the command is expanded after pressing space.
- `MAIN_BRANCH` is the branch tracking the `HEAD` of the main remote
  (the checkout default, origin, or first remote listed).
  This will typically resolve to something like `main`, `master`, or `trunk`.

| Shorthand | Command |
|---|---|
| `ga` | `git add --all` |
| `gcm` | `git commit --message="%"` |
| `gapcm` | `git add --patch --all && git commit --message="%"` |
| `gaec` | `git add --all && git rebase --continue` |
| `ge-urm` | `git rebase --no-update-refs <MAIN_BRANCH>` |
| `gkmd20b` | `git clone --mirror --depth=20 --branch=%` |

## Advantages over aliases
- Abbreviations can be expanded before you run them,
  and they can be undone to modify your shorthand expression before expanding again.
- Commands are composed from a syntax, not just matched against a set of pre-defined aliases.
  A much wider range of commands can be expanded.

## Non-goals
- `git-gud` doesn't try to match the syntax of commonly used sets of git aliases.
  Those are made without abbreviations and shorthand notation in mind.
- The shorthand doesn't try to cover all possible git commands and flags, there are simply too many.
- Matching shorthand letters with the short forms of flags is not always the first priority.
- Typing speed may be prioritized over good mnemonics.

## Installation
... is left as an exercise for the user.

Figure out how to get `git-gud` installed on the system,
then run `git-gud installer | source` somewhere during Fish shell initialization,
in order to register abbrevations.
