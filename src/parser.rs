use crate::structs::{
    Node,
    NodeType::{self, *},
};
use std::{collections::HashMap, str::FromStr};

#[derive(Debug, PartialEq, Eq)]
pub enum MalformedTagError {
    MissingClosingBracket(u32),
    MissingTagName(u32),
}

#[derive(Debug, PartialEq, Eq)]
pub enum MalformedAttributeError {
    MissingQuotationMark(u32),
    MissingAttributeName(u32),
    MissingAttributeValue(u32),
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseHTMLTypeError {
    MalformedTag(String, MalformedTagError),
    MalformedAttribute(String, MalformedAttributeError),
    UnknownNodeType(String, u32),
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
///     attributes: None,
///     children: vec![Node {
///         tag_name: Some(Text),
///         value: Some("hello".to_string()),
///         attributes: None,
///         children: Vec::new(),
///     }],
/// };
///
/// assert_eq!(parsed, Ok(expected));
/// ```
pub fn safe_parse_html(input: String) -> Result<Node, ParseHTMLTypeError> {
    let mut current_index = 0;
    let mut nodes = Vec::new();
    let mut stack: Vec<Node> = Vec::new();

    while current_index < input.len() {
        let rest = &input[current_index..];
        if rest.starts_with('<') {
            if let Some(mut closing_index) = rest.find('>') {
                let self_closing = if rest.chars().nth(closing_index - 1) == Some('/') {
                    closing_index -= 1;
                    true
                } else {
                    false
                };

                let tag_content = &rest[1..closing_index];

                let node_name;
                let mut attribute_map = None;
                if let Some(space_index) = tag_content.find(' ') {
                    node_name = &tag_content[..space_index];
                    let attributes = &tag_content[space_index..];
                    match parse_tag_attributes(attributes, current_index) {
                        Ok(map) => attribute_map = map,
                        Err(err) => return Err(err),
                    }
                } else {
                    node_name = tag_content;
                }

                if node_name.is_empty() {
                    return Err(ParseHTMLTypeError::MalformedTag(
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
                            return Err(ParseHTMLTypeError::MalformedTag(
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

                match NodeType::from_str(node_name) {
                    Ok(node_type) => {
                        let node = Node {
                            tag_name: Some(node_type),
                            value: None,
                            attributes: attribute_map,
                            children: Vec::new(),
                        };
                        if self_closing {
                            if let Some(parent) = stack.last_mut() {
                                parent.children.push(node);
                            } else {
                                nodes.push(node);
                            }
                            current_index += closing_index + 2;
                        } else {
                            stack.push(node);
                            current_index += closing_index + 1;
                        }
                        continue;
                    }
                    Err(_) => {
                        return Err(ParseHTMLTypeError::UnknownNodeType(
                            node_name.to_string(),
                            current_index as u32,
                        ));
                    }
                }
            } else {
                return Err(ParseHTMLTypeError::MalformedTag(
                    rest.to_string(),
                    MalformedTagError::MissingClosingBracket(current_index as u32),
                ));
            }
        }
        let next_opening_tag = rest.find('<').unwrap_or(rest.len());
        let text = &rest[..next_opening_tag];
        if text.trim().is_empty() {
            current_index += next_opening_tag;
            continue;
        }
        match stack.last_mut() {
            Some(parent) => {
                parent.children.push(Node {
                    tag_name: Some(Text),
                    value: Some(text.to_string()),
                    attributes: None,
                    children: Vec::new(),
                });
            }
            None => {
                nodes.push(Node {
                    tag_name: Some(Text),
                    value: Some(text.to_string()),
                    attributes: None,
                    children: Vec::new(),
                });
            }
        }
        current_index += next_opening_tag;
    }

    if nodes.len() == 1 {
        return Ok(nodes.remove(0));
    }

    Ok(Node {
        tag_name: None,
        value: None,
        attributes: None,
        children: nodes,
    })
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
///     children: vec![Node {
///         tag_name: Some(Text),
///         value: Some("hello".to_string()),
///         attributes: None,
///         children: Vec::new(),
///     }],
/// };
///
/// assert_eq!(parsed, expected);
/// ```
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
) -> Result<Option<HashMap<String, String>>, ParseHTMLTypeError> {
    // if the input is empty or only whitespace, return None
    if tag_attributes.trim().is_empty() {
        return Ok(None);
    }

    let mut attribute_map = HashMap::new();

    let mut current_key = String::new();
    let mut current_value_in_quotes = String::new();
    let mut in_quotes = false;

    // loop through each string split by whitespace
    for attr in tag_attributes.split_whitespace() {
        // if the attribute contains an equals sign, it's a key-value pair
        if attr.contains('=') {
            // if the attribute starts with an equals sign, it's malformed
            if attr.starts_with('=') {
                return Err(ParseHTMLTypeError::MalformedAttribute(
                    attr.to_string(),
                    MalformedAttributeError::MissingAttributeName(current_index as u32),
                ));
            }
            //
            // if the attribute ends with an equals sign, it's malformed
            if attr.ends_with('=') {
                return Err(ParseHTMLTypeError::MalformedAttribute(
                    attr.to_string(),
                    MalformedAttributeError::MissingAttributeValue(current_index as u32),
                ));
            }

            // if the attribute has an equal sign, split attribute into key and value
            if let Some((key, value)) = attr.split_once('=') {
                // if the value does
                if !value.starts_with('"') {
                    return Err(ParseHTMLTypeError::MalformedAttribute(
                        value.to_string(),
                        MalformedAttributeError::MissingQuotationMark(current_index as u32),
                    ));
                }

                // if the value ends with a quote, the value is in quotes, so add it to the map
                if value.ends_with('"') {
                    let value = value.strip_prefix('"').unwrap();
                    let value = value.strip_suffix('"').unwrap();
                    attribute_map.insert(key.to_string(), value.to_string());
                    continue;
                }

                // if we are already in quotes, attribute shouldn't start with quotes
                if in_quotes {
                    return Err(ParseHTMLTypeError::MalformedAttribute(
                        current_value_in_quotes,
                        MalformedAttributeError::MissingQuotationMark(current_index as u32),
                    ));
                }

                // if reached this point, the value is not in quotes, so set the current key and value
                current_key = key.to_string();
                in_quotes = true;
                current_value_in_quotes = value[1..].to_string(); // ignore the opening quote
                continue;
            }
            unreachable!() // since this scope is only entered if the attribute contains an equals sign, this is unreachable
        }

        // if the attribute doesn't contain an equals sign, we are in a quote
        // if not, the attribute is malformed
        if !in_quotes {
            return Err(ParseHTMLTypeError::MalformedAttribute(
                attr.to_string(),
                MalformedAttributeError::MissingQuotationMark(current_index as u32),
            ));
        }

        // if the attribute contains a quote, it should be at the end of the attribute
        if attr.contains('"') {
            match attr.strip_prefix('"') {
                // if the attribute ends with a quote, add the current key and value (with the stripped content) to the map
                Some(stripped) => {
                    in_quotes = false;
                    current_value_in_quotes.push(' ');
                    current_value_in_quotes.push_str(stripped);
                    attribute_map.insert(current_key.clone(), current_value_in_quotes.clone());
                    current_key.clear();
                    current_value_in_quotes.clear();
                    continue;
                }
                // if the attribute doesn't end with a quote, it's malformed
                None => {
                    return Err(ParseHTMLTypeError::MalformedAttribute(
                        attr.to_string(),
                        MalformedAttributeError::MissingQuotationMark(current_index as u32),
                    ));
                }
            }
        }

        // if the attribute doesn't contain an equals sign or a quote, add the attribute to the current value
        current_value_in_quotes.push(' ');
        current_value_in_quotes.push_str(attr);
    }

    // if we are still in quotes, the attribute is malformed
    if in_quotes {
        return Err(ParseHTMLTypeError::MalformedAttribute(
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
