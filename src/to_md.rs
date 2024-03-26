//! This module contains functions that converts a Node to a markdown string.

use crate::{
    parser::ParseHTMLError,
    structs::{Node, NodeType::*},
};

/// Converts a Node to a markdown string.
///
/// # Arguments
///
/// * `node` - A Node to be converted to markdown.
///
/// # Examples
///
/// ```
/// use html2md_rs::{
///     to_md::to_md,
///     structs::{
///         Node,
///         NodeType::{H1, Text},
///     },
/// };
///
/// let input = Node {
///     tag_name: Some(H1),
///     value: None,
///     attributes: None,
///     within_special_tag: None,
///     children: vec![Node {
///         tag_name: Some(Text),
///         value: Some("Hello world".to_string()),
///         attributes: None,
///         within_special_tag: None,
///         children: Vec::new(),
///     }],
/// };
/// let parsed = to_md(input);
///
/// assert_eq!(parsed, "# Hello world\n");
/// ```
pub fn to_md(node: Node) -> String {
    let mut res = String::new();
    let mut tail = String::new();

    let mut follow_child = true; // If the function should process the children of the node, defaults to true. False for some tags; like <ul> and <ol>.

    if let Some(tag_type) = &node.tag_name {
        match tag_type {
            H1 | H2 | H3 | H4 | H5 | H6 => tail.push('\n'),
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
                res.push('*');
                tail.push('*');
            }
            A => {
                if let Some(link) = node.attributes.as_ref().and_then(|attrs| attrs.get("href")) {
                    res.push('[');
                    tail.push_str(&format!("]({})", link));
                } else {
                    res.push('[');
                    tail.push(']');
                }
            }
            Ul => {
                for child in &node.children {
                    res.push_str(&child.leading_spaces());
                    res.push_str("- ");
                    res.push_str(&to_md(child.clone()));
                }
                follow_child = false;
            }
            Ol => {
                let mut i = node
                    .attributes
                    .as_ref()
                    .and_then(|attrs| attrs.get("start"))
                    .and_then(|start| start.parse().ok())
                    .unwrap_or(1);
                for child in &node.children {
                    res.push_str(&child.leading_spaces());
                    res.push_str(&format!("{}. ", i));
                    res.push_str(&to_md(child.clone()));
                    i += 1;
                }
                follow_child = false;
            }
            Li => {
                if !&node.children.iter().any(|child| child.tag_name == Some(P)) {
                    tail.push('\n');
                }
            }
            P => {
                if node.children.is_empty() {
                    return res;
                }
                tail.push('\n');
            }
            Code => {
                if let Some(language) = node
                    .attributes
                    .as_ref()
                    .and_then(|attrs| attrs.get("class"))
                    .unwrap_or(&"".to_string())
                    .split_whitespace()
                    .find(|class| class.starts_with("language-"))
                    .map(|class| &class[9..])
                {
                    res.push_str(&format!("```{}", language));
                } else {
                    res.push_str("```\n");
                }
                tail.push_str("```\n");
            }
            Hr => {
                res.push_str("***\n");
                follow_child = false;
            }
            Br => {
                res.push_str("  \n");
                follow_child = false;
            }
            Text => {
                if let Some(special_tags) = &node.within_special_tag {
                    if special_tags.contains(&Blockquote) {
                        res.push_str("> ");
                    }
                }
                res.push_str(&node.value.unwrap_or("".to_string()));
                return res;
            }
            Html | Head | Style | Link | Script | Meta | Title | Body | Div | Pre | Blockquote => {
                ()
            }
            Unknown(tag) => {
                res.push_str(&format!("<{}>", tag));
                tail.push_str(&format!("</{}>", tag));
            }
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

/// Converts a string of HTML to a markdown string.
///
/// Panics if the HTML is invalid.
///
/// # Arguments
///
/// * `input` - A string of HTML to be converted to markdown.
///
/// # Examples
///
/// ```
/// use html2md_rs::to_md::from_html_to_md;
///
/// let input = "<h1>Hello world</h1>".to_string();
/// let parsed = from_html_to_md(input);
///
/// assert_eq!(parsed, "# Hello world\n");
/// ```
pub fn from_html_to_md(input: String) -> String {
    to_md(crate::parser::parse_html(input))
}

/// Safely converts a string of HTML to a markdown string.
///
/// Returns an error if the HTML is invalid.
///
/// # Arguments
///
/// * `input` - A string of HTML to be converted to markdown.
///
/// # Examples
///
/// ```
/// use html2md_rs::to_md::safe_from_html_to_md;
///
/// let input = "<h1>Hello world</h1>".to_string();
/// let parsed = safe_from_html_to_md(input);
///
/// assert_eq!(parsed, Ok("# Hello world\n".to_string()));
/// ```
pub fn safe_from_html_to_md(input: String) -> Result<String, ParseHTMLError> {
    crate::parser::safe_parse_html(input).map(to_md)
}
