use crate::structs::{
    Node,
    NodeType::{self, *},
};
use std::collections::HashMap;

pub fn parse_html(input: String) -> Node {
    let mut current_index = 0;
    let mut nodes = Vec::new();
    let input = input.replace("\n", "");
    let mut stack: Vec<Node> = Vec::new();

    while current_index < input.len() {
        let rest = &input[current_index..];
        if rest.starts_with('<') {
            let mut closing_index = rest.find('>').expect("malformed tag");
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
                attribute_map = Some(HashMap::new());
                let mut in_quotes = false;
                let mut current_key = String::new();
                let mut current_value_in_quotes = String::new();
                'attribute_loop: for attr in attributes.split_whitespace() {
                    if attr.contains('=') {
                        if attr.starts_with('=') || attr.ends_with('=') {
                            panic!(
                                "malformed html, attribute key missing. starting or ending with \"=\": {}",
                                attr
                            );
                        }
                        if let Some(key_value) = attr.split_once('=') {
                            if key_value.1.starts_with('"') {
                                if key_value.1.ends_with('"') {
                                    attribute_map.as_mut().unwrap().insert(
                                        key_value.0.to_string(),
                                        key_value.1[1..key_value.1.len() - 1].to_string(),
                                    );
                                    continue 'attribute_loop;
                                }
                                in_quotes = true;
                                (current_key, current_value_in_quotes) =
                                    (key_value.0.to_string(), key_value.1[1..].to_string());
                                continue 'attribute_loop;
                            }
                            attribute_map
                                .as_mut()
                                .unwrap()
                                .insert(key_value.0.to_string(), key_value.1.to_string());
                        }
                        continue 'attribute_loop;
                    }
                    if !in_quotes {
                        panic!("malformed html, attribute value missing: {}", attr);
                    }
                    if attr.contains('"') {
                        if attr.ends_with('"') {
                            in_quotes = false;
                            current_value_in_quotes.push_str(&attr[..attr.len() - 1]);
                            attribute_map
                                .as_mut()
                                .unwrap()
                                .insert(current_key.clone(), current_value_in_quotes.clone());
                            current_key.clear();
                            current_value_in_quotes.clear();
                            continue 'attribute_loop;
                        }
                        panic!("malformed html, attribute value contains quotes: {}", attr);
                    }
                }
                if in_quotes {
                    panic!("malformed html, missing closing quote for attribute value");
                }
            } else {
                node_name = tag_content;
            }

            let node_type = NodeType::from_str(node_name);
            if rest.starts_with("</") {
                let last_node = stack.pop().expect("malformed html");
                if stack.is_empty() {
                    nodes.push(last_node);
                } else {
                    let parent = stack.last_mut().unwrap();
                    parent.children.push(last_node);
                }
            } else if self_closing {
                let node = Node {
                    tag_name: Some(node_type),
                    value: None,
                    attributes: if attribute_map.clone().unwrap_or_default().is_empty() {
                        None
                    } else {
                        attribute_map
                    },
                    children: Vec::new(),
                };
                if let Some(parent) = stack.last_mut() {
                    parent.children.push(node);
                } else {
                    nodes.push(node);
                }
                current_index += closing_index + 2;
            } else {
                let node = Node {
                    tag_name: Some(node_type),
                    value: None,
                    attributes: if attribute_map.clone().unwrap_or_default().is_empty() {
                        None
                    } else {
                        attribute_map
                    },
                    children: Vec::new(),
                };
                stack.push(node);
            }
            current_index += closing_index + 1;
            continue;
        }
        let next_opening_tag = rest.find('<').unwrap_or(rest.len());
        let text = &rest[..next_opening_tag];
        if text.trim().len() == 0 {
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
        return nodes.remove(0);
    }

    Node {
        tag_name: None,
        value: None,
        attributes: None,
        children: nodes,
    }
}
