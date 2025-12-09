use crate::helpers::*;
use crate::tree::*;

/// Generate the grammar for all commands.
pub fn ast() -> Node {
    // two-letter shortcuts we cannot use:
    // gh: github cli
    // go: golang compiler
    or([
        map("a", add()),
        map("bl", blame()),
        map("b", branch()),
        map("c", commit()),
        map("d", diff()),
        map("e", rebase()),
        map("f", fetch()),
        map("g", checkout()),
        map("i", init()),
        // j
        map("k", clone()),
        map("l", log()),
        map("m", merge()),
        // n
        map("p", push()),
        map("rl", reflog()),
        map("r", reset()),
        map("st", status()),
        map("s", switch()),
        map("t", tag()),
        map("u", restore()), // undo
        map("v", show()),
        map("w", worktree()),
        map("x", clean()),
        map("y", pull()),  // yank
        map("z", stash()), // marks
    ])
}

fn add() -> Node {
    seq([
        Emit("add"),
        argset([
            flag("na", "no-all"),
            flag("a", "all"),
            flag("d", "dry-run"),
            flag("f", "force"),
            flag("s", "sparse"),
            flag("i", "interactive"),
            flag("N", "intent-to-add"),
            flag("r", "refresh"),
            flag("u", "update"),
            flag("p", "patch"),
        ]),
        opt(arg(or([word(".", "."), word("/", ":/")]))),
    ])
}

fn blame() -> Node {
    seq([
        Emit("blame"),
        argset([
            f("t", "t"),
            f("w", "w"),
            f("s", "s"),
            flag("1", "first-parent"),
            seq([f("l", "L"), Emit(" "), Emit(CURSOR)]),
            flag("n", "show-number"),
        ]),
    ])
}

fn branch() -> Node {
    seq([
        Emit("branch"),
        argset([
            flag("f", "force"),
            flag("d", "delete"),
            param_opt("mr", "merged", target_commit()),
            param_opt("nm", "no-merged", target_commit()),
            flag("m", "move"),
            flag("c", "copy"),
            flag("r", "remotes"),
            flag("a", "all"),
            f("vv", "vv"),
            param("u", "set-upstream-to", target_commit()),
            track(),
        ]),
    ])
}

fn commit() -> Node {
    seq([
        Emit("commit"),
        argset([
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
            param_opt("g", "gpg-sign", fail()),
            param(
                "f",
                "fixup",
                seq([
                    opt(set([word("a", "amend:"), word("r", "reword:")])),
                    or([target_commit(), Emit(CURSOR)]),
                ]),
            ),
            param("q", "squash", target_commit()),
            param("c", "reedit-message", target_commit()),
            param("C", "reuse-message", target_commit()),
            message(),
        ]),
    ])
}

fn diff() -> Node {
    seq([
        Emit("diff"),
        argset([
            flag("r", "raw"),
            flag("m", "minimal"),
            flag("h", "histogram"),
            flag("p", "patience"),
            flag("ss", "shortstat"),
            flag("s", "stat"),
            flag("nih", "no-indent-heuristic"),
            flag("ih", "indent-heuristic"),
            flag("ni", "no-index"),
            flag("b", "ignore-space-change"),
            flag("w", "ignore-all-space"),
            param("u", "unified", number_or_zero()),
        ]),
        opt(arg(target_commit())),
    ])
}

fn rebase() -> Node {
    seq([
        Emit("rebase"),
        or([
            arg(flag("a", "abort")),
            arg(flag("c", "continue")),
            arg(flag("e", "edit-todo")),
            arg(flag("h", "show-current-patch")),
            arg(flag("q", "quit")),
            arg(flag("s", "skip")),
            seq([
                argset([
                    flag("i", "interactive"),
                    flag("nf", "no-ff"),
                    flag("r", "root"),
                    flag("ns", "no-stat"),
                    flag("s", "stat"),
                    flag("nu", "no-update-refs"),
                    flag("u", "update-refs"),
                    flag("nV", "no-verify"),
                    flag("V", "verify"),
                ]),
                opt(arg(target_commit())),
            ]),
        ]),
    ])
}

