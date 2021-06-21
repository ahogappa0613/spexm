use crate::chex::Chex;
use crate::spex::Spex;
use crate::token;
use crate::{parser::parse, parser::tokenize, parser::Node};

pub fn spex(spex_str: impl Into<String>) -> Spex {
    build_spex(parse(&tokenize("[^a]|..+")))
}

pub fn build_spex(parsed: Node) -> Spex {
    match parsed {
        Node::IncChex { ref tokens } => Spex::build_by_chex(&Chex::new(tokens.clone(), true)),
        Node::ExcChex { ref tokens } => Spex::build_by_chex(&Chex::new(tokens.clone(), false)),
        Node::Or {
            ref left,
            ref right,
        } => &build_spex(left.as_ref().clone()) | &build_spex(right.as_ref().clone()),
        Node::And {
            ref left,
            ref right,
        } => &build_spex(left.as_ref().clone()) & &build_spex(right.as_ref().clone()),
        Node::Invert { ref node } => !&build_spex(node.as_ref().clone()),
        Node::Repeat { ref node } => build_spex(node.as_ref().clone()).repeat(),
        Node::Concat { ref nodes } => nodes
            .into_iter()
            .map(|node| build_spex(node.clone()))
            .reduce(|a, b| a.concat(&b))
            .unwrap(),
    }
}
