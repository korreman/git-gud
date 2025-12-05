use crate::nodes::*;

#[rustfmt::skip]
pub fn ast() -> Node {
    or(&[
        &subcmd("a", "add",
            seq(&[
                &set(&[
                    &flag("na", "no-all"),
                    &flag("a", "all"),
                    &flag("v", "verbose"),
                    &flag("d", "dry-run"),
                    &flag("f", "force"),
                    &flag("s", "sparse"),
                    &flag("i", "interactive"),
                    &flag("N", "intent-to-add"),
                    &flag("r", "refresh"),
                    &flag("u", "update"),
                    &flag("p", "patch"),
                ]),
                &opt(
                    or(&[
                        &term(".", " ", ".", None),
                        &term("-", " ", "--", None),
                    ])
                ),
            ])
        ),
        &subcmd("bl", "blame", set(&[])),
        &subcmd("b", "branch", set(&[])),
        &subcmd("c", "commit", set(&[])),
        &subcmd("d", "diff", seq(&[
            &local_target()
        ])),
        &subcmd("e", "rebase", set(&[])), // rEbase
        &subcmd("f", "fetch", set(&[])),
        &subcmd("g", "checkout", set(&[])), // goto
        &subcmd("h", "show", set(&[])),     // sHow
        &subcmd("i", "init", set(&[])),
        //("j", _),
        &subcmd("k", "clone", set(&[])),
        &subcmd("l", "log", set(&[])),
        &subcmd("m", "merge", set(&[])),
        //("n", _),
        //("o", _),
        &subcmd("p", "push", set(&[])),
        &subcmd("q", "pull", set(&[])), // visually reversed 'p', on the opposite end of keyboard
        &subcmd("rl", "reflog", set(&[])), // __r__ef__l__og
        &subcmd("r", "reset", set(&[])),
        &subcmd("s", "switch", set(&[])),
        &subcmd("t", "tag", set(&[])),
        &subcmd("u", "restore", set(&[])), // undo
        &subcmd("v", "status", set(&[])),  // View the status
        &subcmd("w", "worktree", set(&[])),
        &subcmd("x", "clean", set(&[])),
        //("y", _),
        &subcmd("z", "stash", set(&[])), // ztash, similar to marks in modal editors
    ])
}

pub fn local_target() -> Node {
    or(&[&term("-", " ", "HEAD~", Some(opt(number(""))))])
}
