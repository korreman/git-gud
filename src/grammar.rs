use crate::helpers::*;
use crate::tree::*;

/// Generate the grammar for all commands.
pub fn ast() -> Node {
    // names we might want to avoid:
    // gh: github cli
    // go: golang compiler
    // gdb: GNU debugger
    // gz: gzip
    or([
        ("a", add()),
        ("bl", blame()),
        ("b", branch()),
        ("cat", cat_file()),
        ("c", commit()),
        ("d", diff()),
        ("e", rebase()),
        ("fer", for_each_ref()),
        ("fa", fetch_all()),
        ("fm", fetch_multiple()),
        ("f", fetch()),
        // ("g", checkout()),
        // // h
        // ("i", init()),
        // // j
        // ("k", clone()),
        // ("l", log()),
        // ("m", merge()),
        // // n
        // // o
        // ("p", push()),
        // ("q", status()), // query
        // ("rl", reflog()),
        // ("rp", rev_parse()),
        // ("r", reset()),
        // ("s", switch()),
        // ("t", tag()),
        // ("u", restore()), // undo
        // ("v", show()),
        // ("w", worktree()),
        // ("x", clean()),
        // ("y", pull()),  // yank
        // ("z", stash()), // marks
    ])
}

fn add() -> Node {
    seq([
        Emit("add"),
        argset([
            ("d", flag("dry-run")),
            ("f", flag("force")),
            ("i", flag("interactive")),
            ("s", flag("sparse")),
            ("N", flag("intent-to-add")),
            ("r", flag("refresh")),
            ("u", flag("update")),
            ("p", flag("patch")),
            ("-a", flag("no-all")),
            ("", or_opt([("a", flag("all")), (EOL, flag("all"))])),
        ]),
        separator(),
        or_opt([(".", Emit(".")), ("/", Emit(":/"))]),
    ])
}

fn blame() -> Node {
    seq([
        Emit("blame"),
        argset([
            ("1", flag("first-parent")),
            ("l", seq([f("L"), Emit(" "), cursor()])),
            ("n", flag("show-number")),
            ("s", f("s")),
            ("t", f("t")),
            ("w", f("w")),
        ]),
    ])
}

fn branch() -> Node {
    seq([
        Emit("branch"),
        argset([
            ("a", flag("all")),
            ("c", flag("copy")),
            ("d", flag("delete")),
            ("f", flag("force")),
            ("mg", param_opt_or("merged", c_h_m_o_u_target_rev(), false)),
            (
                "-mg",
                param_opt_or("no-merged", c_h_m_o_u_target_rev(), false),
            ),
            ("m", flag("move")),
            ("r", flag("remotes")),
            // t_track(),
            (
                "u",
                param_or("set-upstream-to", c_h_m_o_u_target_rev(), false),
            ),
            ("v", flag("verbose")),
            ("v", flag("verbose")),
        ]),
    ])
}

fn cat_file() -> Node {
    seq([
        Emit("cat-file"),
        or_prefix_fallback(
            " ",
            [
                // flags
                ("p", f("p")),  // pretty-print
                ("e", f("e")),  // check if exists
                ("tp", f("t")), // query type of object
                ("s", f("s")),  // query size of object
                // object types
                ("b", Emit("blob")),
                ("tg", Emit("tag")),
                ("tr", Emit("tree")),
                ("c", Emit("commit")),
            ],
            false,
            Fail,
        ),
    ])
}

fn commit() -> Node {
    seq([
        Emit("commit"),
        argset([
            ("a", flag("amend")),
            (
                "C",
                param_or("reuse-message", c_h_m_o_u_target_rev(), false),
            ),
            (
                "c",
                param_or("reedit-message", c_h_m_o_u_target_rev(), false),
            ),
            ("d", flag("dry-run")),
            ("e", flag("edit")),
            ("n", flag("no-edit")),
            (
                "f",
                param_or(
                    "fixup",
                    [],
                    // seq([
                    //     opt(set([word("a", "amend:"), word("r", "reword:")])),
                    //     or([c_h_m_o_u_target_rev(), cursor()]),
                    // ]),
                    false,
                ),
            ),
            // param_opt("g", "gpg-sign", fail()),
            ("-g", flag("no-gpg-sign")),
            ("i", flag("include")),
            m_message(),
            ("o", flag("only")),
            ("sq", param_or("squash", c_h_m_o_u_target_rev(), false)),
            ("q", flag("status")),
            ("-q", flag("no-status")),
            ("s", flag("signoff")),
            ("v", flag("verify")),
            ("-v", flag("no-verify")),
        ]),
    ])
}

fn diff() -> Node {
    seq([
        Emit("diff"),
        argset([
            ("da", diff_algorithm()),
            ("ih", flag("indent-heuristic")),
            ("-ih", flag("no-indent-heuristic")),
            ("-i", flag("no-index")),
            ("mb", flag("merge-base")),
            ("p", flag("patience")),
            ("r", flag("raw")),
            ("ss", flag("shortstat")),
            ("s", flag("stat")),
            ("c", param_or("unified", [], true)),
            ("ww", flag("ignore-all-space")),
            ("w", flag("ignore-space-change")),
        ]),
        separator(),
        or_prefix_fallback(" ", c_h_m_o_u_target_rev(), false, Noop),
        or_prefix_fallback(" ", c_h_m_o_u_target_rev(), false, Noop),
    ])
}

