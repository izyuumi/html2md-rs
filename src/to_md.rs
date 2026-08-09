//! Functions for rendering parsed HTML nodes as Markdown.
//!
//! Use [`safe_from_html_to_md`] for HTML source, [`safe_from_html_to_md_with_config`] to omit
//! selected subtrees, or [`to_md`] when a parsed [`Node`] already exists.
//! Unknown elements pass through as raw HTML. `script`, `style`, and `title` content is omitted.

use crate::{
    escape::{decode_html_entities, escape_html_attribute, escape_markdown},
    parser::ParseHTMLError,
    structs::{AttributeValues, Attributes, Node, NodeType::*, ToMdConfig},
};

#[derive(Clone, Copy)]
enum TextMode {
    Escaped,
    Raw,
}

enum Work {
    Render(Node, TextMode),
    Discard(Node),
    Append(String),
    AppendStatic(&'static str),
}

fn push_children(stack: &mut Vec<Work>, children: Vec<Node>, text_mode: TextMode) {
    stack.extend(
        children
            .into_iter()
            .rev()
            .map(|child| Work::Render(child, text_mode)),
    );
}

fn discard_children(stack: &mut Vec<Work>, children: Vec<Node>) {
    stack.extend(children.into_iter().map(Work::Discard));
}

fn code_language(attributes: Option<&Attributes>) -> Option<&str> {
    attributes
        .and_then(Attributes::get_class)
        .and_then(|classes| {
            classes
                .split_whitespace()
                .find_map(|class| class.strip_prefix("language-"))
        })
        .filter(|language| !language.is_empty())
}

fn literal_text(nodes: Vec<Node>, config: &ToMdConfig) -> String {
    let mut output = String::new();
    let mut stack: Vec<_> = nodes.into_iter().rev().map(|node| (node, true)).collect();

    while let Some((mut node, include)) = stack.pop() {
        let tag_name = node.tag_name.take();
        let value = node.value.take();
        let children = std::mem::take(&mut node.children);
        let include = include
            && !tag_name
                .as_ref()
                .is_some_and(|tag| config.ignore_rendering.contains(tag));

        if include && tag_name == Some(Text) {
            output.push_str(value.as_deref().unwrap_or_default());
        }
        stack.extend(children.into_iter().rev().map(|child| (child, include)));
    }

    output
}

fn longest_backtick_run(value: &str) -> usize {
    value
        .split(|character| character != '`')
        .map(str::len)
        .max()
        .unwrap_or(0)
}

fn fenced_code(mut content: String, language: Option<&str>) -> String {
    if content.starts_with("\r\n") {
        content.drain(..2);
    } else if content.starts_with('\n') {
        content.remove(0);
    }
    while content.ends_with('\n') {
        content.pop();
        if content.ends_with('\r') {
            content.pop();
        }
    }

    let fence = "`".repeat(longest_backtick_run(&content).saturating_add(1).max(3));
    let mut output = fence.clone();
    if let Some(language) = language {
        output.push_str(language);
    }
    output.push('\n');
    output.push_str(&content);
    if !content.is_empty() {
        output.push('\n');
    }
    output.push_str(&fence);
    output.push('\n');
    output
}

fn inline_code(content: String) -> String {
    if content.is_empty() {
        return content;
    }

    let delimiter = "`".repeat(longest_backtick_run(&content).saturating_add(1).max(1));
    let all_spaces = content.chars().all(|character| character == ' ');
    let padded = content.starts_with('`')
        || content.ends_with('`')
        || (!all_spaces && content.starts_with(' ') && content.ends_with(' '));
    if padded {
        format!("{delimiter} {content} {delimiter}")
    } else {
        format!("{delimiter}{content}{delimiter}")
    }
}

fn push_html_attribute(output: &mut String, key: &str, value: &str) {
    output.push(' ');
    output.push_str(key);
    output.push_str("=\"");
    let decoded = decode_html_entities(value);
    output.push_str(&escape_html_attribute(&decoded));
    output.push('"');
}

fn unknown_opening_tag(
    tag: &str,
    attributes: Option<&Attributes>,
    closes_immediately: bool,
) -> String {
    let mut opening = format!("<{}", tag);

    if let Some(attributes) = attributes {
        if let Some(id) = &attributes.id {
            push_html_attribute(&mut opening, "id", id);
        }
        if let Some(class) = &attributes.class {
            push_html_attribute(&mut opening, "class", class);
        }
        let mut remaining: Vec<_> = attributes.attributes.iter().collect();
        remaining.sort_unstable_by_key(|(key, _)| *key);
        for (key, value) in remaining {
            match value {
                AttributeValues::Bool(true) => {
                    opening.push(' ');
                    opening.push_str(key);
                }
                value => {
                    let value = value.to_string();
                    push_html_attribute(&mut opening, key, &value);
                }
            }
        }
    }

    if closes_immediately {
        opening.push_str(" />");
    } else {
        opening.push('>');
    }
    opening
}

/// Consumes a [`Node`] tree and renders it as Markdown with default configuration.
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
/// let input = Node::new(
///     Some(H1),
///     None,
///     None,
///     None,
///     vec![Node::new(
///         Some(Text),
///         Some("Hello world".to_string()),
///         None,
///         None,
///         Vec::new(),
///     )],
/// );
/// let parsed = to_md(input);
///
/// assert_eq!(parsed, "# Hello world\n");
/// ```
pub fn to_md(node: Node) -> String {
    to_md_with_config(node, &ToMdConfig::default())
}

/// Consumes a [`Node`] tree and renders it as Markdown using [`ToMdConfig`].
///
/// # Arguments
///
/// * `node` - A `Node` to be converted to markdown.
/// * `config` - Rendering configuration. Ignored node types have their complete subtrees omitted.
///
/// # Examples
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
/// let input = Node::new(
///     Some(Div),
///     None,
///     None,
///     None,
///     vec![
///         Node::new(
///             Some(H1),
///             None,
///             None,
///             None,
///             vec![Node::new(
///                 Some(Text),
///                 Some("Hello world".to_string()),
///                 None,
///                 None,
///                 Vec::new(),
///             )],
///         ),
///         Node::new(
///             Some(P),
///             None,
///             None,
///             None,
///             vec![Node::new(
///                 Some(Text),
///                 Some("This will be ignored".to_string()),
///                 None,
///                 None,
///                 Vec::new(),
///             )],
///         ),
///     ],
/// );
/// let config = ToMdConfig {
///     ignore_rendering: vec![P],
/// };
/// let parsed = to_md_with_config(input, &config);
///
/// assert_eq!(parsed, "# Hello world\n");
/// ```
pub fn to_md_with_config(node: Node, config: &ToMdConfig) -> String {
    let mut output = String::new();
    let mut stack = vec![Work::Render(node, TextMode::Escaped)];

    while let Some(work) = stack.pop() {
        match work {
            Work::Discard(mut node) => {
                discard_children(&mut stack, std::mem::take(&mut node.children))
            }
            Work::Append(value) => output.push_str(&value),
            Work::AppendStatic(value) => output.push_str(value),
            Work::Render(mut node, text_mode) => {
                let closes_immediately = node.closes_immediately();
                let tag_name = node.tag_name.take();
                let value = node.value.take();
                let attributes = node.attributes.take();
                let within_special_tag = node.within_special_tag.take();
                let mut children = std::mem::take(&mut node.children);

                let Some(tag_type) = tag_name else {
                    push_children(&mut stack, children, text_mode);
                    continue;
                };
                if config.ignore_rendering.contains(&tag_type) {
                    discard_children(&mut stack, children);
                    continue;
                }

                match tag_type {
                    H1 | H2 | H3 | H4 | H5 | H6 => {
                        stack.push(Work::AppendStatic("\n"));
                        push_children(&mut stack, children, text_mode);
                        output.push_str(match tag_type {
                            H1 => "# ",
                            H2 => "## ",
                            H3 => "### ",
                            H4 => "#### ",
                            H5 => "##### ",
                            H6 => "###### ",
                            _ => "",
                        });
                    }
                    Strong => {
                        stack.push(Work::AppendStatic("**"));
                        push_children(&mut stack, children, text_mode);
                        output.push_str("**");
                    }
                    Em => {
                        stack.push(Work::AppendStatic("*"));
                        push_children(&mut stack, children, text_mode);
                        output.push('*');
                    }
                    A => {
                        let tail = if let Some(link) =
                            attributes.as_ref().and_then(|attrs| attrs.get_href())
                        {
                            // TODO: Percent-decoding can prevent exact href round-tripping.
                            let link = percent_encoding::percent_decode(link.as_bytes())
                                .decode_utf8()
                                .map(|decoded| decoded.into_owned())
                                .unwrap_or(link);
                            if link.contains(' ') {
                                format!("](<{}>)", link)
                            } else {
                                format!("]({})", link)
                            }
                        } else {
                            "]".to_string()
                        };
                        stack.push(Work::Append(tail));
                        push_children(&mut stack, children, text_mode);
                        output.push('[');
                    }
                    Ul => {
                        for child in children.into_iter().rev() {
                            if child
                                .tag_name
                                .as_ref()
                                .is_some_and(|tag| config.ignore_rendering.contains(tag))
                            {
                                stack.push(Work::Discard(child));
                                continue;
                            }
                            let prefix = format!("{}- ", child.leading_spaces());
                            stack.push(Work::Render(child, text_mode));
                            stack.push(Work::Append(prefix));
                        }
                    }
                    Ol => {
                        const MAX_MARKER: u32 = 999_999_999;
                        let start = attributes
                            .as_ref()
                            .and_then(|attrs| attrs.get("start"))
                            .and_then(|start| match start {
                                AttributeValues::String(start) => start.parse::<u32>().ok(),
                                AttributeValues::Number(start) => u32::try_from(start).ok(),
                                _ => None,
                            })
                            .filter(|start| *start <= MAX_MARKER)
                            .unwrap_or(1);
                        for (index, child) in children.into_iter().enumerate().rev() {
                            if child
                                .tag_name
                                .as_ref()
                                .is_some_and(|tag| config.ignore_rendering.contains(tag))
                            {
                                stack.push(Work::Discard(child));
                                continue;
                            }
                            let number = start
                                .saturating_add(u32::try_from(index).unwrap_or(u32::MAX))
                                .min(MAX_MARKER);
                            let prefix = format!("{}{}. ", child.leading_spaces(), number);
                            stack.push(Work::Render(child, text_mode));
                            stack.push(Work::Append(prefix));
                        }
                    }
                    Li => {
                        if !children.iter().any(|child| child.tag_name == Some(P)) {
                            stack.push(Work::AppendStatic("\n"));
                        }
                        push_children(&mut stack, children, text_mode);
                    }
                    P => {
                        if !children.is_empty() {
                            if children.iter().any(|child| {
                                child.tag_name.as_ref().is_some_and(|tag| {
                                    *tag != Comment && !config.ignore_rendering.contains(tag)
                                })
                            }) {
                                stack.push(Work::AppendStatic("\n"));
                            }
                            push_children(&mut stack, children, text_mode);
                        }
                    }
                    Code => {
                        output.push_str(&inline_code(literal_text(children, config)));
                    }
                    Hr => {
                        output.push_str("***\n");
                        discard_children(&mut stack, children);
                    }
                    Br => {
                        output.push_str("  \n");
                        discard_children(&mut stack, children);
                    }
                    Text => {
                        if within_special_tag
                            .as_ref()
                            .is_some_and(|tags| tags.contains(&Blockquote))
                        {
                            output.push_str("> ");
                        }
                        let value = value.unwrap_or_default();
                        match text_mode {
                            TextMode::Escaped => {
                                let decoded = decode_html_entities(&value);
                                output.push_str(&escape_markdown(&decoded));
                            }
                            TextMode::Raw => output.push_str(&value),
                        }
                        discard_children(&mut stack, children);
                    }
                    Html | Head | Link | Meta | Body | Div | Blockquote => {
                        push_children(&mut stack, children, text_mode);
                    }
                    Pre => {
                        if children
                            .first()
                            .is_some_and(|child| child.tag_name == Some(Code))
                        {
                            let Some(code) = children.pop() else {
                                continue;
                            };
                            let mut code = code;
                            let attributes = code.attributes.take();
                            let code_children = std::mem::take(&mut code.children);
                            if config.ignore_rendering.contains(&Code) {
                                discard_children(&mut stack, code_children);
                                continue;
                            }
                            let language = code_language(attributes.as_ref()).map(str::to_owned);
                            output.push_str(&fenced_code(
                                literal_text(code_children, config),
                                language.as_deref(),
                            ));
                        } else {
                            output.push_str(&fenced_code(literal_text(children, config), None));
                        }
                    }
                    Style | Script | Title => discard_children(&mut stack, children),
                    Comment => {
                        output.push_str("<!--");
                        output.push_str(&value.unwrap_or_default());
                        output.push_str("-->");
                        discard_children(&mut stack, children);
                    }
                    Unknown(tag) => {
                        output.push_str(&unknown_opening_tag(
                            &tag,
                            attributes.as_ref(),
                            closes_immediately,
                        ));
                        if !closes_immediately {
                            stack.push(Work::Append(format!("</{}>", tag)));
                            push_children(&mut stack, children, TextMode::Raw);
                        }
                    }
                }
            }
        }
    }

    output
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

/// Parses an owned HTML string and renders it as Markdown with default configuration.
///
/// # Arguments
///
/// * `input` - A string of HTML to be converted to markdown.
///
/// # Errors
///
/// Propagates errors from [`crate::parser::safe_parse_html`].
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

/// Parses an owned HTML string and renders it as Markdown using [`ToMdConfig`].
///
/// # Arguments
///
/// * `input` - A string of HTML to be converted to markdown.
/// * `config` - Rendering configuration. Ignored node types have their complete subtrees omitted.
///
/// # Errors
///
/// Propagates errors from [`crate::parser::safe_parse_html`].
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
