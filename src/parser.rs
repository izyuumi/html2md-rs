//! This module contains functions which parsees HTML string into a custom Node struct.
//!
//! The Node struct is used to represent the HTML elements and their children in a tree-like structure.
//!
//! With the `safe_parse_html` function, malformed HTML will return an error instead of panicking.
//! The `parse_html` function is a wrapper around `safe_parse_html` that panics if the input is malformed. However, it is deprecated and will be removed in future versions.

use crate::structs::{
    AttributeValues, Attributes, Node,
    NodeType::{self, *},
};
use std::{collections::VecDeque, fmt::Display};

/// Errors that will be returned when parsing malformed HTML tags
#[derive(Debug, PartialEq, Eq)]
pub enum MalformedTagError {
    /// The closing bracket of the tag is missing
    MissingClosingBracket(u32),
    /// The tag name is missing
    MissingTagName(u32),
}

/// Errors that will be returned when parsing malformed HTML attributes
#[derive(Debug, PartialEq, Eq)]
pub enum MalformedAttributeError {
    /// The quotation mark of the attribute is missing
    MissingQuotationMark(u32),
    /// The attribute name is missing
    MissingAttributeName(u32),
    /// The attribute value is missing
    MissingAttributeValue(u32),
}

/// Errors that can occur when parsing HTML
#[derive(Debug, PartialEq, Eq)]
pub enum ParseHTMLError {
    /// The tag is malformed
    MalformedTag(String, MalformedTagError),
    /// The attribute is malformed
    MalformedAttribute(String, MalformedAttributeError),
}

impl Display for ParseHTMLError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseHTMLError::MalformedTag(tag, error) => match error {
                MalformedTagError::MissingClosingBracket(index) => {
                    write!(
                        f,
                        "Malformed tag: {} - Missing closing bracket at around index {}",
                        tag, index
                    )
                }
                MalformedTagError::MissingTagName(index) => {
                    write!(
                        f,
                        "Malformed tag: {} - Missing tag name at around index {}",
                        tag, index
                    )
                }
            },
            ParseHTMLError::MalformedAttribute(attr, error) => match error {
                MalformedAttributeError::MissingQuotationMark(index) => {
                    write!(
                        f,
                        "Malformed attribute: {} - Missing quotation mark at around index {}",
                        attr, index
                    )
                }
                MalformedAttributeError::MissingAttributeName(index) => {
                    write!(
                        f,
                        "Malformed attribute: {} - Missing attribute name at around index {}",
                        attr, index
                    )
                }
                MalformedAttributeError::MissingAttributeValue(index) => {
                    write!(
                        f,
                        "Malformed attribute: {} - Missing attribute value at around index {}",
                        attr, index
                    )
                }
            },
        }
    }
}