fn rebase() -> Node {
    seq([
        Emit("rebase"),
        seq([or_prefix_fallback(
            " ",
            [
                ("a", flag("abort")),
                ("c", flag("continue")),
                ("e", flag("edit-todo")),
                ("h", flag("show-current-patch")),
                ("q", flag("quit")),
                ("s", flag("skip")),
            ],
            false,
            seq([
                argset([
                    ("da", diff_algorithm()),
                    ("i", flag("interactive")),
                    ("r", flag("root")),
                    ("-f", flag("no-ff")),
                    ("s", flag("stat")),
                    ("-s", flag("no-stat")),
                    ("ur", flag("update-refs")),
                    ("-ur", flag("no-update-refs")),
                    ("v", flag("verify")),
                    ("-v", flag("no-verify")),
                ]),
                separator(),
                or_prefix_fallback(" ", c_h_m_o_u_target_rev(), false, Noop),
            ]),
        )]),
    ])
}

fn fetch() -> Node {
    seq([
        Emit("fetch"),
        fetch_args(),
        separator(),
        or_prefix_fallback(
            " ",
            c_o_target_remote(),
            false,
            or_prefix_fallback(" ", c_h_m_o_u_target_branch(), false, Noop),
        ),
    ])
}

fn fetch_multiple() -> Node {
    seq([
        Emit("fetch --multiple"),
        fetch_args(),
        separator(),
        or_prefix_fallback(" ", c_o_target_remote(), false, Noop),
    ])
}

fn fetch_all() -> Node {
    seq([Emit("fetch --all"), fetch_args()])
}

fn fetch_args() -> Node {
    argset([
        ("4", flag("ipv4")),
        ("6", flag("ipv6")),
        ("A", flag("append")),
        ("-a", flag("no-all")),
        ("d", flag("dry-run")),
        ("f", flag("force")),
        ("k", flag("keep")),
        ("p", flag("prune")),
        ("t", flag("tags")),
        ("-t", flag("no-tags")),
    ])
}

fn for_each_ref() -> Node {
    seq([
        Emit("for-each-ref"),
        argset([
            ("c", flag("contains")),     // TODO: object param
            ("nc", flag("no-contains")), // TODO: object param
            ("f", param("format", for_each_ref_fields())),
            ("irr", param_or("include-root-refs", [], false)),
            ("i", flag("ignore-case")),
            ("m", flag("merged")),     // TODO: object param
            ("nm", flag("no-merged")), // TODO: object param
            ("n", param_or("count", [], true)),
            ("oe", flag("omit-empty")),
            ("e", param_or("exclude", [], false)),
            ("s", param_or("sort", for_each_ref_field_name(), false)),
        ]),
    ])
}

fn for_each_ref_fields() -> Node {
    seq([
        Emit("\""),
        argset(for_each_ref_field_name().map(|(s, f)| (s, seq([Emit("%("), f, Emit(")")])))),
        Emit("\""),
    ])
}

fn for_each_ref_field_name() -> [(Str, Node); 28] {
    [
        // TODO: stuff that targets <commitish> could be implemented somewhere around here
        ("al", Emit("align:left")),
        ("am", Emit("align:middle")),
        ("ar", Emit("align:right")),
        ("cb", Emit("contents:body")),
        ("csb", Emit("contents:subject")),
        ("cs", Emit("contents:size")),
        ("csg", Emit("contents:signature")),
        ("cl", Emit("contents:lines")),
        ("d", Emit("describe")),
        ("db", Emit("deltabase")),
        ("h", Emit("HEAD")),
        ("on", Emit("objectname")),
        ("os", Emit("objectsize")),
        ("ot", Emit("objecttype")),
        ("p", Emit("push")),
        ("rn", Emit("refname")),
        ("rns", Emit("refname:short")),
        ("rs", Emit("raw:size")),
        ("sf", Emit("signature:fingerprint")),
        ("sg", Emit("signature:grade")),
        ("sk", Emit("signature:key")),
        ("spkf", Emit("signature:primarykeyfingerprint")),
        ("sr", Emit("symref")),
        ("s", Emit("signature")),
        ("ss", Emit("signature:signer")),
        ("stl", Emit("signature:trustlevel")),
        ("u", Emit("upstream")),
        ("wp", Emit("worktreepath")),
    ]
}

fn checkout() -> Node {
    seq([
        Emit("checkout"),
        argset([
            ("B", f("B")),
            ("bb", f("B")),
            ("b", f("b")),
            ("l", f("l")),
            ("d", flag("detach")),
            ("f", flag("force")),
            ("g", flag("guess")),
            ("-g", flag("no-guess")),
            ("mg", flag("merge")),
            ("-o", flag("no-overlay")),
            // os_ts_ours_theirs(),
            ("p", flag("patch")),
            // t_track(),
        ]),
        // separator(),
        // opt(arg(c_h_m_o_u_target_rev())),
    ])
}

