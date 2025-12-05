use crate::helpers::*;
use crate::tree::*;

// Helper ASTs

const CURSOR: &'static str = "%";

fn target_branch() -> Node {
    or([
        map_custom("h", current_branch),
        map_custom("u", current_upstream),
        map_custom("m", main_branch),
        map_custom("o", main_remote_head),
    ])
}

fn remote() -> Node {
    or([
        map_custom("h", current_remote),
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

fn commit_or_cursor() -> Node {
    or([target_commit(), word("=", CURSOR), Emit(CURSOR)])
}

fn message() -> Node {
    param("m", "message", Emit(CURSOR))
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
            word("=", "format:%"),
            word("=t", "tformat:%"),
        ]),
    )
}

fn reflog_expire_param() -> Node {
    or([
        word("a", "all"),
        word("n", "never"),
        word("=", CURSOR),
        Emit(CURSOR),
    ])
}

/// Generate the grammar for all commands.
pub fn ast() -> Node {
    or([
        add(),
        blame(),
        branch(),
        commit(),
        diff(),
        rebase(),
        fetch(),
        checkout(),
        show(),
        init(),
        clone(),
        log(),
        merge(),
        push(),
        pull(),
        reflog(),
        reset(),
        switch(),
        tag(),
        restore(),
        status(),
        worktree(),
        clean(),
        stash(),
    ])
}

fn add() -> Node {
    seq([
        word("a", "add"),
        prefix_set(
            " ",
            [
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
            ],
        ),
        opt(or([word(".", "."), word("/", ":/")])),
    ])
}

fn blame() -> Node {
    seq([
        word("bl", "blame"),
        prefix_set(
            " ",
            [
                f("t", "t"),
                f("w", "w"),
                f("s", "s"),
                flag("1", "first-parent"),
                seq([f("l", "L"), Emit(" "), Emit(CURSOR)]),
                flag("n", "show-number"),
            ],
        ),
    ])
}

fn branch() -> Node {
    seq([
        word("b", "branch"),
        prefix_set(
            " ",
            [
                flag("f", "force"),
                flag("d", "delete"),
                param("me", "merged", commit_or_cursor()),
                param("nme", "no-merged", commit_or_cursor()),
                flag("m", "move"),
                flag("c", "copy"),
                flag("r", "remotes"),
                flag("a", "all"),
                f("vv", "vv"),
                flag("v", "verbose"),
                flag("q", "quiet"),
                param("u", "set-upstream-to", commit_or_cursor()),
                track(),
            ],
        ),
    ])
}

fn commit() -> Node {
    seq([
        word("c", "commit"),
        prefix_set(
            " ",
            [
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
                param_opt("g", "gpg-sign", word("=", CURSOR)),
                param(
                    "f",
                    "fixup",
                    seq([
                        opt(set([word("a", "amend:"), word("r", "reword:")])),
                        commit_or_cursor(),
                    ]),
                ),
                param("q", "squash", commit_or_cursor()),
                param("c", "reedit-message", commit_or_cursor()),
                param("C", "reuse-message", commit_or_cursor()),
                message(),
            ],
        ),
    ])
}

fn diff() -> Node {
    seq([
        word("d", "diff"),
        prefix_set(
            " ",
            [
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
                param("u", "unified", number_or_zero()),
            ],
        ),
    ])
}

fn rebase() -> Node {
    seq([
        word("e", "rebase"),
        prefix_or(
            " ",
            [
                flag("a", "abort"),
                flag("c", "continue"),
                flag("e", "edit-todo"),
                flag("h", "show"),
                flag("q", "quit"),
                flag("s", "skip"),
                seq([
                    prefix_set(
                        " ",
                        [
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
                        ],
                    ),
                    opt(target_commit()),
                ]),
            ],
        ),
    ])
}

fn fetch() -> Node {
    seq([
        word("f", "fetch"),
        prefix_set(
            " ",
            [
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
            ],
        ),
    ])
}

