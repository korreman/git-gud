use std::fmt::{self, Display, Write};

use anyhow::Result;
use log::trace;

pub type Str = &'static str;

pub use Node::*;

/// Data structure for constructing grammars.
#[derive(Clone, Debug)]
pub enum Node {
    /// Succeeds without consuming anything at the end of a line.
    Eol,
    /// Consume the given string and produce nothing.
    Eat(Str),
    /// Eat nothing and produce the given string.
    Emit(Str),
    /// Eat a number and produce it.
    Number,
    /// Run a function and produce its output.
    Custom(fn() -> Option<String>, &'static str),
    /// Try to run each child until one succeeds.
    Or(Vec<Node>),
    /// Run every child, fail if any child fails.
    Seq(Vec<Node>),
    /// Repeatedly try running each child, one after the other.
    /// When a child succeeds, it will no longer be tried.
    /// Finishes when none of the remaining children succeed.
    /// The bool specifies whether at least one child must match.
    /// If set to `false`, the node will always succeed, even if no children match.
    Set(Vec<Node>, bool),
}

impl Node {
    /// Normalize the grammar:
    /// - Collapse structures where possible.
    /// - Sort elements such that longer sequences are matched before shorter ones.
    ///   Prevents shadowing.
    pub fn _preprocess(&mut self) -> Result<()> {
        todo!()
    }

    /// Identify and report ambiguities in the grammar.
    ///
    /// For it to be usable and memorable, any grammar needs unambiguous.
    /// That is, it shouldn't be possible for an expression to be interpreted in multiple ways.
    /// I also think that it should fulfill some concept of a local unambiguity property.
    /// The entire output shouldn't be able to depend on the final character.
    fn _find_ambiguities(&self) -> Result<(), Vec<(String, Vec<String>)>> {
        todo!()
    }

    pub fn normalize(self) -> Self {
        let new_self = self.flatten();
        match new_self {
            Seq(mut nodes) => {
                nodes.sort_by(|a, b| match (a, b) {
                    (Eat(_), Eat(_)) => std::cmp::Ordering::Equal,
                    (Eat(_), _) => std::cmp::Ordering::Less,
                    (_, Eat(_)) => std::cmp::Ordering::Greater,
                    _ => std::cmp::Ordering::Equal,
                });
                Seq(nodes.into_iter().map(Self::normalize).collect())
            }
            Or(nodes) => Or(nodes.into_iter().map(Self::normalize).collect()),
            Set(nodes, b) => Set(nodes.into_iter().map(Self::normalize).collect(), b),
            x => x,
        }
    }

    pub fn flatten(self) -> Self {
        match self {
            Seq(nodes) => {
                let mut result = Vec::new();
                for node in nodes {
                    let flattened = node.flatten();
                    match flattened {
                        Seq(mut internal) => result.append(&mut internal),
                        n => result.push(n),
                    }
                }
                Seq(result)
            }
            Or(nodes) => Or(nodes.into_iter().map(Self::flatten).collect()),
            Set(nodes, b) => Set(nodes.into_iter().map(Self::flatten).collect(), b),
            x => x,
        }
    }