/// Safely parses a string of HTML into a Node struct
///
/// # Arguments
///
/// * `input` - A string slice that holds the HTML to be parsed
///
/// # Examples
///
/// ```
/// use html2md_rs::{
///     parser::safe_parse_html,
///     structs::{
///         Node,
///         NodeType::{Div, Text},
///     },
/// };
///
/// let input = "<div>hello</div>".to_string();
/// let parsed = safe_parse_html(input);
/// let expected = Node {
///     tag_name: Some(Div),
///     value: None,
///     within_special_tag: None,
///     attributes: None,
///     children: vec![Node {
///         tag_name: Some(Text),
///         value: Some("hello".to_string()),
///         attributes: None,
///         within_special_tag: None,
///         children: Vec::new(),
///     }],
/// };
///
/// assert_eq!(parsed, Ok(expected));
/// ```
pub fn safe_parse_html(input: String) -> Result<Node, ParseHTMLError> {
    // current_index is the index of the current character being processed
    let mut current_index = 0;
    // nodes is a vector of nodes that will be returned as an attribute of the resulting node
    let mut nodes = Vec::new();
    // stack is a LIFO stack of nodes that are being processed
    let mut stack: Vec<Node> = Vec::new();

    while current_index < input.len() {
        let rest = &input[current_index..];
        if rest.starts_with("<!") {
            // if the current character is an exclamation mark, it's a comment or DOCTYPE
            if rest.starts_with("<!DOCTYPE") {
                // if the comment is a DOCTYPE, ignore it
                current_index += rest.find('>').unwrap() + 1;
                continue;
            }
            // find the closing comment tag
            if let Some(closing_comment_index) = rest.find("-->") {
                // if the closing comment tag is found, the comment is valid
                // extract the comment from the rest
                let comment = &rest[..closing_comment_index + 3];
                // create a new node with the comment
                let new_node = Node {
                    tag_name: Some(Comment),
                    value: Some(
                        comment
                            .trim_start_matches("<!")
                            .trim_start_matches("--")
                            .trim_end_matches("-->")
                            .to_string(),
                    ),
                    attributes: None,
                    within_special_tag: None,
                    children: Vec::new(),
                };
                // add the new_node to the stack
                nodes.push(new_node);
                // increment the current_index by the closing_comment_index + 3
                // and continue to the next iteration
                current_index += closing_comment_index + 3;
                continue;
            }
            // if the closing comment tag is not found, the comment is malformed
            return Err(ParseHTMLError::MalformedTag(
                rest.to_string(),
                MalformedTagError::MissingClosingBracket(current_index as u32),
            ));
        }

        if rest.starts_with('<') {
            if let Some(mut closing_index) = find_closing_bracket_index(rest) {
                // if the tag is a self-closing tag (i.e. <tag_name ... />)
                let self_closing = if rest.chars().nth(closing_index - 1) == Some('/') {
                    // if the last character right before the closing bracket is a forward slash, the tag is self-closing
                    // closing_index is the index of the closing bracket, so decrement it to ignore the forward slash
                    closing_index -= 1;
                    true
                } else {
                    // if the last character right before the closing bracket is not a forward slash, the tag is not self-closing
                    false
                };

                // the tag content is the string between the opening and closing brackets
                let tag_content = &rest[1..closing_index];

                // initialize the node name and attribute map
                let node_name;
                let mut attribute_map = None;
                if let Some(space_index) = tag_content.find(|c: char| c.is_whitespace()) {
                    // if the tag contains a space, split the tag into the node name and attributes
                    // space_index is the index of the first spce
                    // node_name is the tag name (i.e. <tag_name ...>)
                    node_name = &tag_content[..space_index];
                    // attributes is the string after the first space before the closing bracket
                    let attributes = &tag_content[space_index..];
                    // parse the attribute string into a map
                    match parse_tag_attributes(attributes, current_index) {
                        Ok(map) => attribute_map = map,
                        Err(err) => return Err(err),
                    }
                } else {
                    // if the tag doesn't contain a space, the tag is the node name
                    node_name = tag_content;
                }

                if node_name.is_empty() {
                    // if the tag name is empty, the tag is malformed
                    return Err(ParseHTMLError::MalformedTag(
                        tag_content.to_string(),
                        MalformedTagError::MissingTagName(current_index as u32),
                    ));
                }

                if rest.starts_with("</") {
                    // if the tag is a closing tag, pop the last node from the stack and add it to the parent
                    match stack.pop() {
                        Some(last_node) => {
                            if stack.is_empty() {
                                // if the stack is empty, the last node is the root node
                                nodes.push(last_node);
                            } else {
                                let parent = stack.last_mut().unwrap(); // stack is not empty, so unwrap is safe
                                parent.children.push(last_node);
                            }
                            current_index += closing_index + 1;
                            continue;
                        }
                        None => {
                            // if there is nothing in the stack, the tag is malformed
                            let closing_bracket_of_closing_tag = rest.find('>');
                            return Err(ParseHTMLError::MalformedTag(
                                if let Some(index) = closing_bracket_of_closing_tag {
                                    // if there is a closing bracket, return the tag with the error
                                    rest[..index + 1].to_string()
                                } else {
                                    rest.to_string()
                                },
                                MalformedTagError::MissingClosingBracket(current_index as u32),
                            ));
                        }
                    }
                }

                // parse thae tag name into a NodeType from the node_name string
                let node_type = NodeType::from_tag_str(node_name);

                // initialize a new node with the tag name and attribute map
                let mut new_node = Node {
                    tag_name: Some(node_type.clone()),
                    value: None,
                    attributes: attribute_map,
                    within_special_tag: None,
                    children: Vec::new(),
                };

                if self_closing {
                    // if the tag is self-closing, add the node to the parent
                    // if a parent does not exist, add the node to the nodes vector
                    if let Some(parent) = stack.last_mut() {
                        modify_node_with_parent(&mut new_node, parent);
                        parent.children.push(new_node);
                    } else {
                        nodes.push(new_node);
                    }
                    // because the tag is self-closing, increment the current_index by the closing_index + 2
                    // and continute to the next iteration
                    current_index += closing_index + 2;
                    continue;
                }
                // if the tag is not self-closing
                // add the new_node to the stack
                if let Some(parent) = stack.last_mut() {
                    modify_node_with_parent(&mut new_node, parent);
                }
                stack.push(new_node);
                // because the tag is not self-closing, increment the current_index by the closing_index + 1
                current_index += closing_index + 1;
                continue;
            } else {
                // if a closing bracket is not found, the tag is malformed
                return Err(ParseHTMLError::MalformedTag(
                    rest.to_string(),
                    MalformedTagError::MissingClosingBracket(current_index as u32),
                ));
            }
        }

        // if the current character is not a '<', it's just a text
        // if an opening bracket is not found, the rest is the content of the text
        // else, anything upto the opening bracket is the content of the text
        let next_opening_tag = rest.find('<').unwrap_or(rest.len());
        let text = &rest[..next_opening_tag];
        if text.trim().is_empty() {
            // if text is empty or only whitespace, ignore it
            // increment the current_index by next_opening_tag and continue to the next iteration
            current_index += next_opening_tag;
            continue;
        }

        // initialize new_node as text with the content of the text
        let new_node = Node {
            tag_name: Some(Text),
            value: Some(text.to_string()),
            attributes: None,
            within_special_tag: None,
            children: Vec::new(),
        };

        // add the new_node to the stack
        modify_stack_with_node(&mut stack, new_node);

        current_index += next_opening_tag
    }

    // if the stack is not empty, add the stack to the nodes vector
    if !stack.is_empty() {
        for stack_node in stack.drain(..) {
            nodes.push(stack_node);
        }
    }

    if nodes.len() == 1 {
        return Ok(nodes.remove(0));
    }

    Ok(Node {
        tag_name: None,
        value: None,
        attributes: None,
        within_special_tag: None,
        children: nodes,
    })
}

