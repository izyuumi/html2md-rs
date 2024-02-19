use crate::structs::{Node, NodeType::*};

pub fn to_md(node: Node) -> String {
    let mut res = String::new();
    let mut tail = String::new();

    if let Some(tag_type) = node.tag_name {
        match tag_type {
            H1 | H2 | H3 | H4 | H5 | H6 => tail.push_str("\n"),
            _ => (),
        }
        match tag_type {
            H1 => res.push_str("# "),
            H2 => res.push_str("## "),
            H3 => res.push_str("### "),
            H4 => res.push_str("#### "),
            H5 => res.push_str("##### "),
            H6 => res.push_str("###### "),
            Strong => {
                res.push_str("**");
                tail.push_str("**");
            }
            Em => {
                res.push_str("*");
                tail.push_str("*");
            }
            A => {
                res.push_str("[");
                tail.push_str("](");
            }
            Text => {
                res.push_str(&node.value.unwrap_or("".to_string()));
                return res;
            }
            _ => (),
        }
    }

    for child in node.children {
        res.push_str(&to_md(child));
    }

    res.push_str(&tail);

    res
}
