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

pub fn param_variant(shorthand: Str, expansion: Str) -> Node {
    term(shorthand, "=", expansion, None)
}

pub fn noneat(prefix: Str, expansion: Str) -> Node {
    Node::NonEat { prefix, expansion }
}

pub fn term(shorthand: Str, prefix: Str, expansion: Str, child: Option<Node>) -> Node {
    let child = child.map(Box::new);
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

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    impl Node {
        pub fn enumerate_shorthand(&self) -> Vec<String> {
            self.enumerate_shorthand_helper("".to_owned())
        }

        fn enumerate_shorthand_helper(&self, mut head: String) -> Vec<String> {
            let heads = self.enumerate_shorthand_helper_2(head);
            println!("{heads:?}");
            heads
        }

        fn enumerate_shorthand_helper_2(&self, mut head: String) -> Vec<String> {
            match self {
                Node::Opt(node) => {
                    let mut heads = Vec::new();
                    heads.push(head.to_owned());
                    heads.extend(node.enumerate_shorthand_helper(head).into_iter());
                    heads
                }
                Node::Seq(nodes) => {
                    let mut heads = vec![head.to_owned()];
                    let mut buffer = vec![];
                    for node in nodes {
                        for head in heads.drain(..) {
                            let new_heads = node.enumerate_shorthand_helper(head);
                            buffer.extend(new_heads.into_iter());
                        }
                        std::mem::swap(&mut heads, &mut buffer);
                    }
                    heads
                }
                Node::Or(nodes) => {
                    let mut heads = vec![];
                    for node in nodes {
                        heads.extend(node.enumerate_shorthand_helper(head.clone()).into_iter());
                    }
                    heads
                }
                Node::Set(nodes) => {
                    let mut heads = vec![];
                    for set in nodes.clone().into_iter().powerset() {
                        let k = set.len();
                        for perm in set.into_iter().permutations(k) {
                            let new_heads =
                                Node::Seq(perm).enumerate_shorthand_helper(head.clone());
                            heads.extend(new_heads.into_iter());
                        }
                    }
                    heads
                }
                Node::NonEat { .. } => {
                    vec![head]
                }
                Node::Term {
                    shorthand, child, ..
                } => {
                    head.push_str(shorthand);
                    let Some(child) = child else {
                        return vec![head];
                    };
                    child.enumerate_shorthand_helper(head)
                }
                Node::Number { empty_is_zero, .. } => {
                    let mut heads = if *empty_is_zero {
                        vec![head.clone()]
                    } else {
                        vec![]
                    };
                    head.push('0');
                    heads.push(head);
                    heads
                }
                Node::Custom { shorthand, .. } => {
                    head.push_str(shorthand);
                    vec![head]
                }
            }
        }
    }

    #[test]
    fn nonambiguity() {
        let mut all_commands = crate::ast::ast().enumerate_shorthand();
        let len = all_commands.len();
        all_commands.sort();
        all_commands.dedup();
        println!("{all_commands:?}");
        assert_eq!(len, all_commands.len(), "grammar shouldn't be ambiguous");
    }
}
