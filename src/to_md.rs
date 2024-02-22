use crate::structs::{Node, NodeType::*};

pub fn to_md(node: Node) -> String {
    let mut res = String::new();
    let mut tail = String::new();
    let mut follow_child = true;

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
            Ul => {
                for child in &node.children {
                    res.push_str("- ");
                    res.push_str(&to_md(child.clone()));
                }
                follow_child = false;
            }
            Ol => {
                let mut i = 1;
                for child in &node.children {
                    res.push_str(&format!("{}. ", i));
                    res.push_str(&to_md(child.clone()));
                    i += 1;
                }
                follow_child = false;
            }
            Li => {
                tail.push_str("\n");
            }
            P => {
                if node.children.len() == 0 {
                    return res;
                }
                tail.push_str("\n\n");
            }
            Text => {
                res.push_str(&node.value.unwrap_or("".to_string()));
                return res;
            }
            _ => (),
        }
    }

    if follow_child {
        for child in node.children {
            res.push_str(&to_md(child));
        }
    }

    res.push_str(&tail);

    res
}

pub fn from_html_to_md(input: String) -> String {
    let node = crate::parser::parse_html(input);
    to_md(node)
}
