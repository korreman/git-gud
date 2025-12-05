pub type Str = &'static str;

#[derive(Clone, Debug)]
pub enum Node {
    Opt(Box<Node>),
    Seq(Vec<Node>),
    Or(Vec<Node>),
    Set(Vec<Node>),
    NonEat {
        prefix: Str,
        expansion: Str,
    },
    Term {
        shorthand: Str,
        prefix: Str,
        expansion: Str,
        child: Option<Box<Node>>,
    },
    Number {
        prefix: Str,
        empty_is_zero: bool,
    },
    Custom {
        shorthand: Str,
        prefix: Str,
        func: fn() -> Option<String>,
    },
}

impl Node {
    pub fn expand<'a>(&self, input: &'a str, output: &mut String) -> Option<&'a str> {
        match self {
            Node::NonEat { prefix, expansion } => {
                output.push_str(prefix);
                output.push_str(expansion);
                Some(input)
            }
            Node::Term {
                shorthand,
                prefix,
                expansion,
                child,
            } => {
                let tail = input.strip_prefix(shorthand)?;
                let backtrack_len = output.len();
                output.push_str(prefix);
                output.push_str(expansion);
                if let Some(child) = child {
                    let Some(tail2) = child.expand(tail, output) else {
                        output.truncate(backtrack_len);
                        return None;
                    };
                    return Some(tail2);
                }
                Some(tail)
            }
            Node::Number {
                empty_is_zero,
                prefix,
            } => {
                let split_idx = input
                    .char_indices()
                    .find_map(|(idx, c)| if c.is_ascii_digit() { None } else { Some(idx) })
                    .unwrap_or(input.len());
                let (number, tail) = if split_idx == 0 {
                    if *empty_is_zero {
                        ("0", input)
                    } else {
                        return None;
                    }
                } else {
                    input.split_at(split_idx)
                };
                output.push_str(prefix);
                output.push_str(number);
                Some(tail)
            }
            Node::Custom {
                shorthand,
                prefix,
                func,
            } => {
                let tail = input.strip_prefix(shorthand)?;
                let expansion = func()?;
                output.push_str(prefix);
                output.push_str(&expansion);
                Some(tail)
            }
            Node::Or(nodes) => {
                for node in nodes {
                    if let Some(tail) = node.expand(input, output) {
                        return Some(tail);
                    }
                }
                None
            }
            Node::Set(nodes) => {
                let mut input = input;
                let mut parsed = vec![false; nodes.len()];
                'outer: loop {
                    for (idx, node) in nodes.iter().enumerate() {
                        if parsed[idx] {
                            continue 'outer;
                        }
                        if let Some(tail) = node.expand(input, output) {
                            parsed[idx] = true;
                            input = tail;
                        }
                    }
                    break;
                }
                Some(input)
            }
            Node::Seq(nodes) => {
                let backtrack_len = output.len();
                let mut input = input;
                for node in nodes {
                    let Some(tail) = node.expand(input, output) else {
                        output.truncate(backtrack_len);
                        return None;
                    };
                    input = tail;
                }
                Some(input)
            }
            Node::Opt(node) => {
                if let Some(tail) = node.expand(input, output) {
                    Some(tail)
                } else {
                    Some(input)
                }
            }
        }
    }
}

// Convenience builders

pub fn opt(node: Node) -> Node {
    Node::Opt(Box::new(node))
}

pub fn seq(nodes: &[&Node]) -> Node {
    Node::Seq(nodes.iter().map(|n| (*n).clone()).collect())
}

pub fn or(nodes: &[&Node]) -> Node {
    Node::Or(nodes.iter().map(|n| (*n).clone()).collect())
}

pub fn set(nodes: &[&Node]) -> Node {
    Node::Set(nodes.iter().map(|n| (*n).clone()).collect())
}

pub fn noneat(prefix: Str, expansion: Str) -> Node {
    Node::NonEat { prefix, expansion }
}

pub fn term(shorthand: Str, prefix: Str, expansion: Str, child: Option<Node>) -> Node {
    let child = child.map(|c| Box::new(c));
    Node::Term {
        shorthand,
        prefix,
        expansion,
        child,
    }
}

pub fn number(prefix: Str) -> Node {
    Node::Number {
        prefix,
        empty_is_zero: false,
    }
}

pub fn number_or_zero(prefix: Str) -> Node {
    Node::Number {
        prefix,
        empty_is_zero: true,
    }
}

// Specific builders

pub fn subcmd(shorthand: Str, expansion: Str, child: Node) -> Node {
    term(shorthand, " ", expansion, Some(child))
}

pub fn flag(shorthand: Str, expansion: Str) -> Node {
    term(shorthand, " --", expansion, None)
}

pub fn param(shorthand: Str, expansion: Str, child: Node) -> Node {
    term(shorthand, " --", expansion, Some(child))
}