/// Adds a new node to the stack with respect to the parent node's special tag and tag type
///
/// # Arguments
///
/// * `stack` - A mutable reference to a vector of nodes
/// * `new_node` - A mutable reference to a node to be added to the stack
fn modify_stack_with_node(stack: &mut Vec<Node>, mut new_node: Node) {
    if let Some(parent) = stack.last_mut() {
        // if the stack is not empty, add new_node to the parent
        // modify the new_node with the parent's within_special_tag and tag type
        modify_node_with_parent(&mut new_node, parent);
        parent.children.push(new_node.clone());
        return;
    }
    // if stack is empty, add new_node to the stack
    stack.push(new_node.clone());
}

/// Modifies a node with the parent's within_special_tag and tag type
///
/// # Arguments
///
/// * `node` - A mutable reference to a Node to be modified
/// * `parent` - A reference to the parent Node
fn modify_node_with_parent(node: &mut Node, parent: &Node) {
    if parent.within_special_tag.is_some() {
        node.within_special_tag
            .clone_from(&parent.within_special_tag)
    }
    if let Some(parent_tag_name) = &parent.tag_name {
        if parent_tag_name.is_special_tag() {
            if let Some(within_special_tag) = &mut node.within_special_tag {
                within_special_tag.push(parent_tag_name.clone());
            } else {
                node.within_special_tag = Some(vec![parent_tag_name.clone()]);
            }
        }
    }
}

/// Parses a string of HTML into a Node struct
///
/// Panics if the input is malformed
///
/// # Arguments
///
/// * `input` - A string slice that holds the HTML to be parsed
///
/// # Examples
///
/// ```
/// use html2md_rs::{
///     parser::parse_html,
///     structs::{
///         Node,
///         NodeType::{Div, Text},
///     },
/// };
///
/// let input = "<div>hello</div>".to_string();
/// let parsed = parse_html(input);
/// let expected = Node {
///     tag_name: Some(Div),
///     value: None,
///     attributes: None,
///     within_special_tag: None,
///     children: vec![Node {
///         tag_name: Some(Text),
///         value: Some("hello".to_string()),
///         attributes: None,
///         within_special_tag: None,
///         children: Vec::new(),
///     }],
/// };
///
/// assert_eq!(parsed, expected);
/// ```
#[deprecated(
    since = "0.7.0",
    note = "This function is deprecated and will be removed in future versions. Please use the safe_parse_html function instead."
)]
pub fn parse_html(input: String) -> Node {
    let parsed = safe_parse_html(input);
    match parsed {
        Ok(node) => node,
        Err(err) => panic!("error parsing html: {:?}", err),
    }
}

