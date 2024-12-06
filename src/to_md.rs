//! This module contains functions that converts a Node to a markdown string.

use crate::{
    parser::ParseHTMLError,
    structs::{AttributeValues, Node, NodeType::*, ToMdConfig},
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
///     structs::{
///         Node,
///         NodeType::{Text, H1},
///     },
///     to_md::to_md,
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
    to_md_with_config(node, &ToMdConfig::default())
}

/// Converts a Node to a markdown string with custom config.
///
/// # Arguments
///
/// * `node` - A `Node` to be converted to markdown.
/// * `config` - A custom configuration, `ToMdConfig`, to use to configure how to render the output markdown.
///
/// # Example's
/// ```
/// use html2md_rs::{
///     structs::{
///         Node,
///         NodeType::{Div, Text, H1, P},
///         ToMdConfig,
///     },
///     to_md::to_md_with_config,
/// };
///
/// let input = Node {
///     tag_name: Some(Div),
///     children: vec![
///         Node {
///             tag_name: Some(H1),
///             children: vec![Node {
///                 tag_name: Some(Text),
///                 value: Some("Hello world".to_string()),
///                 ..Default::default()
///             }],
///             ..Default::default()
///         },
///         Node {
///             tag_name: Some(P),
///             children: vec![Node {
///                 tag_name: Some(Text),
///                 value: Some("This will be ignored".to_string()),
///                 ..Default::default()
///             }],
///             ..Default::default()
///         },
///     ],
///     ..Default::default()
/// };
/// let config = ToMdConfig {
///     ignore_rendering: vec![P],
/// };
/// let parsed = to_md_with_config(input, &config);
///
/// assert_eq!(parsed, "# Hello world\n");
/// ```
pub fn to_md_with_config(node: Node, config: &ToMdConfig) -> String {
    let mut res = String::new();
    let mut tail = String::new();

    let mut follow_child = true; // If the function should process the children of the node, defaults to true. False for some tags; like <ul> and <ol>.

    if let Some(tag_type) = &node.tag_name {
        if config.ignore_rendering.contains(tag_type) {
            follow_child = false;
        } else {
            match tag_type {
                h @ H1 | h @ H2 | h @ H3 | h @ H4 | h @ H5 | h @ H6 => {
                    tail.push('\n');
                    match h {
                        H1 => res.push_str("# "),
                        H2 => res.push_str("## "),
                        H3 => res.push_str("### "),
                        H4 => res.push_str("#### "),
                        H5 => res.push_str("##### "),
                        H6 => res.push_str("###### "),
                        _ => (),
                    }
                }
                Strong => {
                    res.push_str("**");
                    tail.push_str("**");
                }
                Em => {
                    res.push('*');
                    tail.push('*');
                }
                A => {
                    if let Some(link) = node.attributes.as_ref().and_then(|attrs| attrs.get_href())
                    {
                        let link = percent_encoding::percent_decode(link.as_bytes())
                            .decode_utf8()
                            .map(|s| s.to_string())
                            .unwrap_or(link);

                        res.push('[');
                        if link.contains(' ') {
                            tail.push_str(&format!("](<{}>)", link));
                        } else {
                            tail.push_str(&format!("]({})", link));
                        }
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
                        .and_then(|start| match start {
                            AttributeValues::String(start) => start.parse::<usize>().ok(),
                            AttributeValues::Number(start) => Some(start as usize),
                            _ => None,
                        })
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
                        .and_then(|attr| attr.get_class())
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
                Html | Head | Style | Link | Script | Meta | Body | Div | Pre | Blockquote => (),
                Title => {
                    follow_child = false;
                }
                Comment => {
                    res.push_str(&format!("<!--{}-->", &node.value.unwrap_or("".to_string())));
                    return res;
                }
                Unknown(tag) => {
                    res.push_str(&format!("<{}>", tag));
                    tail.push_str(&format!("</{}>", tag));
                }
            }
        }
    }

    if follow_child {
        for child in node.children {
            res.push_str(&to_md_with_config(child, config));
        }
    }

    res.push_str(&tail);

    res
}

// https://github.com/izyuumi/html2md-rs/issues/34
#[test]
fn issue34() {
    let input = "<p><a href=\"/my uri\">link</a></p>";
    let expected = "[link](</my uri>)\n";
    assert_eq!(safe_from_html_to_md(input.to_string()).unwrap(), expected);

    let input = "<p><a href=\"/myuri\">link</a></p>";
    let expected = "[link](/myuri)\n";
    assert_eq!(safe_from_html_to_md(input.to_string()).unwrap(), expected);
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
#[deprecated(
    since = "0.7.0",
    note = "This function is deprecated and will be removed in future versions. Please use the safe_parse_html function instead."
)]
#[allow(deprecated)]
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

/// Safely converts a string of HTML to a markdown string with custom config.
///
/// Returns an error if the HTML is invalid.
///
/// # Arguments
///
/// * `input` - A string of HTML to be converted to markdown.
/// * `config` - Custom configuration `ToMdConfig`
///
/// # Examples
///
/// ```
/// use html2md_rs::{
///     structs::{NodeType::P, ToMdConfig},
///     to_md::safe_from_html_to_md_with_config,
/// };
///
/// let input = "<h1>Hello world</h1><p>this will not be rendered</p>".to_string();
/// let config = ToMdConfig {
///     ignore_rendering: vec![P],
/// };
/// let parsed = safe_from_html_to_md_with_config(input, &config);
///
/// assert_eq!(parsed, Ok("# Hello world\n".to_string()));
/// ```
pub fn safe_from_html_to_md_with_config(
    input: String,
    config: &ToMdConfig,
) -> Result<String, ParseHTMLError> {
    crate::parser::safe_parse_html(input).map(|html| to_md_with_config(html, config))
}
