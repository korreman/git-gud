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
        // h
        map("i", init()),
        // j
        map("k", clone()),
        map("l", log()),
        map("m", merge()),
        // n
        // o
        map("p", push()),
        map("q", status()), // query
        map("rl", reflog()),
        map("r", reset()),
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
            flag("a", "all"),
            flag("-a", "no-all"),
            flag("d", "dry-run"),
            flag("f", "force"),
            flag("i", "interactive"),
            flag("s", "sparse"),
            flag("N", "intent-to-add"),
            flag("r", "refresh"),
            flag("u", "update"),
            flag("p", "patch"),
        ]),
        separator(),
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
            param_opt("mg", "merged", c_h_m_o_u_target_rev()),
            param_opt("-m", "no-merged", c_h_m_o_u_target_rev()),
            flag("m", "move"),
            flag("r", "remotes"),
            t_track(),
            param("u", "set-upstream-to", c_h_m_o_u_target_rev()),
            flag("v", "verbose"),
            flag("v", "verbose"),
        ]),
    ])
}

fn commit() -> Node {
    seq([
        Emit("commit"),
        argset([
            flag("a", "amend"),
            param("C", "reuse-message", c_h_m_o_u_target_rev()),
            param("c", "reedit-message", c_h_m_o_u_target_rev()),
            flag("d", "dry-run"),
            flag("e", "edit"),
            flag("-e", "no-edit"),
            param(
                "f",
                "fixup",
                seq([
                    opt(set([word("a", "amend:"), word("r", "reword:")])),
                    or([c_h_m_o_u_target_rev(), Emit(CURSOR)]),
                ]),
            ),
            param_opt("g", "gpg-sign", fail()),
            flag("-g", "no-gpg-sign"),
            flag("i", "include"),
            m_message(),
            flag("o", "only"),
            param("sq", "squash", c_h_m_o_u_target_rev()),
            flag("q", "status"),
            flag("-q", "no-status"),
            flag("s", "signoff"),
            flag("v", "verify"),
            flag("-v", "no-verify"),
        ]),
    ])
}

fn diff() -> Node {
    seq([
        Emit("diff"),
        argset([
            da_diff_algorithm(),
            flag("ih", "indent-heuristic"),
            flag("-ih", "no-indent-heuristic"),
            flag("-i", "no-index"),
            flag("p", "patience"),
            flag("r", "raw"),
            flag("ss", "shortstat"),
            flag("s", "stat"),
            param("u", "unified", number_or_zero()),
            flag("ww", "ignore-all-space"),
            flag("w", "ignore-space-change"),
        ]),
        separator(),
        opt(arg(c_h_m_o_u_target_rev())),
        opt(arg(c_h_m_o_u_target_rev())),
    ])
}

// TODO: this one could maybe be contextual,
// so subcommands only work if we're in a rebase,
// and targets only work if we're out of the rebase again
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
                    da_diff_algorithm(),
                    flag("i", "interactive"),
                    flag("r", "root"),
                    flag("-f", "no-ff"),
                    flag("s", "stat"),
                    flag("-s", "no-stat"),
                    flag("u", "update-refs"),
                    flag("-u", "no-update-refs"),
                    flag("v", "verify"),
                    flag("-v", "no-verify"),
                ]),
                separator(),
                opt(arg(c_h_m_o_u_target_rev())),
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
            flag("-a", "no-all"),
            flag("d", "dry-run"),
            flag("f", "force"),
            flag("k", "keep"),
            flag("m", "multiple"),
            flag("p", "prune"),
            flag("t", "tags"),
            flag("-t", "no-tags"),
        ]),
        separator(),
        opt(seq([c_o_target_remote(), opt(c_h_m_o_u_target_branch())])),
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
            flag("-g", "no-guess"),
            flag("m", "merge"),
            flag("-o", "no-overlay"),
            os_ts_ours_theirs(),
            flag("p", "patch"),
            t_track(),
        ]),
        opt(Eat(",")),
        opt(arg(c_h_m_o_u_target_rev())),
    ])
}

// TODO: Show have most of the same options as diff command
fn show() -> Node {
    seq([
        Emit("show"),
        argset([
            flag("a", "abbrev-commit"),
            flag("w", "ignore-space-change"),
            f_pretty(),
            f("m", "m"),
            flag("-n", "no-notes"),
            flag("-p", "no-patch"),
            flag("o", "oneline"),
            flag("s", "stat"),
        ]),
        separator(),
        opt(arg(c_h_m_o_u_target_rev())),
    ])
}

fn init() -> Node {
    seq([
        Emit("init"),
        argset([
            flag("b", "bare"),
            param(
                "o",
                "object-format",
                or([word("1", "sha1"), word("2", "sha256")]),
            ),
            param(
                "r",
                "ref-format",
                or([word("f", "files"), word("t", "reftable")]),
            ),
            param("ib", "initial-branch", Emit(CURSOR)),
            param("t", "template", Emit(CURSOR)),
        ]),
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
            flag("-g", "no-checkout"),
            flag("-hl", "no-hardlinks"),
            param("o", "origin", Emit(CURSOR)),
            flag("s", "sparse"),
            flag("t", "tags"),
            flag("-t", "no-tags"),
            param("rf", "reference", Emit(CURSOR)),
            param("rv", "revision", Emit(CURSOR)),
        ]),
        separator(),
        opt(arg(c_h_m_o_u_target_rev())),
    ])
}