fn parse_tag_attributes(
    tag_attributes: &str,
    current_index: usize,
) -> Result<Option<Attributes>, ParseHTMLError> {
    let tag_attributes = tag_attributes.trim();

    // if the input is empty or only whitespace, return None
    if tag_attributes.is_empty() {
        return Ok(None);
    }

    let mut attribute_map = Attributes::new();

    let mut current_key = String::new();
    let mut current_value_in_quotes = String::new();
    let mut in_quotes = false;
    let mut may_be_reading_non_quoted_value = false;

    for char in tag_attributes.trim().chars() {
        // iterate through each character in the trimmed tag_attributes string

        if in_quotes {
            // if we are in quotation marks, just add the character to the current_value_in_quotes
            // except for if the character is a quotation mark, which indicates the end of the value
            if char.eq(&'"') {
                // if the character is a quotation mark, add the current_value_in_quotes to the attribute_map
                // and reset the current_key and current_value_in_quotes
                add_to_attribute_map(&mut attribute_map, &current_key, &current_value_in_quotes);
                current_key.clear();
                current_value_in_quotes.clear();
                in_quotes = false;
                continue;
            }
            current_value_in_quotes.push(char);
            continue;
        }

        if char.eq(&'"') {
            // if the character is a quotation mark, we are about to start the value
            // we know in_quotes is false because that is checked above
            if current_key.is_empty() {
                // if the current_key is empty, the attribute is malformed
                return Err(ParseHTMLError::MalformedAttribute(
                    tag_attributes.to_string(),
                    MalformedAttributeError::MissingAttributeName(current_index as u32),
                ));
            }
            // set the in_quotes flag to true
            in_quotes = true;
            // if the character is a quotation mark, we are going to be in quotes
            // so we don't need to keep track of non-quoted value flag
            may_be_reading_non_quoted_value = false;
            continue;
        }

        if char.is_whitespace() {
            if may_be_reading_non_quoted_value {
                if current_value_in_quotes.is_empty() {
                    // if we are reading a non-quoted value and the value is empty, we can ignore the whitespace
                    continue;
                }
                // if we are reading a non-quoted value, the whitespace indicates the end of the value
                // add the value to the attribute_map
                add_to_attribute_map(&mut attribute_map, &current_key, &current_value_in_quotes);
                current_key.clear();
                current_value_in_quotes.clear();
                may_be_reading_non_quoted_value = false;
                continue;
            }
            // if the character is whitespace, if could be indicating the end of a key
            if !current_key.is_empty() {
                // if the key has some value, add it to the attribute_map with value true
                attribute_map.insert(current_key.clone(), AttributeValues::from(true));
                current_key.clear();
                continue;
            }
            // if the current_key is empty, the whitespace can be ignored
            continue;
        }

        if !in_quotes && !may_be_reading_non_quoted_value && char.eq(&'=') {
            // if the character is an equal sign, the current_key is complete
            // if we are in quotes or reading a non-quoted value, the equal sign is part of the value
            // and we are about to start the value
            if current_key.is_empty() {
                // if the current_key is empty, the attribute is malformed
                return Err(ParseHTMLError::MalformedAttribute(
                    tag_attributes.to_string(),
                    MalformedAttributeError::MissingAttributeName(current_index as u32),
                ));
            }
            // equal sign indicates the start of the value up to the next whitespace
            may_be_reading_non_quoted_value = true;
            continue;
        }

        if may_be_reading_non_quoted_value {
            // if we are reading a non-quoted value, add the character to the current_value_in_quotes
            current_value_in_quotes.push(char);
            continue;
        }

        // otherwise, add the character to the current_key
        current_key.push(char);
    }

    if may_be_reading_non_quoted_value && !current_value_in_quotes.is_empty() {
        // if we are reading a non-quoted value and the value is not empty, add the value to the attribute_map
        add_to_attribute_map(&mut attribute_map, &current_key, &current_value_in_quotes);
    }

    if in_quotes {
        return Err(ParseHTMLError::MalformedAttribute(
            current_value_in_quotes,
            MalformedAttributeError::MissingQuotationMark(current_index as u32),
        ));
    }

    // if not, return the attribute map
    match attribute_map.is_empty() {
        true => Ok(None),
        false => Ok(Some(attribute_map)),
    }
}

