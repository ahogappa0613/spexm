use crate::token::{self, Token};

type Tokens = Vec<String>;

pub fn tokenize(input_str: impl Into<String>) -> Tokens {
    let mut ret: Tokens = Vec::new();
    let mut escape_flg = false;
    for ref char in input_str.into().chars() {
        if escape_flg {
            if char == &Token::ESC.value() {
                ret.push(format!("{}{}", Token::ESC.value(), Token::ESC.value()))
            } else if Token::escapes().contains(char) {
                ret.push(format!("{}{}", Token::ESC.value(), char));
            } else {
                ret.push(Token::ESC.value().to_string());
                ret.push(char.to_string());
            }
            escape_flg = false;
        } else if char == &Token::ESC.value() {
            escape_flg = true;
        } else {
            ret.push(char.to_string());
        }
    }
    if escape_flg {
        ret.push(format!("{}{}", Token::ESC.value(), Token::ESC.value()))
    }

    ret
}

pub fn parse(tokens: &Tokens) -> Node {
    parse_and_or(tokens)
}

pub fn parse_and_or(tokens: &Tokens) -> Node {
    let mut gourp_level = 0;
    for (i, token) in tokens.iter().enumerate() {
        if *token == Token::SP_S.value().to_string() {
            gourp_level += 1;
        } else if *token == Token::SP_E.value().to_string() {
            gourp_level -= 1;
        } else if gourp_level == 0 {
            if *token == Token::AND.value().to_string() {
                return get_and_node(
                    &parse_and_or(&tokens[..i].to_vec()),
                    &parse_and_or(&tokens[i + 1..].to_vec()),
                );
            } else if *token == Token::OR.value().to_string() {
                return get_or_node(
                    &parse_and_or(&tokens[..i].to_vec()),
                    &parse_and_or(&tokens[i + 1..].to_vec()),
                );
            }
        }
    }
    if gourp_level != 0 {
        panic!("SyntaxError {} not enough", Token::SP_E.value());
    }

    return parse_invert(tokens);
}

pub fn parse_invert(tokens: &Tokens) -> Node {
    if tokens.len() == 0 {
        panic!("SyntaxError invalid blank node")
    } else if tokens[0] == Token::INVT.value().to_string() {
        get_invert_node(parse_concat(&tokens[1..].to_vec()))
    } else {
        parse_concat(&tokens)
    }
}

pub fn parse_concat(tokens: &Tokens) -> Node {
    let mut gourp_level = 0;
    let mut in_ch = false;
    let mut current_tokens: Tokens = vec![];
    let mut node_kind = NodeKind::UNNECESSARY;
    let mut nodes: Vec<Node> = vec![];

    for (i, token) in tokens.iter().enumerate() {
        if node_kind != NodeKind::UNNECESSARY {
            let mut node = match node_kind {
                NodeKind::GROUP => parse_and_or(&current_tokens),
                NodeKind::SINGLE => parse_inc_chex(&current_tokens),
                NodeKind::MULTI => parse_chex(&current_tokens),
                _ => unreachable!(),
            };
            current_tokens = vec![];
            node_kind = NodeKind::UNNECESSARY;
            if token == &Token::REPT.value().to_string() {
                node = get_repeat_node(node);
                nodes.push(node);
                continue;
            }
            nodes.push(node);
        }

        if gourp_level == 0 {
            if token == &Token::INVT.value().to_string() {
                panic!("SyntaxError {} Npt at the beginning", Token::INVT.value());
            } else if token == &Token::SP_E.value().to_string() {
                panic!("SyntaxError {} invalid position", Token::SP_E.value());
            } else if token == &Token::REPT.value().to_string() {
                panic!("SyntaxError {} invalid position", Token::REPT.value());
            } else if token == &Token::SP_S.value().to_string() {
                if in_ch {
                    panic!("SyntaxError {} invalid position", Token::SP_S.value());
                }
                gourp_level += 1;
            } else if token == &Token::CH_S.value().to_string() {
                if in_ch {
                    panic!("SyntaxError {} invalid position", Token::CH_S.value());
                }
                in_ch = true;
            } else if token == &Token::CH_E.value().to_string() {
                if !in_ch {
                    panic!("SyntaxError {} invalid position", Token::CH_E.value());
                }
                in_ch = false;
                node_kind = NodeKind::MULTI;
            } else {
                current_tokens.push(token.clone());
                if !in_ch {
                    node_kind = NodeKind::SINGLE;
                }
            }
        } else {
            if token == &Token::SP_S.value().to_string() {
                gourp_level += 1;
                current_tokens.push(token.clone());
            } else if token == &Token::SP_E.value().to_string() {
                gourp_level -= 1;
                if gourp_level != 0 {
                    current_tokens.push(token.clone());
                } else {
                    node_kind = NodeKind::GROUP;
                }
            } else {
                current_tokens.push(token.clone());
            }
        }
    }

    if gourp_level != 0 {
        panic!("SyntaxError {} not enough", Token::SP_E.value());
    }
    if in_ch {
        panic!("SyntaxError {} not nough", Token::CH_E.value());
    }
    if node_kind != NodeKind::UNNECESSARY {
        match node_kind {
            NodeKind::GROUP => nodes.push(parse_and_or(&current_tokens)),
            NodeKind::SINGLE => nodes.push(parse_inc_chex(&current_tokens)),
            NodeKind::MULTI => nodes.push(parse_chex(&current_tokens)),
            _ => unreachable!(),
        }
    }
    if nodes.len() == 0 {
        panic!("SyntaxError invalid blank code");
    } else if nodes.len() == 1 {
        return nodes[0].clone();
    } else {
        return get_concat_node(nodes);
    }
}

