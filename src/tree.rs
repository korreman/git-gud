use std::fmt::{self, Display, Write};

use anyhow::Result;
use log::trace;

pub type Str = &'static str;

pub use Node::*;

// Major difference I'm considering: Make expansion completely greedy.
// Once a prefix takes you down a path in the tree, that path needs to succeed.

pub const EOL: Str = "EOL";

/// Data structure for constructing grammars.
#[derive(Clone, Debug)]
pub enum Node {
    /// Always succeeds without consuming or producing anything.
    Noop,
    /// Always fails.
    Fail,
    /// Fails if at the end of the input, and the input is terminated.
    Eoi,
    /// Produce the given string.
    Emit(Str),
    /// Run a function and produce its output.
    Custom(fn() -> Option<String>, &'static str),
    /// Run every child in sequence, all children must succeed.
    Seq(Vec<Node>),
    /// Match zero or more alternatives based on their shortcode.
    /// If `set` is true, any number of options can be matched.
    /// The string `prefix` is prepended to the result of each successful alternative.
    /// If `number` is true, an additional custom alternative that parses and spits out a number is included.
    /// If none of the options are successful, `fallback` is run instead.
    Alt {
        /// Prepended to all matching node outputs (including number and EOI, excluding fallback)
        prefix: Str,
        /// Normally only one node can be matched, but if this is true,
        /// an arbitrary sequence of the nodes is matched,
        /// matching each node once at most.
        set: bool,
        /// Regular shortcodes and their matching nodes.
        nodes: Vec<(Str, Node)>,
        /// Add the number node, which parses and outputs a number.
        number: bool,
        /// Fallback node, used if no other nodes match.
        fallback: Box<Node>,
    },
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

    /// Recursively flatten nested sequences into flat sequences.
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
            Alt {
                prefix,
                nodes,
                number,
                fallback,
                set,
            } => Alt {
                prefix,
                nodes: nodes
                    .into_iter()
                    .map(|(p, n)| (p, Self::flatten(n)))
                    .collect(),
                fallback,
                set,
                number,
            },
            x => x,
        }
    }

    pub fn expand<'a>(&self, input: &'a str, eol: bool, output: &mut String) -> Option<&'a str> {
        match self {
            Node::Fail => None,
            Node::Noop => Some(input),
            Node::Eoi => {
                if input.is_empty() && eol {
                    trace!("match");
                    Some(input)
                } else {
                    None
                }
            }
            Node::Emit(result) => {
                trace!("emit {result:?} | {input}");
                output.push_str(result);
                Some(input)
            }
            Node::Custom(func, ..) => {
                trace!("custom func | {input}");
                let expansion = func()?;
                trace!("match: {expansion} | {input}");
                output.push_str(&expansion);
                Some(input)
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
            Node::Alt {
                prefix,
                nodes,
                fallback,
                set,
                number,
            } => {
                trace!("set {{..}} | {input}");
                let mut input = input;
                // Allocate a bool array to track which nodes in the set have already been parsed.
                // The additional final slot is for the number.
                let mut parsed = vec![false; nodes.len() + 1];
                let number_idx = nodes.len();
                // The body of this loop tries to parse each node in sequence,
                // restarting after each match.
                'outer: loop {
                    // Try each node in sequence
                    for (idx, (shortcode, node)) in nodes.iter().enumerate() {
                        // Skip nodes that have already been parsed
                        if parsed[idx] {
                            continue;
                        }
                        // Parse the shortcode (handling EOL) special case.
                        let tail = if *shortcode == EOL {
                            if input.is_empty() && eol {
                                Some(input)
                            } else {
                                None
                            }
                        } else if let Some(tail) = input.strip_prefix(shortcode) {
                            Some(tail)
                        } else {
                            None
                        };
                        if let Some(tail) = tail {
                            parsed[idx] = true;
                            output.push_str(prefix);
                            // Recursively expand the node.
                            match node.expand(tail, eol, output) {
                                // If not a set, we are done, return the tail of the input.
                                Some(tail) if !*set => return Some(tail),
                                Some(tail) => {
                                    input = tail;
                                    continue 'outer;
                                }
                                None => return None,
                            }
                        }
                    }

                    // If none of the nodes parse, try the number node if applicable.
                    if *number || !parsed[number_idx] {
                        // Find the index of the first character that isn't an ASCII digit.
                        let split_idx = input
                            .char_indices()
                            .find_map(|(idx, c)| if c.is_ascii_digit() { None } else { Some(idx) })
                            .unwrap_or(input.len());
                        trace!("split_idx: {split_idx}");
                        // If this index is non-zero,
                        // the input currently starts with a sequence of digits.
                        if split_idx > 0 {
                            // Mark number node as parsed and output the sequence of digits.
                            parsed[number_idx] = true;
                            let (number, tail) = input.split_at(split_idx);
                            trace!("match: {number} | {tail}");
                            input = tail;
                            output.push_str(prefix);
                            output.push_str(number);

                            // If not a set, we are done again, return the tail of the parsed input.
                            if !*set {
                                return Some(tail);
                            }
                        }
                    }
                    // If neither the normal nodes or the number node match,
                    // we are done parsing nodes.
                    break;
                }

                // If no nodes matched, run the fallback, otherwise return the rest of the input.
                if !parsed.iter().any(|x| *x) {
                    trace!("no normal matches, trying fallback");
                    fallback.expand(input, eol, output)
                } else {
                    trace!("set match | {input}");
                    Some(input)
                }
            }
        }
    }

    // fn fmt_helper(&self, f: &mut fmt::Formatter<'_>, indent: u32) -> fmt::Result {
    //     match self {
    //         Eol => f.write_str("<EOL> ⇒ ")?,
    //         Eat(e) => {
    //             f.write_str(e)?;
    //             f.write_str(" ⇒ ")?;
    //         }
    //         Emit(e) => f.write_str(e)?,
    //         Number => f.write_str("<NUMBER>")?,
    //         Custom(_, desc) => f.write_fmt(format_args!("<{desc}>"))?,
    //         Seq(nodes) => {
    //             f.write_char('[')?;
    //             for node in nodes {
    //                 node.fmt_helper(f, indent + 1)?;
    //             }
    //             f.write_char(']')?;
    //         }
    //         Or(nodes) => {
    //             write_newline_indent(f, indent)?;
    //             f.write_str("|| [")?;
    //             for node in nodes {
    //                 write_newline_indent(f, indent + 1)?;
    //                 node.fmt_helper(f, indent + 1)?;
    //             }
    //             write_newline_indent(f, indent)?;
    //             f.write_str("]")?;
    //         }
    //         Set(nodes, _) => {
    //             write_newline_indent(f, indent)?;
    //             f.write_str("⊆ [")?;
    //             for node in nodes {
    //                 write_newline_indent(f, indent + 1)?;
    //                 node.fmt_helper(f, indent + 1)?;
    //             }
    //             write_newline_indent(f, indent)?;
    //             f.write_str("]")?;
    //         }
    //     }
    //     Ok(())
    // }
}