    pub fn expand<'a>(&self, input: &'a str, eol: bool, output: &mut String) -> Option<&'a str> {
        match self {
            Node::Eol => {
                if input.is_empty() && eol {
                    trace!("match");
                    Some(input)
                } else {
                    None
                }
            }
            Node::Eat(shorthand) => {
                trace!("eat {shorthand} | {input}");
                let tail = input.strip_prefix(shorthand)?;
                trace!("match | {tail:?}");
                Some(tail)
            }
            Node::Emit(result) => {
                trace!("emit {result:?} | {input}");
                output.push_str(result);
                Some(input)
            }
            Node::Number => {
                trace!("number | {input}");
                let split_idx = input
                    .char_indices()
                    .find_map(|(idx, c)| if c.is_ascii_digit() { None } else { Some(idx) })
                    .unwrap_or(input.len());
                trace!("split_off: {split_idx}");
                if split_idx == 0 {
                    return None;
                }
                let (number, tail) = input.split_at(split_idx);
                trace!("match: {number} | {tail}");
                output.push_str(number);
                Some(tail)
            }
            Node::Custom(func, ..) => {
                trace!("custom func | {input}");
                let expansion = func()?;
                trace!("match: {expansion} | {input}");
                output.push_str(&expansion);
                Some(input)
            }
            Node::Or(nodes) => {
                trace!("or {{..}} | {input}");
                for node in nodes {
                    if let Some(tail) = node.expand(input, eol, output) {
                        trace!("or match | {tail}");
                        return Some(tail);
                    }
                }
                None
            }
            Node::Seq(nodes) => {
                trace!("seq {{..}} | {input}");
                let backtrack_len = output.len();
                let mut input = input;
                for node in nodes {
                    let Some(tail) = node.expand(input, eol, output) else {
                        output.truncate(backtrack_len);
                        return None;
                    };
                    input = tail;
                }
                trace!("seq match | {input}");
                Some(input)
            }
            Node::Set(nodes, one_minimum) => {
                trace!("set {{..}} | {input}");
                let mut input = input;
                let mut parsed = vec![false; nodes.len()];
                'outer: loop {
                    for (idx, node) in nodes.iter().enumerate() {
                        if parsed[idx] {
                            continue;
                        }
                        if let Some(tail) = node.expand(input, eol, output) {
                            parsed[idx] = true;
                            input = tail;
                            continue 'outer;
                        }
                    }
                    break;
                }
                if *one_minimum && !parsed.iter().any(|x| *x) {
                    trace!("set empty, one required");
                    None
                } else {
                    trace!("set match | {input}");
                    Some(input)
                }
            }
        }
    }

    fn fmt_helper(&self, f: &mut fmt::Formatter<'_>, indent: u32) -> fmt::Result {
        match self {
            Eol => f.write_str("<EOL> ⇒ ")?,
            Eat(e) => {
                f.write_str(e)?;
                f.write_str(" ⇒ ")?;
            }
            Emit(e) => f.write_str(e)?,
            Number => f.write_str("<NUMBER>")?,
            Custom(_, desc) => f.write_fmt(format_args!("<{desc}>"))?,
            Seq(nodes) => {
                f.write_char('[')?;
                for node in nodes {
                    node.fmt_helper(f, indent + 1)?;
                }
                f.write_char(']')?;
            }
            Or(nodes) => {
                write_newline_indent(f, indent)?;
                f.write_str("|| [")?;
                for node in nodes {
                    write_newline_indent(f, indent + 1)?;
                    node.fmt_helper(f, indent + 1)?;
                }
                write_newline_indent(f, indent)?;
                f.write_str("]")?;
            }
            Set(nodes, _) => {
                write_newline_indent(f, indent)?;
                f.write_str("⊆ [")?;
                for node in nodes {
                    write_newline_indent(f, indent + 1)?;
                    node.fmt_helper(f, indent + 1)?;
                }
                write_newline_indent(f, indent)?;
                f.write_str("]")?;
            }
        }
        Ok(())
    }
}

fn write_newline_indent(f: &mut fmt::Formatter<'_>, indent: u32) -> fmt::Result {
    f.write_char('\n')?;
    for _ in 0..indent {
        f.write_str("  ")?;
    }
    Ok(())
}

impl Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_helper(f, 0)
    }
}

// Useful combinators
pub fn cursor() -> Node {
    Custom(|| Some("{GIT_GUD_CURSOR}".to_owned()), "CURSOR")
}

pub fn fail() -> Node {
    or([])
}

pub fn word(i: Str, o: Str) -> Node {
    Seq(vec![Eat(i), Emit(o)])
}

pub fn map(i: Str, o: Node) -> Node {
    Seq(vec![Eat(i), o])
}

pub fn prefix(p: Str, node: Node) -> Node {
    Seq(vec![Emit(p), node])
}

pub fn arg(node: Node) -> Node {
    prefix(" ", node)
}

pub fn flag(i: Str, o: Str) -> Node {
    prefix("--", word(i, o))
}

pub fn f(i: Str, o: Str) -> Node {
    prefix("-", word(i, o))
}

pub fn or<const N: usize>(nodes: [Node; N]) -> Node {
    Or(nodes.to_vec())
}

pub fn seq<const N: usize>(nodes: [Node; N]) -> Node {
    Seq(nodes.to_vec())
}

pub fn opt(node: Node) -> Node {
    Set(vec![node], false)
}

pub fn set<const N: usize>(nodes: [Node; N]) -> Node {
    Set(nodes.to_vec(), false)
}

pub fn prefix_set<const N: usize>(p: Str, nodes: [Node; N], one_minimum: bool) -> Node {
    let nodes = nodes.into_iter().map(|node| prefix(p, node)).collect();
    Set(nodes, one_minimum)
}

pub fn argset<const N: usize>(nodes: [Node; N]) -> Node {
    prefix_set(" ", nodes, false)
}

pub fn argset_one<const N: usize>(nodes: [Node; N]) -> Node {
    prefix_set(" ", nodes, true)
}

pub fn number_or_zero() -> Node {
    or([Number, Emit("0")])
}

pub fn map_custom(i: Str, func: fn() -> Option<String>, desc: &'static str) -> Node {
    seq([Eat(i), Custom(func, desc)])
}

pub fn param(i: Str, o: Str, arg: Node) -> Node {
    seq([Emit("--"), word(i, o), prefix("=", or([arg, cursor()]))])
}

pub fn param_opt(i: Str, o: Str, arg: Node) -> Node {
    seq([
        Emit("--"),
        word(i, o),
        opt(prefix("=", or([arg, map("_", cursor())]))),
    ])
}