fn log() -> Node {
    seq([
        Emit("log"),
        argset([
            flag("1", "first-parent"),
            flag("ac", "abbrev-commit"),
            flag("-ac", "no-abbrev-commit"),
            flag("a", "all"),
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
            flag("-d", "no-decorate"),
            flag("F", "follow"),
            f_pretty(),
            flag("g", "graph"),
            flag("m", "merges"),
            param("n", "max-count", Number),
            flag("o", "oneline"),
            flag("p", "patch"),
            flag("s", "stat"),
            flag("ww", "ignore-all-space"),
            flag("w", "ignore-space-change"),
        ]),
        separator(),
        opt(arg(c_h_m_o_u_target_rev())),
    ])
}

fn merge() -> Node {
    seq([
        Emit("merge"),
        arg(or([
            flag("a", "abort"),
            flag("c", "continue"),
            flag("q", "quit"),
            seq([argset([]), separator(), opt(c_h_m_o_u_target_rev())]),
        ])),
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
            flag("-th", "no-thin"),
            flag("t", "tags"),
            flag("-t", "no-tags"),
            flag("u", "set-upstream"),
            flag("v", "verify"),
            flag("-v", "no-verify"),
        ]),
        separator(),
        opt(arg(seq([
            c_o_target_remote(),
            opt(arg(c_h_m_o_u_target_branch())),
        ]))),
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
            flag("-ff", "no-ff"),
            flag("f", "force"),
            flag("p", "prune"),
            flag("r", "rebase"),
            flag("-r", "no-rebase"),
            flag("t", "tags"),
            flag("-t", "no-tags"),
            flag("v", "verify"),
            flag("-v", "no-verify"),
        ]),
    ])
}

fn reflog() -> Node {
    seq([
        Emit("reflog"),
        arg(or([
            seq([
                word("s", "show"), // TODO: add all log expansions here
                argset([]),
            ]),
            word("l", "list"),
            word("e", "exists"),
            seq([
                word("x", "expire"),
                set([
                    flag("a", "all"),
                    flag("d", "dry-run"),
                    param("ee", "expire", a_n_reflog_expire_param()),
                    param("e", "expire-unreachable", a_n_reflog_expire_param()),
                    flag("r", "rewrite"),
                    flag("sf", "stale-fix"),
                    flag("sw", "single-worktree"),
                    flag("u", "updateref"),
                ]),
            ]),
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
        argset([
            or([
                flag("h", "hard"),
                flag("k", "keep"),
                flag("m", "merge"),
                flag("r", "recurse-submodules"),
                flag("s", "soft"),
            ]),
            flag("-r", "no-refresh"),
        ]),
        separator(),
        opt(arg(c_h_m_o_u_target_rev())),
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
            flag("-g", "no-guess"),
            t_track(),
        ]),
        separator(),
        opt(arg(c_h_m_o_u_target_branch())),
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
            param_opt("mg", "merged", c_h_m_o_u_target_rev()),
            param_opt("-m", "no-merged", c_h_m_o_u_target_rev()),
            m_message(),
            flag("-s", "no-sign"),
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
            os_ts_ours_theirs(),
            flag("p", "patch"),
            rs_recurse_submodules(),
            param("s", "source", c_h_m_o_u_target_rev()),
            flag("w", "worktree"),
        ]),
    ])
}

fn status() -> Node {
    seq([
        Emit("status"),
        argset([
            flag("a", "ahead-behind"),
            flag("-a", "no-ahead-behind"),
            param("fr", "find-renames", Number),
            param(
                "i",
                "ignored",
                opt(or([word("t", "traditional"), word("m", "matching")])),
            ),
            param("-i", "ignored", Emit("no")),
            flag("l", "long"),
            flag("r", "renames"),
            flag("-r", "no-renames"),
            flag("s", "short"),
            param(
                "u",
                "untracked-files",
                opt(or([word("n", "normal"), word("a", "all")])),
            ),
            param("nu", "untracked-files", Emit("no")),
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
                    flag("-c", "no-checkout"),
                    flag("-g", "no-guess-remote"),
                    flag("-rp", "no-relative-paths"),
                    flag("-t", "no-track"),
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
            flag("f", "force"),
            flag("i", "interactive"),
            flag("n", "dry-run"), // TODO
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
                    m_message(),
                    flag("p", "patch"),
                    flag("s", "staged"),
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

fn separator() -> Node {
    opt(or([Eat(","), Eat("/")]))
}

fn da_diff_algorithm() -> Node {
    param(
        "da",
        "diff-algorithm",
        or([
            word("h", "histogram"),
            word("m", "minimal"),
            word("y", "myers"),
            word("p", "patience"),
        ]),
    )
}

fn c_h_m_o_u_target_branch() -> Node {
    or([
        map_custom("c", current_branch),
        word("h", "HEAD"),
        map_custom("m", main_branch),
        map_custom("o", main_remote_head),
        map_custom("u", current_upstream),
    ])
}

fn c_o_target_remote() -> Node {
    or([
        map_custom("c", current_remote),
        map_custom("o", main_remote),
    ])
}

fn rs_recurse_submodules() -> Node {
    flag("rs", "recurse-submodules")
}

fn c_h_m_o_u_target_rev() -> Node {
    or([
        c_h_m_o_u_target_branch(),
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
        flag("-t", "no-track"),
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

fn a_n_reflog_expire_param() -> Node {
    or([word("a", "all"), word("n", "never"), map("_", Emit(CURSOR))])
}

fn os_ts_ours_theirs() -> Node {
    or([flag("os", "ours"), flag("ts", "theirs")])
}