// fn write_newline_indent(f: &mut fmt::Formatter<'_>, indent: u32) -> fmt::Result {
//     f.write_char('\n')?;
//     for _ in 0..indent {
//         f.write_str("  ")?;
//     }
//     Ok(())
// }

// impl Display for Node {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         self.fmt_helper(f, 0)
//     }
// }

// Useful combinators

pub fn cursor() -> Node {
    Custom(|| Some("{GIT_GUD_CURSOR}".to_owned()), "CURSOR")
}

pub fn prefix(p: Str, node: Node) -> Node {
    Seq(vec![Emit(p), node])
}

pub fn zero() -> Node {
    Emit("0")
}

pub fn arg(node: Node) -> Node {
    prefix(" ", node)
}

pub fn flag(o: Str) -> Node {
    prefix("--", Emit(o))
}

pub fn f(o: Str) -> Node {
    prefix("-", Emit(o))
}

pub fn seq<const N: usize>(nodes: [Node; N]) -> Node {
    Seq(nodes.to_vec())
}

pub fn or<const N: usize>(nodes: [(Str, Node); N]) -> Node {
    Alt {
        prefix: "",
        nodes: nodes.to_vec(),
        number: false,
        fallback: Box::new(Fail),
        set: false,
    }
}

pub fn or_opt<const N: usize>(nodes: [(Str, Node); N]) -> Node {
    Alt {
        prefix: "",
        nodes: nodes.to_vec(),
        number: false,
        fallback: Box::new(Noop),
        set: false,
    }
}

pub fn or_fallback<const N: usize>(nodes: [(Str, Node); N], number: bool, fallback: Node) -> Node {
    Alt {
        prefix: "",
        nodes: nodes.to_vec(),
        number,
        fallback: Box::new(fallback),
        set: false,
    }
}

pub fn or_prefix_fallback<const N: usize>(
    p: Str,
    nodes: [(Str, Node); N],
    number: bool,
    fallback: Node,
) -> Node {
    Alt {
        prefix: p,
        nodes: nodes.to_vec(),
        number,
        fallback: Box::new(fallback),
        set: false,
    }
}

pub fn set<const N: usize>(nodes: [(Str, Node); N]) -> Node {
    Alt {
        prefix: "",
        nodes: nodes.to_vec(),
        number: false,
        fallback: Box::new(Noop),
        set: true,
    }
}

pub fn set_prefix_fallback<const N: usize>(
    p: Str,
    nodes: [(Str, Node); N],
    fallback: Node,
) -> Node {
    Alt {
        prefix: p,
        nodes: nodes.to_vec(),
        number: false,
        fallback: Box::new(fallback),
        set: true,
    }
}

pub fn argset<const N: usize>(nodes: [(Str, Node); N]) -> Node {
    set_prefix_fallback(" ", nodes, Noop)
}

pub fn argset_one<const N: usize>(nodes: [(Str, Node); N]) -> Node {
    set_prefix_fallback(" ", nodes, Fail)
}

pub fn param<const N: usize>(name: Str, params: [(Str, Node); N], number: bool) -> Node {
    seq([
        Emit("--"),
        Emit(name),
        or_prefix_fallback("=", params, number, cursor()),
    ])
}

pub fn param_opt<const N: usize>(name: Str, params: [(Str, Node); N], number: bool) -> Node {
    seq([
        Emit("--"),
        Emit(name),
        or_prefix_fallback("=", params, number, Noop),
    ])
}
