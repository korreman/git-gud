use crate::helpers::*;
use crate::tree::*;

// Helper ASTs

const POSITION_STR: &'static str = "%";

fn target_branch(prefix: Str) -> Node {
    or(&[
        custom("h", prefix, current_branch),
        custom("u", prefix, current_upstream),
        custom("m", prefix, main_branch),
        custom("o", prefix, main_remote_head),
    ])
}

fn remote() -> Node {
    or(&[
        custom("h", " ", current_remote),
        custom("o", " ", main_remote),
    ])
}

fn recurse_submodules() -> Node {
    flag("rs", "recurse-submodules")
}

fn commit_or_user_input(prefix: Str) -> Node {
    or(&[
        commit(prefix),
        term("=", prefix, POSITION_STR, None),
        noneat(prefix, POSITION_STR),
    ])
}

fn commit(prefix: Str) -> Node {
    or(&[
        target_branch(prefix),
        term("-", prefix, "HEAD~", Some(opt(number("")))),
        term(
            "@",
            prefix,
            "HEAD@{",
            Some(seq(&[number(""), noneat("", "}")])),
        ),
    ])
}

fn message() -> Node {
    param("m", "message", noneat("=", POSITION_STR))
}

fn track() -> Node {
    or(&[
        flag("nt", "no-track"),
        param(
            "t",
            "track",
            opt(or(&[
                param_variant("d", "direct"),
                param_variant("i", "indirect"),
            ])),
        ),
    ])
}

fn pretty() -> Node {
    param(
        "f",
        "pretty",
        opt(or(&[
            param_variant("o", "oneline"),
            param_variant("s", "short"),
            param_variant("m", "medium"),
            param_variant("ff", "fuller"),
            param_variant("f", "full"),
            param_variant("rf", "reference"),
            param_variant("r", "raw"),
            param_variant("e", "email"),
            param_variant("=", "format:%"),
            param_variant("=t", "tformat:%"),
        ])),
    )
}

fn reflog_expire_param() -> Node {
    or(&[
        term("a", "=", "all", None),
        term("n", "=", "never", None),
        term("=", "=", POSITION_STR, None),
        noneat("=", POSITION_STR),
    ])
}

