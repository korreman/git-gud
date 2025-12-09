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
            flag("i", "interactive"),
            flag("s", "sparse"),
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
            flag("1", "first-parent"),
            seq([f("l", "L"), Emit(" "), Emit(CURSOR)]),
            flag("n", "show-number"),
            f("s", "s"),
            f("t", "t"),
            f("w", "w"),
        ]),
    ])
}

fn branch() -> Node {
    seq([
        Emit("branch"),
        argset([
            flag("a", "all"),
            flag("c", "copy"),
            flag("d", "delete"),
            flag("f", "force"),
            param_opt("mr", "merged", target_commit()),
            param_opt("nm", "no-merged", target_commit()),
            flag("m", "move"),
            flag("r", "remotes"),
            t_track(),
            param("u", "set-upstream-to", target_commit()),
            flag("v", "verbose"),
            f("vv", "vv"),
        ]),
    ])
}

fn commit() -> Node {
    seq([
        Emit("commit"),
        argset([
            flag("a", "amend"),
            param("C", "reuse-message", target_commit()),
            param("c", "reedit-message", target_commit()),
            flag("d", "dry-run"),
            flag("e", "edit"),
            flag("ne", "no-edit"),
            param(
                "f",
                "fixup",
                seq([
                    opt(set([word("a", "amend:"), word("r", "reword:")])),
                    or([target_commit(), Emit(CURSOR)]),
                ]),
            ),
            param_opt("g", "gpg-sign", fail()),
            flag("ng", "no-gpg-sign"),
            flag("i", "include"),
            m_message(),
            flag("o", "only"),
            param("q", "squash", target_commit()),
            flag("st", "status"),
            flag("s", "signoff"),
            flag("v", "verify"),
            flag("nv", "no-verify"),
        ]),
    ])
}

fn diff() -> Node {
    seq([
        Emit("diff"),
        argset([
            flag("b", "ignore-space-change"),
            flag("h", "histogram"),
            flag("ih", "indent-heuristic"),
            flag("nih", "no-indent-heuristic"),
            flag("m", "minimal"),
            flag("ni", "no-index"),
            flag("p", "patience"),
            flag("r", "raw"),
            flag("ss", "shortstat"),
            flag("s", "stat"),
            param("u", "unified", number_or_zero()),
            flag("w", "ignore-all-space"),
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
                    flag("r", "root"),
                    flag("nf", "no-ff"),
                    flag("s", "stat"),
                    flag("ns", "no-stat"),
                    flag("u", "update-refs"),
                    flag("nu", "no-update-refs"),
                    flag("v", "verify"),
                    flag("nv", "no-verify"),
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
            flag("A", "append"),
            flag("a", "all"),
            flag("na", "no-all"),
            flag("d", "dry-run"),
            flag("f", "force"),
            flag("k", "keep"),
            flag("m", "multiple"),
            flag("p", "prune"),
            flag("t", "tags"),
            flag("nt", "no-tags"),
        ]),
    ])
}

fn checkout() -> Node {
    seq([
        Emit("checkout"),
        argset([
            f("B", "B"),
            f("bb", "B"),
            f("b", "b"),
            f("l", "l"),
            flag("d", "detach"),
            flag("f", "force"),
            flag("g", "guess"),
            flag("ng", "no-guess"),
            flag("m", "merge"),
            flag("no", "no-overlay"),
            ours_theirs(),
            flag("p", "patch"),
            t_track(),
        ]),
        opt(arg(target_commit())),
    ])
}