fn checkout() -> Node {
    seq([
        word("g", "checkout"),
        prefix_set(
            " ",
            [
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
            ],
        ),
        opt(target_commit()),
    ])
}

fn show() -> Node {
    seq([
        word("h", "show"),
        prefix_set(
            " ",
            [
                flag("o", "oneline"),
                flag("nn", "no-notes"),
                flag("a", "abbrev-commit"),
                flag("s", "no-patch"),
                f("m", "m"),
                pretty(),
            ],
        ),
        opt(target_commit()),
    ])
}

fn init() -> Node {
    seq([
        word("i", "init"),
        prefix_set(" ", [flag("b", "bare")]),
        opt(target_commit()),
    ])
}

fn clone() -> Node {
    seq([
        word("k", "clone"),
        prefix_set(
            " ",
            [
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
            ],
        ),
        opt(target_commit()),
    ])
}

fn log() -> Node {
    seq([
        word("l", "log"),
        prefix_set(
            " ",
            [
                flag("ac", "abbrev-commit"),
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
                flag("me", "merges"),
                flag("a", "all"),
                flag("g", "graph"),
                flag("p", "patch"),
                flag("b", "ignore-space-change"),
                flag("w", "ignore-all-space"),
                param("m", "max-count", Number),
                pretty(),
            ],
        ),
        opt(target_commit()),
    ])
}

fn merge() -> Node {
    seq([
        word("m", "merge"),
        prefix_or(
            " ",
            [
                flag("c", "continue"),
                flag("a", "abort"),
                flag("q", "quit"),
                seq([prefix_set(" ", []), opt(target_commit())]),
            ],
        ),
        opt(target_commit()),
    ])
}

fn push() -> Node {
    seq([
        word("q", "push"),
        prefix_set(
            " ",
            [
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
            ],
        ),
        opt(seq([remote(), opt(target_branch())])),
    ])
}

fn pull() -> Node {
    seq([
        word("q", "pull"),
        prefix_set(
            " ",
            [
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
            ],
        ),
    ])
}

fn reflog() -> Node {
    seq([
        word("rl", "reflog"),
        prefix_or(
            " ",
            [
                seq([
                    word("s", "show"), // TODO: add all log expansions here
                    prefix_set(
                        " ",
                        [
                            flag("a", "all"),
                            flag("d", "dry-run"),
                            flag("r", "rewrite"),
                            flag("sf", "stale-fix"),
                            flag("sw", "single-worktree"),
                            flag("u", "updateref"),
                            param("e", "expire", reflog_expire_param()),
                            param("eu", "expire-unreachable", reflog_expire_param()),
                        ],
                    ),
                ]),
                word("l", "list"),
                word("e", "exists"),
                word("x", "expire"),
                seq([
                    word("d", "delete"),
                    prefix_set(
                        " ",
                        [
                            flag("d", "dry-run"),
                            flag("r", "rewrite"),
                            flag("u", "updateref"),
                        ],
                    ),
                ]),
                seq([
                    word("D", "drop"),
                    opt(seq([
                        prefix(" ", flag("a", "all")),
                        opt(flag("sw", "single-worktree")),
                    ])),
                ]),
            ],
        ),
    ])
}

fn reset() -> Node {
    seq([
        word("r", "reset"),
        opt(prefix_or(
            " ",
            [
                flag("s", "soft"),
                flag("h", "hard"),
                flag("m", "merge"),
                flag("k", "keep"),
                flag("r", "recurse-submodules"),
            ],
        )),
        prefix_set(" ", [flag("q", "quiet"), flag("nr", "no-refresh")]),
        opt(prefix(" ", target_commit())),
    ])
}

fn switch() -> Node {
    seq([
        word("s", "switch"),
        prefix_set(
            " ",
            [
                flag("fc", "force-create"),
                flag("f", "force"),
                flag("C", "force-create"),
                flag("c", "create"),
                flag("d", "detach"),
                flag("ng", "no-guess"),
                flag("iow", "ignore-other-worktrees"),
                track(),
            ],
        ),
        opt(prefix(" ", target_branch())),
    ])
}