fn add_to_attribute_map(
    attribute_map: &mut Attributes,
    current_key: &str,
    current_value_in_quotes: &str,
) {
    if current_key.is_empty() || current_value_in_quotes.is_empty() {
        return;
    }
    attribute_map.insert(
        current_key.to_string(),
        AttributeValues::from(current_value_in_quotes),
    );
}

fn find_closing_bracket_index(rest: &str) -> Option<usize> {
    let mut attribute_value_stack: VecDeque<char> = VecDeque::new(); // needed to fix #31
    for (idx, char) in rest.char_indices() {
        if char.eq(&'"') || char.eq(&'\'') {
            if let Some(back) = attribute_value_stack.back() {
                if back.eq(&char) {
                    attribute_value_stack.pop_back();
                } else {
                    attribute_value_stack.push_back(char)
                }
            } else {
                attribute_value_stack.push_back(char)
            }
        }
        if char.eq(&'>') && attribute_value_stack.is_empty() {
            return Some(idx);
        }
    }
    None
}

// https://github.com/izyuumi/html2md-rs/issues/25
#[test]
fn issue_25() {
    let input = "property=\"og:type\" content= \"website\"".to_string();
    let expected = Attributes::from(vec![
        ("property".to_string(), AttributeValues::from("og:type")),
        ("content".to_string(), AttributeValues::from("website")),
    ]);
    let parsed = parse_tag_attributes(&input, 0).unwrap().unwrap();
    assert_eq!(parsed, expected);
}

// https://github.com/izyuumi/html2md-rs/issues/31
#[test]
fn issue_31() {
    let input = r#"<img src="https://exmaple.com/img.png" alt="Rust<br/>Logo"/>"#.to_string();
    let expected = Node {
        tag_name: Some(Unknown("img".to_string())),
        value: None,
        attributes: Some(Attributes {
            id: None,
            class: None,
            href: None,
            attributes: std::collections::HashMap::from([
                (
                    "src".to_string(),
                    AttributeValues::from("https://exmaple.com/img.png"),
                ),
                ("alt".to_string(), AttributeValues::from("Rust<br/>Logo")),
            ]),
        }),
        children: Vec::new(),
        within_special_tag: None,
    };
    let parsed = safe_parse_html(input).unwrap();
    assert_eq!(parsed, expected)
}

// https://github.com/izyuumi/html2md-rs/issues/36
#[test]
fn issue_36() {
    let input = "<img src=\"https://hoerspiele.dra.de/fileadmin/www.hoerspiele.dra.de/images/vollinfo/4970918_B01.jpg\" />".to_string();
    let expected = Node {
        tag_name: Some(Unknown("img".to_string())),
        value: None,
        attributes: Some(Attributes {
            id: None,
            class: None,
            href: None,
            attributes: std::collections::HashMap::from([(
                "src".to_string(),
                AttributeValues::from("https://hoerspiele.dra.de/fileadmin/www.hoerspiele.dra.de/images/vollinfo/4970918_B01.jpg"),
            )]),
        }),
        children: Vec::new(),
        within_special_tag: None,
    };
    let parsed = safe_parse_html(input).unwrap();
    assert_eq!(parsed, expected);

    let input = r#"<!DOCTYPE html><meta http-equiv="content-type" content="text/html; charset=utf-8"><div class="column"><div class="gallery-wrap single">
    <div class="gallery-container">
        <figure class="image">
            <figure class="image">
            <img title="Illustration »Der dunkle Kongress« © ARD / Jürgen Frey"
                 alt="Illustration »Der dunkle Kongress« © ARD / Jürgen Frey" 
                 src="https://hoerspiele.dra.de/fileadmin/www.hoerspiele.dra.de/images/vollinfo/4970918_B01.jpg">
                <figcaption class="image-caption">Illustration »Der dunkle Kongress«
© ARD / Jürgen Frey</figcaption>
</figure></div></div></div>"#.to_string();
    safe_parse_html(input).unwrap();
}
