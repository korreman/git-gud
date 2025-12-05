use crate::nodes::*;

pub fn ast() -> Node {
    or(&[
        &subcmd(
            "a",
            "add",
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
                &opt(or(&[
                    &term(".", " ", ".", None),
                    &term("-", " ", "--", None),
                ])),
            ]),
        ),
        &subcmd(
            "bl",
            "blame",
            set(&[
                &shortflag("t", "t"),
                &shortflag("w", "w"),
                &shortflag("s", "s"),
                &flag("1", "first-parent"),
                &term("l", " -", "L", Some(noneat(" ", "%"))),
                &flag("n", "show-number"),
            ]),
        ),
        &subcmd(
            "b",
            "branch",
            set(&[
                &flag("f", "force"),
                &flag("d", "delete"),
                &flag("m", "move"),
                &flag("c", "copy"),
                &flag("r", "remotes"),
                &flag("a", "all"),
                &flag("vv", "vv"),
                &flag("v", "verbose"),
                &flag("q", "quiet"),
                &flag("nt", "no-track"),
                &param(
                    "t",
                    "track",
                    opt(or(&[
                        &param_variant("d", "direct"),
                        &param_variant("i", "indirect"),
                    ])),
                ),
            ]),
        ),
        &subcmd(
            "c",
            "commit",
            seq(&[
                &set(&[
                    &flag("a", "amend"),
                    &flag("A", "all"),
                    &flag("p", "patch"),
                    &flag("d", "dry-run"),
                    &flag("ne", "no-edit"),
                    &flag("e", "edit"),
                    &flag("nv", "no-verify"),
                    &flag("V", "verify"),
                    &flag("i", "include"),
                    &flag("o", "only"),
                    &flag("s", "status"),
                    &param("m", "message", noneat("=", "%")),
                ]),
                &opt(term("-", " ", "--", None)),
            ]),
        ),
        &subcmd(
            "d",
            "diff",
            seq(&[
                &set(&[
                    &flag("r", "raw"),
                    &flag("m", "minimal"),
                    &flag("h", "histogram"),
                    &flag("p", "patience"),
                    &flag("ss", "shortstat"),
                    &flag("s", "stat"),
                    &flag("ni", "no-indent-heuristic"),
                    &flag("i", "indent-heuristic"),
                    &flag("b", "ignore-space-change"),
                    &flag("w", "ignore-all-space"),
                    &param("u", "unified", number("=")),
                ]),
                &opt(local_target()),
            ]),
        ),
        &subcmd(
            "e",
            "rebase",
            or(&[
                &flag("a", "abort"),
                &flag("c", "continue"),
                &flag("e", "edit-todo"),
                &flag("h", "show"),
                &flag("q", "quit"),
                &flag("s", "skip"),
                &seq(&[
                    &set(&[
                        &flag("i", "interactive"),
                        &flag("nf", "no-ff"),
                        &flag("r", "root"),
                        &flag("q", "quiet"),
                        &flag("ns", "no-stat"),
                        &flag("s", "stat"),
                        &flag("nu", "no-update-refs"),
                        &flag("u", "update-refs"),
                        &flag("nV", "no-verify"),
                        &flag("V", "verify"),
                        &flag("v", "verbose"),
                    ]),
                    &opt(local_target()),
                ]),
            ]),
        ),
        &subcmd(
            "f",
            "fetch",
            set(&[
                &flag("4", "ipv4"),
                &flag("6", "ipv6"),
                &flag("na", "no-all"),
                &flag("a", "all"),
                &flag("A", "append"),
                &flag("d", "dry-run"),
                &flag("f", "force"),
                &flag("k", "keep"),
                &flag("m", "multiple"),
                &flag("p", "prune"),
                &flag("nt", "no-tags"),
                &flag("t", "tags"),
            ]),
        ),
        &subcmd(
            "g",
            "checkout",
            set(&[
                &flag("q", "quiet"),
                &flag("f", "force"),
                &flag("g", "guess"),
                &flag("d", "detach"),
                &flag("m", "merge"),
                &flag("p", "patch"),
                &flag("no", "no-overlay"),
                &flag("o", "overlay"),
            ]),
        ),
        &subcmd(
            "h",
            "show",
            set(&[&flag("o", "oneline"), &flag("nn", "no-notes")]),
        ),
        &subcmd("i", "init", set(&[&flag("b", "bare")])),
        &subcmd(
            "k",
            "clone",
            set(&[
                &flag("l", "local"),
                &flag("h", "shared"),
                &flag("b", "bare"),
                &flag("s", "sparse"),
                &flag("nt", "no-tags"),
                &flag("t", "tags"),
                &param("d", "depth", number("=")),
                &param("j", "jobs", number("=")),
            ]),
        ),
        &subcmd(
            "l",
            "log",
            set(&[
                &flag("1", "first-parent"),
                &flag("o", "oneline"),
                &flag("nd", "decorate"),
                &flag("d", "decorate"),
                &flag("f", "follow"),
                &flag("m", "merges"),
                &flag("a", "all"),
                &flag("g", "graph"),
                &flag("n", "no-patch"),
                &flag("p", "patch"),
                &flag("b", "ignore-space-change"),
                &flag("w", "ignore-all-space"),
                &param("d", "depth", number("=")),
            ]),
        ),
        &subcmd("m", "merge", set(&[])),
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