fn tag() -> Node {
    seq([
        word("t", "tag"),
        prefix_set(
            " ",
            [
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
            ],
        ),
    ])
}

fn restore() -> Node {
    seq([
        word("u", "restore"),
        prefix_set(
            " ",
            [
                flag("p", "patch"),
                flag("w", "worktree"),
                flag("i", "staged"), // i for index
                flag("o", "ours"),
                flag("t", "theirs"),
                flag("m", "merge"),
                param("s", "source", Emit(CURSOR)),
                recurse_submodules(),
            ],
        ),
    ])
}

fn status() -> Node {
    seq([
        word("v", "status"),
        prefix_set(
            " ",
            [
                flag("s", "short"),
                flag("l", "long"),
                flag("z", "show-stash"),
                flag("v", "verbose"),
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
            ],
        ),
    ])
}

fn worktree() -> Node {
    seq([
        word("w", "worktree"),
        prefix_or(
            " ",
            [
                seq([
                    word("a", "add"),
                    prefix_set(
                        " ",
                        [
                            flag("f", "force"),
                            flag("d", "detach"),
                            flag("nc", "no-checkout"),
                            flag("ng", "no-guess-remote"),
                            flag("nrp", "no-relative-paths"),
                            flag("nt", "no-track"),
                            flag("l", "lock"),
                            flag("o", "orphan"),
                            flag("q", "quiet"),
                        ],
                    ),
                ]),
                seq([word("v", "list"), prefix_set(" ", [flag("v", "verbose")])]),
                seq([word("m", "move"), prefix_set(" ", [flag("f", "force")])]),
                seq([
                    word("p", "prune"),
                    prefix_set(" ", [flag("d", "dry-run"), flag("v", "verbose")]),
                ]),
                seq([word("r", "remove"), prefix_set(" ", [flag("f", "force")])]),
                seq([word("R", "repair"), prefix_set(" ", [])]),
                seq([word("l", "lock"), prefix_set(" ", [])]),
                seq([word("u", "unlock"), prefix_set(" ", [])]),
            ],
        ),
    ])
}

fn clean() -> Node {
    seq([
        word("x", "clean"),
        prefix_set(
            " ",
            [
                f("d", "d"),
                f("xx", "xX"),
                f("x", "x"),
                f("X", "X"),
                flag("f", "force"),
                flag("i", "interactive"),
                flag("d", "dry-run"),
                flag("q", "quiet"),
            ],
        ),
    ])
}

fn stash() -> Node {
    seq([
        word("z", "stash"),
        opt(prefix_or(
            " ",
            [
                seq([
                    word("p", "push"),
                    prefix_set(
                        " ",
                        [
                            flag("a", "all"),
                            flag("p", "patch"),
                            flag("s", "staged"),
                            message(),
                        ],
                    ),
                ]),
                seq([word("o", "pop"), prefix_set(" ", [])]),
                seq([
                    word("s", "save"),
                    prefix_set(
                        " ",
                        [
                            flag("a", "all"),
                            flag("p", "patch"),
                            flag("s", "staged"),
                            flag("q", "quiet"),
                        ],
                    ),
                ]),
                seq([word("l", "list"), prefix_set(" ", [])]),
                seq([word("h", "show"), prefix_set(" ", [])]),
                seq([word("d", "drop"), prefix_set(" ", [])]),
                seq([word("a", "apply"), prefix_set(" ", [])]),
                seq([word("b", "branch"), prefix_set(" ", [])]),
                seq([word("c", "clear"), prefix_set(" ", [])]),
                seq([word("m", "create"), prefix_set(" ", [])]),
                seq([word("t", "store"), prefix_set(" ", [])]),
            ],
        )),
    ])
}