pub fn parse_chex(tokens: &Tokens) -> Node {
    if tokens.len() > 0 && tokens[0] == Token::DENY.value().to_string() {
        get_exc_chex(&tokens[1..].to_vec())
    } else {
        get_inc_chex(tokens)
    }
}

pub fn parse_inc_chex(tokens: &Tokens) -> Node {
    if tokens.contains(&Token::WHOL.value().to_string()) {
        get_exc_chex(&vec![])
    } else {
        get_inc_chex(tokens)
    }
}

pub fn parse_exc_chex(tokens: &Tokens) -> Node {
    if tokens.contains(&Token::WHOL.value().to_string()) {
        get_inc_chex(&vec![])
    } else {
        get_exc_chex(tokens)
    }
}

pub fn get_and_node(left: &Node, right: &Node) -> Node {
    Node::And {
        left: Box::new(left.clone()),
        right: Box::new(right.clone()),
    }
}

pub fn get_or_node(left: &Node, right: &Node) -> Node {
    Node::Or {
        left: Box::new(left.clone()),
        right: Box::new(right.clone()),
    }
}

pub fn get_concat_node(nodes: Vec<Node>) -> Node {
    Node::Concat { nodes }
}

pub fn get_invert_node(node: Node) -> Node {
    Node::Invert {
        node: Box::new(node),
    }
}

pub fn get_repeat_node(node: Node) -> Node {
    Node::Repeat {
        node: Box::new(node),
    }
}

pub fn get_inc_chex(tokens: &Tokens) -> Node {
    Node::IncChex {
        tokens: tokens.clone(),
    }
}

pub fn get_exc_chex(tokens: &Tokens) -> Node {
    Node::ExcChex {
        tokens: tokens.clone(),
    }
}

#[derive(Debug, Clone)]
pub enum Node {
    And { left: Box<Node>, right: Box<Node> },
    Or { left: Box<Node>, right: Box<Node> },
    IncChex { tokens: Tokens },
    ExcChex { tokens: Tokens },
    Invert { node: Box<Node> },
    Repeat { node: Box<Node> },
    Concat { nodes: Vec<Node> },
}

#[derive(Debug, Clone, PartialEq)]
enum NodeKind {
    /// 0
    UNNECESSARY,
    /// 1
    GROUP,
    /// 2
    SINGLE,
    /// 3
    MULTI,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        assert_eq!(tokenize("[abc]"), vec!["[", "a", "b", "c", "]"]);
        assert_eq!(tokenize("[^abc]"), vec!["[", "^", "a", "b", "c", "]"]);
        assert_eq!(tokenize("[\\[abc]"), vec!["[", "\\[", "a", "b", "c", "]"]);
    }
}