fn fetch() -> Node {
    seq([
        Emit("fetch"),
        argset([
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
    ])
}

fn checkout() -> Node {
    seq([
        Emit("checkout"),
        argset([
            f("b", "b"),
            f("bb", "B"),
            f("B", "B"),
            f("l", "l"),
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
        opt(arg(target_commit())),
    ])
}

// TODO: Show have most of the same options as diff command
fn show() -> Node {
    seq([
        Emit("show"),
        argset([
            flag("o", "oneline"),
            flag("nn", "no-notes"),
            flag("a", "abbrev-commit"),
            flag("np", "no-patch"),
            flag("b", "ignore-space-change"),
            flag("s", "stat"),
            f("m", "m"),
            pretty(),
        ]),
        opt(arg(target_commit())),
    ])
}

fn init() -> Node {
    seq([
        Emit("init"),
        argset([flag("b", "bare")]),
        opt(arg(target_commit())),
    ])
}

fn clone() -> Node {
    seq([
        Emit("clone"),
        argset([
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
            param("b", "branch", Emit(CURSOR)),
            param("d", "depth", Number),
            param("d", "dissociate", Emit(CURSOR)),
            param("j", "jobs", Number),
            param("o", "origin", Emit(CURSOR)),
            param("rf", "reference", Emit(CURSOR)),
            param("rv", "revision", Emit(CURSOR)),
        ]),
        opt(arg(target_commit())),
    ])
}

fn log() -> Node {
    seq([
        Emit("log"),
        argset([
            flag("ac", "abbrev-commit"),
            flag("s", "stat"),
            flag("nac", "no-abbrev-commit"),
            flag("1", "first-parent"),
            flag("o", "oneline"),
            param_opt(
                "d",
                "decorate",
                or([
                    word("s", "short"),
                    word("f", "full"),
                    word("a", "auto"),
                    word("n", "no"),
                ]),
            ),
            flag("nd", "no-decorate"),
            flag("F", "follow"),
            flag("m", "merges"),
            param("n", "max-count", Number),
            flag("a", "all"),
            flag("g", "graph"),
            flag("p", "patch"),
            flag("b", "ignore-space-change"),
            flag("w", "ignore-all-space"),
            pretty(),
        ]),
        opt(arg(target_commit())),
    ])
}

fn merge() -> Node {
    seq([
        Emit("merge"),
        arg(or([
            flag("c", "continue"),
            flag("a", "abort"),
            flag("q", "quit"),
            seq([argset([]), opt(target_commit())]),
        ])),
        opt(arg(target_commit())),
    ])
}

fn push() -> Node {
    seq([
        Emit("push"),
        argset([
            flag("nt", "no-tags"),
            flag("t", "tags"),
            flag("nth", "no-thin"),
            flag("th", "thin"),
            flag("ff", "force"),
            flag("f", "force-with-lease"),
            flag("d", "dry-run"),
            flag("V", "verify"),
            flag("nV", "no-verify"),
            flag("4", "ipv4"),
            flag("6", "ipv6"),
            flag("u", "set-upstream"),
        ]),
        opt(arg(seq([remote(), opt(arg(target_branch()))]))),
    ])
}

fn pull() -> Node {
    seq([
        Emit("pull"),
        argset([
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
    ])
}

fn reflog() -> Node {
    seq([
        Emit("reflog"),
        arg(or([
            seq([
                word("s", "show"), // TODO: add all log expansions here
                argset([
                    flag("a", "all"),
                    flag("d", "dry-run"),
                    flag("r", "rewrite"),
                    flag("sf", "stale-fix"),
                    flag("sw", "single-worktree"),
                    flag("u", "updateref"),
                    param("eu", "expire-unreachable", reflog_expire_param()),
                    param("e", "expire", reflog_expire_param()),
                ]),
            ]),
            word("l", "list"),
            word("e", "exists"),
            word("x", "expire"),
            seq([
                word("d", "delete"),
                argset([
                    flag("d", "dry-run"),
                    flag("r", "rewrite"),
                    flag("u", "updateref"),
                ]),
            ]),
            seq([
                word("D", "drop"),
                opt(seq([
                    arg(flag("a", "all")),
                    opt(arg(flag("sw", "single-worktree"))),
                ])),
            ]),
        ])),
    ])
}

fn reset() -> Node {
    seq([
        Emit("reset"),
        opt(arg(or([
            flag("s", "soft"),
            flag("h", "hard"),
            flag("m", "merge"),
            flag("k", "keep"),
            flag("r", "recurse-submodules"),
        ]))),
        argset([flag("nr", "no-refresh")]),
        opt(arg(target_commit())),
    ])
}

fn switch() -> Node {
    seq([
        Emit("switch"),
        argset([
            flag("fc", "force-create"),
            flag("f", "force"),
            flag("C", "force-create"),
            flag("c", "create"),
            flag("d", "detach"),
            flag("ng", "no-guess"),
            flag("iow", "ignore-other-worktrees"),
            track(),
        ]),
        opt(arg(target_branch())),
    ])
}

fn tag() -> Node {
    seq([
        Emit("tag"),
        argset([
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
            param_opt("mr", "merged", target_commit()),
            param_opt("nm", "no-merged", target_commit()),
            flag("e", "edit"),
            message(),
        ]),
    ])
}

fn restore() -> Node {
    seq([
        Emit("restore"),
        argset([
            flag("p", "patch"),
            flag("w", "worktree"),
            flag("i", "staged"), // i for index
            flag("o", "ours"),
            flag("t", "theirs"),
            flag("m", "merge"),
            param("s", "source", target_commit()),
            recurse_submodules(),
        ]),
    ])
}

fn status() -> Node {
    seq([
        Emit("status"),
        argset([
            flag("s", "short"),
            flag("l", "long"),
            flag("z", "show-stash"),
            flag("a", "ahead-behind"),
            flag("na", "no-ahead-behind"),
            flag("r", "renames"),
            flag("nr", "no-renames"),
            param("fr", "find-renames", Number),
            param(
                "u",
                "untracked-files",
                opt(or([
                    word("no", "no"),
                    word("n", "normal"),
                    word("a", "all"),
                ])),
            ),
            param(
                "i",
                "ignored",
                opt(or([
                    word("no", "no"),
                    word("t", "traditional"),
                    word("m", "matching"),
                ])),
            ),
        ]),
    ])
}

fn worktree() -> Node {
    seq([
        Emit("worktree"),
        arg(or([
            seq([
                word("a", "add"),
                argset([
                    flag("f", "force"),
                    flag("d", "detach"),
                    flag("nc", "no-checkout"),
                    flag("ng", "no-guess-remote"),
                    flag("nrp", "no-relative-paths"),
                    flag("nt", "no-track"),
                    flag("l", "lock"),
                    flag("o", "orphan"),
                ]),
            ]),
            seq([word("v", "list"), argset([])]),
            seq([word("m", "move"), argset([flag("f", "force")])]),
            seq([word("p", "prune"), argset([flag("d", "dry-run")])]),
            seq([word("r", "remove"), argset([flag("f", "force")])]),
            seq([word("R", "repair"), argset([])]),
            seq([word("l", "lock"), argset([])]),
            seq([word("u", "unlock"), argset([])]),
        ])),
    ])
}

fn clean() -> Node {
    seq([
        Emit("clean"),
        argset([
            f("d", "d"),
            f("xx", "xX"),
            f("x", "x"),
            f("X", "X"),
            flag("f", "force"),
            flag("i", "interactive"),
            flag("d", "dry-run"),
        ]),
    ])
}

fn stash() -> Node {
    seq([
        Emit("stash"),
        opt(arg(or([
            seq([
                word("p", "push"),
                argset([
                    flag("a", "all"),
                    flag("p", "patch"),
                    flag("s", "staged"),
                    message(),
                ]),
            ]),
            seq([word("o", "pop"), argset([])]),
            seq([
                word("s", "save"),
                argset([flag("a", "all"), flag("p", "patch"), flag("s", "staged")]),
            ]),
            seq([word("l", "list"), argset([])]),
            seq([word("h", "show"), argset([])]),
            seq([word("d", "drop"), argset([])]),
            seq([word("a", "apply"), argset([])]),
            seq([word("b", "branch"), argset([])]),
            seq([word("c", "clear"), argset([])]),
            seq([word("m", "create"), argset([])]),
            seq([word("t", "store"), argset([])]),
        ]))),
    ])
}

// Helpers

fn target_branch() -> Node {
    or([
        word("h", "HEAD"),
        map_custom("c", current_branch),
        map_custom("u", current_upstream),
        map_custom("m", main_branch),
        map_custom("o", main_remote_head),
    ])
}

fn remote() -> Node {
    or([
        map_custom("c", current_remote),
        map_custom("o", main_remote),
    ])
}

fn recurse_submodules() -> Node {
    flag("rs", "recurse-submodules")
}

fn target_commit() -> Node {
    or([
        target_branch(),
        seq([word("-", "HEAD~"), opt(Number)]),
        seq([word("@", "HEAD@{"), Number, Emit("}")]),
    ])
}

fn message() -> Node {
    param("m", "message", seq([Emit("\""), Emit(CURSOR), Emit("\"")]))
}

fn track() -> Node {
    or([
        flag("nt", "no-track"),
        param(
            "t",
            "track",
            opt(or([word("d", "direct"), word("i", "indirect")])),
        ),
    ])
}

fn pretty() -> Node {
    param_opt(
        "f",
        "pretty",
        or([
            word("o", "oneline"),
            word("s", "short"),
            word("m", "medium"),
            word("ff", "fuller"),
            word("f", "full"),
            word("rf", "reference"),
            word("r", "raw"),
            word("e", "email"),
            word("_", "format:%"),
            word("t_", "tformat:%"),
        ]),
    )
}

fn reflog_expire_param() -> Node {
    or([word("a", "all"), word("n", "never")])
}