/// Generate the grammar for all commands.
pub fn ast() -> Node {
    or(&[
        subcmd(
            "a",
            "add",
            seq(&[
                set(&[
                    flag("na", "no-all"),
                    flag("a", "all"),
                    flag("v", "verbose"),
                    flag("d", "dry-run"),
                    flag("f", "force"),
                    flag("s", "sparse"),
                    flag("i", "interactive"),
                    flag("N", "intent-to-add"),
                    flag("r", "refresh"),
                    flag("u", "update"),
                    flag("p", "patch"),
                ]),
                opt(or(&[term(".", " ", ".", None), term("/", " ", ":/", None)])),
            ]),
        ),
        subcmd(
            "bl",
            "blame",
            set(&[
                shortflag("t", "t"),
                shortflag("w", "w"),
                shortflag("s", "s"),
                flag("1", "first-parent"),
                term("l", " -", "L", Some(noneat(" ", POSITION_STR))),
                flag("n", "show-number"),
            ]),
        ),
        subcmd(
            "b",
            "branch",
            set(&[
                flag("f", "force"),
                flag("d", "delete"),
                param("me", "merged", commit_or_user_input("=")),
                param("nme", "no-merged", commit_or_user_input("=")),
                flag("m", "move"),
                flag("c", "copy"),
                flag("r", "remotes"),
                flag("a", "all"),
                shortflag("vv", "vv"),
                flag("v", "verbose"),
                flag("q", "quiet"),
                param("u", "set-upstream-to", commit_or_user_input("=")),
                track(),
            ]),
        ),
        subcmd(
            "c",
            "commit",
            set(&[
                flag("a", "amend"),
                flag("d", "dry-run"),
                flag("ne", "no-edit"),
                flag("e", "edit"),
                flag("nv", "no-verify"),
                flag("v", "verify"),
                flag("i", "include"),
                flag("o", "only"),
                flag("st", "status"),
                flag("s", "signoff"),
                flag("ng", "no-gpg-sign"),
                param("g", "gpg-sign", opt(term("=", "=", POSITION_STR, None))),
                param(
                    "f",
                    "fixup=",
                    seq(&[
                        opt(set(&[
                            term("a", "", "amend:", None),
                            term("r", "", "reword:", None),
                        ])),
                        commit_or_user_input(""),
                    ]),
                ),
                param("q", "squash", commit_or_user_input("=")),
                param("c", "reedit-message", commit_or_user_input("=")),
                param("C", "reuse-message", commit_or_user_input("=")),
                message(),
            ]),
        ),
        subcmd(
            "d",
            "diff",
            seq(&[
                set(&[
                    flag("r", "raw"),
                    flag("m", "minimal"),
                    flag("h", "histogram"),
                    flag("p", "patience"),
                    flag("ss", "shortstat"),
                    flag("s", "stat"),
                    flag("ni", "no-indent-heuristic"),
                    flag("i", "indent-heuristic"),
                    flag("b", "ignore-space-change"),
                    flag("w", "ignore-all-space"),
                    param("u", "unified", number("=")),
                ]),
                opt(commit(" ")),
            ]),
        ),
        subcmd(
            "e",
            "rebase",
            or(&[
                flag("a", "abort"),
                flag("c", "continue"),
                flag("e", "edit-todo"),
                flag("h", "show"),
                flag("q", "quit"),
                flag("s", "skip"),
                seq(&[
                    set(&[
                        flag("i", "interactive"),
                        flag("nf", "no-ff"),
                        flag("r", "root"),
                        flag("q", "quiet"),
                        flag("ns", "no-stat"),
                        flag("s", "stat"),
                        flag("nu", "no-update-refs"),
                        flag("u", "update-refs"),
                        flag("nV", "no-verify"),
                        flag("V", "verify"),
                        flag("v", "verbose"),
                    ]),
                    opt(commit(" ")),
                ]),
            ]),
        ),
        subcmd(
            "f",
            "fetch",
            set(&[
                flag("4", "ipv4"),
                flag("6", "ipv6"),
                flag("na", "no-all"),
                flag("a", "all"),
                flag("A", "append"),
                flag("d", "dry-run"),
                flag("f", "force"),
                flag("k", "keep"),
                flag("m", "multiple"),
                flag("p", "prune"),
                flag("nt", "no-tags"),
                flag("t", "tags"),
            ]),
        ),
        subcmd(
            "g",
            "checkout",
            seq(&[
                set(&[
                    shortflag("b", "b"),
                    shortflag("bb", "B"),
                    shortflag("B", "B"),
                    shortflag("l", "l"),
                    flag("f", "force"),
                    flag("ng", "no-guess"),
                    flag("g", "guess"),
                    flag("d", "detach"),
                    flag("m", "merge"),
                    flag("p", "patch"),
                    flag("no", "no-overlay"),
                    flag("os", "ours"),
                    flag("ts", "theirs"),
                    track(),
                ]),
                opt(commit(" %")),
            ]),
        ),
        subcmd(
            "h",
            "show",
            set(&[
                flag("o", "oneline"),
                flag("nn", "no-notes"),
                flag("a", "abbrev-commit"),
                flag("s", "no-patch"),
                shortflag("m", "m"),
                pretty(),
            ]),
        ),
        subcmd("i", "init", set(&[flag("b", "bare")])),
        subcmd(
            "k",
            "clone",
            set(&[
                flag("1", "single-branch"),
                flag("0", "bare"),
                flag("h", "shared"),
                flag("l", "local"),
                flag("m", "mirror"),
                flag("ng", "no-checkout"),
                flag("s", "sparse"),
                flag("nt", "no-tags"),
                flag("nhl", "no-hardlinks"),
                flag("t", "tags"),
                param("b", "branch", noneat(" ", POSITION_STR)),
                param("d", "depth", number("=")),
                param("d", "dissociate", noneat(" ", POSITION_STR)),
                param("j", "jobs", number("=")),
                param("o", "origin", noneat(" ", POSITION_STR)),
                param("rf", "reference", noneat(" ", POSITION_STR)),
                param("rv", "revision", noneat(" ", POSITION_STR)),
            ]),
        ),
        subcmd(
            "l",
            "log",
            seq(&[
                set(&[
                    flag("ac", "abbrev-commit"),
                    flag("nac", "no-abbrev-commit"),
                    flag("1", "first-parent"),
                    flag("o", "oneline"),
                    param(
                        "d",
                        "decorate",
                        opt(or(&[
                            param_variant("s", "short"),
                            param_variant("f", "full"),
                            param_variant("a", "auto"),
                            param_variant("n", "no"),
                        ])),
                    ),
                    flag("nd", "no-decorate"),
                    flag("F", "follow"),
                    flag("me", "merges"),
                    flag("a", "all"),
                    flag("g", "graph"),
                    flag("p", "patch"),
                    flag("b", "ignore-space-change"),
                    flag("w", "ignore-all-space"),
                    param("m", "max-count", number("=")),
                    pretty(),
                ]),
                opt(commit(" ")),
            ]),
        ),
        subcmd(
            "m",
            "merge",
            or(&[
                flag("c", "continue"),
                flag("a", "abort"),
                flag("q", "quit"),
                seq(&[set(&[]), opt(commit(" "))]),
            ]),
        ),
        subcmd(
            "p",
            "push",
            seq(&[
                set(&[
                    flag("nt", "no-tags"),
                    flag("t", "tags"),
                    flag("nth", "no-thin"),
                    flag("th", "thin"),
                    flag("f", "force"),
                    flag("d", "dry-run"),
                    flag("q", "quiet"),
                    flag("v", "verbose"),
                    flag("V", "verify"),
                    flag("nV", "no-verify"),
                    flag("4", "ipv4"),
                    flag("6", "ipv6"),
                    flag("u", "set-upstream"),
                ]),
                opt(seq(&[remote(), opt(target_branch(" "))])),
            ]),
        ),
        subcmd(
            "q",
            "pull",
            set(&[
                flag("a", "all"),
                flag("p", "prune"),
                flag("nv", "no-verify"),
                flag("v", "verify"),
                flag("ffo", "ff-only"),
                flag("ff", "ff"),
                flag("f", "force"),
                flag("nff", "no-ff"),
                flag("nr", "no-rebase"),
                flag("r", "rebase"),
                flag("d", "dry-run"),
                flag("nt", "no-tags"),
                flag("t", "tags"),
                flag("4", "ipv4"),
                flag("6", "ipv6"),
            ]),
        ),
        subcmd(
            "rl",
            "reflog",
            opt(or(&[
                subcmd("s", "show", seq(&[])), // TODO: All of the log expansions
                subcmd("l", "list", seq(&[])),
                subcmd(
                    "e",
                    "list",
                    set(&[
                        flag("a", "all"),
                        flag("d", "dry-run"),
                        flag("r", "rewrite"),
                        flag("sf", "stale-fix"),
                        flag("sw", "single-worktree"),
                        flag("u", "updateref"),
                        param("e", "expire", reflog_expire_param()),
                        param("eu", "expire-unreachable", reflog_expire_param()),
                    ]),
                ),
                subcmd(
                    "d",
                    "delete",
                    set(&[
                        flag("d", "dry-run"),
                        flag("r", "rewrite"),
                        flag("u", "updateref"),
                    ]),
                ),
                subcmd(
                    "D",
                    "drop",
                    opt(term(
                        "a",
                        " ",
                        "all",
                        Some(opt(flag("sw", "single-worktree"))),
                    )),
                ),
                subcmd("x", "exists", set(&[])),
            ])),
        ),
        subcmd(
            "r",
            "reset",
            seq(&[
                opt(or(&[
                    flag("s", "soft"),
                    flag("h", "hard"),
                    flag("m", "merge"),
                    flag("k", "keep"),
                    flag("r", "recurse-submodules"),
                ])),
                set(&[
                    flag("q", "quiet"),
                    flag("nr", "no-refresh"),
                    flag("nr", "no-refresh"),
                ]),
                opt(commit(" ")),
            ]),
        ),
        subcmd(
            "s",
            "switch",
            seq(&[
                set(&[
                    flag("fc", "force-create"),
                    flag("f", "force"),
                    flag("C", "force-create"),
                    flag("c", "create"),
                    flag("d", "detach"),
                    flag("ng", "no-guess"),
                    flag("iow", "ignore-other-worktrees"),
                    track(),
                ]),
                opt(target_branch(" ")),
            ]),
        ),
        subcmd(
            "t",
            "tag",
            set(&[
                flag("a", "annotate"),
                flag("s", "sign"),
                flag("ns", "no-sign"),
                flag("f", "force"),
                flag("d", "delete"),
                flag("v", "verify"),
                flag("l", "list"),
                flag("ic", "ignore-case"),
                flag("oe", "omit-empty"),
                flag("oe", "omit-empty"),
                flag("e", "edit"),
                message(),
            ]),
        ),
        subcmd(
            "u",
            "restore",
            set(&[
                flag("p", "patch"),
                flag("w", "worktree"),
                flag("i", "staged"), // i for index
                flag("o", "ours"),
                flag("t", "theirs"),
                flag("m", "merge"),
                param("s", "source", term("", "=", POSITION_STR, None)),
                recurse_submodules(),
            ]),
        ),
        subcmd(
            "v",
            "status",
            set(&[
                flag("s", "short"),
                flag("l", "long"),
                flag("z", "show-stash"),
                flag("v", "verbose"),
                flag("a", "ahead-behind"),
                flag("na", "no-ahead-behind"),
                flag("r", "renames"),
                flag("nr", "no-renames"),
                param("fr", "find-renames", number("=")),
                param(
                    "u",
                    "untracked-files",
                    opt(or(&[
                        param_variant("no", "no"),
                        param_variant("n", "normal"),
                        param_variant("a", "all"),
                    ])),
                ),
                param(
                    "i",
                    "ignored",
                    opt(or(&[
                        param_variant("no", "no"),
                        param_variant("t", "traditional"),
                        param_variant("m", "matching"),
                    ])),
                ),
            ]),
        ),
        subcmd(
            "w",
            "worktree",
            or(&[
                subcmd(
                    "a",
                    "add",
                    set(&[
                        flag("f", "force"),
                        flag("d", "detach"),
                        flag("nc", "no-checkout"),
                        flag("ng", "no-guess-remote"),
                        flag("nrp", "no-relative-paths"),
                        flag("nt", "no-track"),
                        flag("l", "lock"),
                        flag("o", "orphan"),
                        flag("q", "quiet"),
                    ]),
                ),
                subcmd("v", "list", set(&[flag("v", "verbose")])),
                subcmd("m", "move", set(&[flag("f", "force")])),
                subcmd(
                    "p",
                    "prune",
                    set(&[flag("d", "dry-run"), flag("v", "verbose")]),
                ),
                subcmd("r", "remove", set(&[flag("f", "force")])),
                subcmd("R", "repair", set(&[])),
                subcmd("l", "lock", set(&[])),
                subcmd("u", "unlock", set(&[])),
            ]),
        ),
        subcmd(
            "x",
            "clean",
            set(&[
                shortflag("d", "d"),
                shortflag("xx", "xX"),
                shortflag("x", "x"),
                shortflag("X", "X"),
                flag("f", "force"),
                flag("i", "interactive"),
                flag("d", "dry-run"),
                flag("q", "quiet"),
            ]),
        ),
        subcmd(
            "z",
            "stash",
            opt(or(&[
                subcmd(
                    "p",
                    "push",
                    set(&[
                        flag("a", "all"),
                        flag("p", "patch"),
                        flag("s", "staged"),
                        message(),
                    ]),
                ),
                subcmd("o", "pop", set(&[])),
                subcmd(
                    "s",
                    "save",
                    set(&[
                        flag("a", "all"),
                        flag("p", "patch"),
                        flag("s", "staged"),
                        flag("q", "quiet"),
                    ]),
                ),
                subcmd("l", "list", set(&[])),
                subcmd("h", "show", set(&[])),
                subcmd("d", "drop", set(&[])),
                subcmd("a", "apply", set(&[])),
                subcmd("b", "branch", set(&[])),
                subcmd("c", "clear", set(&[])),
                subcmd("m", "create", set(&[])),
                subcmd("t", "store", set(&[])),
            ])),
        ),
    ])
}