// TODO: Show have most of the same options as diff command
fn show() -> Node {
    seq([
        Emit("show"),
        argset([
            flag("a", "abbrev-commit"),
            flag("b", "ignore-space-change"),
            f_pretty(),
            f("m", "m"),
            flag("nn", "no-notes"),
            flag("np", "no-patch"),
            flag("o", "oneline"),
            flag("s", "stat"),
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
            flag("0", "bare"),
            flag("1", "single-branch"),
            param("b", "branch", Emit(CURSOR)),
            param("d", "depth", Number),
            param("d", "dissociate", Emit(CURSOR)),
            flag("h", "shared"),
            param("j", "jobs", Number),
            flag("l", "local"),
            flag("m", "mirror"),
            flag("ng", "no-checkout"),
            flag("nhl", "no-hardlinks"),
            param("o", "origin", Emit(CURSOR)),
            flag("s", "sparse"),
            flag("t", "tags"),
            flag("nt", "no-tags"),
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
            flag("1", "first-parent"),
            flag("ac", "abbrev-commit"),
            flag("nac", "no-abbrev-commit"),
            flag("a", "all"),
            flag("b", "ignore-space-change"),
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
            f_pretty(),
            flag("g", "graph"),
            flag("m", "merges"),
            param("n", "max-count", Number),
            flag("o", "oneline"),
            flag("p", "patch"),
            flag("s", "stat"),
            flag("w", "ignore-all-space"),
        ]),
        opt(arg(target_commit())),
    ])
}

fn merge() -> Node {
    seq([
        Emit("merge"),
        arg(or([
            flag("a", "abort"),
            flag("c", "continue"),
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
            flag("4", "ipv4"),
            flag("6", "ipv6"),
            flag("d", "dry-run"),
            flag("ff", "force"),
            flag("f", "force-with-lease"),
            flag("th", "thin"),
            flag("nth", "no-thin"),
            flag("t", "tags"),
            flag("nt", "no-tags"),
            flag("u", "set-upstream"),
            flag("v", "verify"),
            flag("nv", "no-verify"),
        ]),
        opt(arg(seq([remote(), opt(arg(target_branch()))]))),
    ])
}

fn pull() -> Node {
    seq([
        Emit("pull"),
        argset([
            flag("4", "ipv4"),
            flag("6", "ipv6"),
            flag("a", "all"),
            flag("d", "dry-run"),
            flag("ffo", "ff-only"),
            flag("ff", "ff"),
            flag("nff", "no-ff"),
            flag("f", "force"),
            flag("p", "prune"),
            flag("r", "rebase"),
            flag("nr", "no-rebase"),
            flag("t", "tags"),
            flag("nt", "no-tags"),
            flag("v", "verify"),
            flag("nv", "no-verify"),
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
                    param("eu", "expire-unreachable", reflog_expire_param()),
                    param("e", "expire", reflog_expire_param()),
                    flag("r", "rewrite"),
                    flag("sf", "stale-fix"),
                    flag("sw", "single-worktree"),
                    flag("u", "updateref"),
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
            flag("h", "hard"),
            flag("k", "keep"),
            flag("m", "merge"),
            flag("r", "recurse-submodules"),
            flag("s", "soft"),
        ]))),
        argset([flag("nr", "no-refresh")]),
        opt(arg(target_commit())),
    ])
}

fn switch() -> Node {
    seq([
        Emit("switch"),
        argset([
            flag("C", "force-create"),
            flag("c", "create"),
            flag("d", "detach"),
            flag("fc", "force-create"),
            flag("f", "force"),
            flag("iow", "ignore-other-worktrees"),
            flag("ng", "no-guess"),
            t_track(),
        ]),
        opt(arg(target_branch())),
    ])
}

fn tag() -> Node {
    seq([
        Emit("tag"),
        argset([
            flag("a", "annotate"),
            flag("d", "delete"),
            flag("e", "edit"),
            flag("f", "force"),
            flag("ic", "ignore-case"),
            flag("l", "list"),
            param_opt("mr", "merged", target_commit()),
            param_opt("nm", "no-merged", target_commit()),
            m_message(),
            flag("ns", "no-sign"),
            flag("oe", "omit-empty"),
            flag("s", "sign"),
            flag("v", "verify"),
        ]),
    ])
}

fn restore() -> Node {
    seq([
        Emit("restore"),
        argset([
            flag("i", "staged"), // i for index
            flag("m", "merge"),
            ours_theirs(),
            flag("p", "patch"),
            recurse_submodules(),
            param("s", "source", target_commit()),
            flag("w", "worktree"),
        ]),
    ])
}