/*
// TODO: Should have most of the same options as diff command
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
            param("ib", "initial-branch", cursor()),
            param("t", "template", cursor()),
        ]),
    ])
}

fn clone() -> Node {
    seq([
        Emit("clone"),
        argset([
            flag("0", "bare"),
            flag("1", "single-branch"),
            param("b", "branch", cursor()),
            param("d", "depth", Number),
            param("d", "dissociate", cursor()),
            flag("h", "shared"),
            param("j", "jobs", Number),
            flag("l", "local"),
            flag("m", "mirror"),
            flag("-g", "no-checkout"),
            flag("-hl", "no-hardlinks"),
            param("o", "origin", cursor()),
            flag("s", "sparse"),
            flag("t", "tags"),
            flag("-t", "no-tags"),
            param("rf", "reference", cursor()),
            param("rv", "revision", cursor()),
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
            flag("mg", "merges"),
            param("n", "max-count", Number),
            flag("o", "oneline"),
            flag("p", "patch"),
            flag("r", "reverse"),
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
            seq([
                in_merge(),
                or([flag("a", "abort"), flag("c", "continue"), flag("q", "quit")]),
            ]),
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

fn rev_parse() -> Node {
    seq([
        Emit("rev-parse"),
        argset([]),
        separator(),
        opt(arg(c_h_m_o_u_target_rev())),
    ])
}

fn reset() -> Node {
    seq([
        Emit("reset"),
        argset([
            or([
                flag("h", "hard"),
                flag("k", "keep"),
                flag("mg", "merge"),
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
            param_opt("-mg", "no-merged", c_h_m_o_u_target_rev()),
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
            flag("mg", "merge"),
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
            seq([word("v", "show"), argset([])]),
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
*/

// Helpers
fn separator() -> Node {
    or_opt([(",", Noop), ("/", Noop)])
}

fn diff_algorithm() -> Node {
    param_or(
        "diff-algorithm",
        [
            ("h", Emit("histogram")),
            ("m", Emit("minimal")),
            ("y", Emit("myers")),
            ("p", Emit("patience")),
        ],
        false,
    )
}

fn c_h_m_o_u_target_branch() -> [(Str, Node); 5] {
    [
        ("c", Custom(current_branch, "CURRENT BRANCH")),
        ("h", Emit("HEAD")),
        ("m", Custom(main_branch, "MAIN BRANCH")),
        ("o", Custom(main_remote_head, "MAIN REMOTE HEAD")),
        ("u", Custom(current_upstream, "CURRENT UPSTREAM")),
    ]
}

fn c_h_m_o_u_target_rev() -> [(Str, Node); 7] {
    [
        ("c", Custom(current_branch, "CURRENT BRANCH")),
        ("h", Emit("HEAD")),
        ("m", Custom(main_branch, "MAIN BRANCH")),
        ("o", Custom(main_remote_head, "MAIN REMOTE HEAD")),
        ("u", Custom(current_upstream, "CURRENT UPSTREAM")),
        ("-", seq([Emit("HEAD~"), or_fallback([], true, Noop)])),
        (
            "@",
            seq([Emit("HEAD@{"), or_fallback([], true, Fail), Emit("}")]),
        ),
    ]
}

fn c_o_target_remote() -> [(Str, Node); 2] {
    [
        ("c", Custom(current_remote, "CURRENT REMOTE")),
        ("o", Custom(main_remote, "MAIN REMOTE")),
    ]
}

/*
fn rs_recurse_submodules() -> Node {
    flag("rs", "recurse-submodules")
}

*/
fn m_message() -> (Str, Node) {
    ("m", seq([Emit("--message="), custom_quoted()]))
}

/*
fn t_track() -> Node {
    // or([
    //     param(
    //         "t",
    //         "track",
    //         opt(or([word("d", "direct"), word("i", "indirect")])),
    //     ),
    //     flag("-t", "no-track"),
    // ])
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
    or([word("a", "all"), word("n", "never"), map("_", cursor())])
}

fn os_ts_ours_theirs() -> Node {
    or([flag("os", "ours"), flag("ts", "theirs")])
}

fn in_rebase() -> Node {
    Custom(crate::helpers::in_rebase, "MUST BE IN REBASE")
}

fn in_revert() -> Node {
    Custom(crate::helpers::in_revert, "MUST BE IN REVERT")
}

fn in_merge() -> Node {
    Custom(crate::helpers::in_merge, "MUST BE IN MERGE")
}

fn in_cherry_pick() -> Node {
    Custom(crate::helpers::in_cherry_pick, "MUST BE IN CHERRY-PICK")
}

*/
fn custom_quoted() -> Node {
    seq([Emit("\""), cursor(), Emit("\"")])
}

/*
fn custom_quoted_single() -> Node {
    seq([Emit("'"), cursor(), Emit("'")])
}
*/