fn status() -> Node {
    seq([
        Emit("status"),
        argset([
            flag("a", "ahead-behind"),
            flag("na", "no-ahead-behind"),
            param("fr", "find-renames", Number),
            param(
                "i",
                "ignored",
                opt(or([
                    word("no", "no"),
                    word("t", "traditional"),
                    word("m", "matching"),
                ])),
            ),
            flag("l", "long"),
            flag("r", "renames"),
            flag("nr", "no-renames"),
            flag("s", "short"),
            param(
                "u",
                "untracked-files",
                opt(or([
                    word("no", "no"),
                    word("n", "normal"),
                    word("a", "all"),
                ])),
            ),
            flag("z", "show-stash"),
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
                    flag("d", "detach"),
                    flag("f", "force"),
                    flag("l", "lock"),
                    flag("nc", "no-checkout"),
                    flag("ng", "no-guess-remote"),
                    flag("nrp", "no-relative-paths"),
                    flag("nt", "no-track"),
                    flag("o", "orphan"),
                ]),
            ]),
            seq([word("l", "lock"), argset([])]),
            seq([word("m", "move"), argset([flag("f", "force")])]),
            seq([word("p", "prune"), argset([flag("d", "dry-run")])]),
            seq([word("R", "repair"), argset([])]),
            seq([word("r", "remove"), argset([flag("f", "force")])]),
            seq([word("u", "unlock"), argset([])]),
            seq([word("v", "list"), argset([])]),
        ])),
    ])
}

fn clean() -> Node {
    seq([
        Emit("clean"),
        argset([
            f("d", "d"),
            flag("d", "dry-run"), // TODO
            flag("f", "force"),
            flag("i", "interactive"),
            f("xx", "xX"),
            f("x", "x"),
            f("X", "X"),
        ]),
    ])
}

fn stash() -> Node {
    seq([
        Emit("stash"),
        opt(arg(or([
            seq([word("a", "apply"), argset([])]),
            seq([word("b", "branch"), argset([])]),
            seq([word("c", "clear"), argset([])]),
            seq([word("d", "drop"), argset([])]),
            seq([word("h", "show"), argset([])]),
            seq([word("l", "list"), argset([])]),
            seq([word("m", "create"), argset([])]),
            seq([word("o", "pop"), argset([])]),
            seq([
                word("p", "push"),
                argset([
                    flag("a", "all"),
                    flag("p", "patch"),
                    flag("s", "staged"),
                    m_message(),
                ]),
            ]),
            seq([
                word("s", "save"),
                argset([flag("a", "all"), flag("p", "patch"), flag("s", "staged")]),
            ]),
            seq([word("t", "store"), argset([])]),
        ]))),
    ])
}

// Helpers

fn target_branch() -> Node {
    or([
        map_custom("c", current_branch),
        word("h", "HEAD"),
        map_custom("m", main_branch),
        map_custom("o", main_remote_head),
        map_custom("u", current_upstream),
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

fn m_message() -> Node {
    param("m", "message", seq([Emit("\""), Emit(CURSOR), Emit("\"")]))
}

fn t_track() -> Node {
    or([
        param(
            "t",
            "track",
            opt(or([word("d", "direct"), word("i", "indirect")])),
        ),
        flag("nt", "no-track"),
    ])
}

fn f_pretty() -> Node {
    param_opt(
        "f",
        "pretty",
        or([
            word("e", "email"),
            word("ff", "fuller"),
            word("f", "full"),
            word("m", "medium"),
            word("o", "oneline"),
            word("rf", "reference"),
            word("r", "raw"),
            word("s", "short"),
            word("_", "format:%"),
            word("t_", "tformat:%"),
        ]),
    )
}

fn reflog_expire_param() -> Node {
    or([word("a", "all"), word("n", "never")])
}

fn ours_theirs() -> Node {
    or([flag("os", "ours"), flag("ts", "theirs")])
}
