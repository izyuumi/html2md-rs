use std::collections::HashMap;

#[derive(Debug)]
enum NodeType {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    P,
    Strong,
    Em,
    A,
    Text,
}
use NodeType::*;

#[derive(Debug)]
pub struct Node {
    tag_name: Option<NodeType>,
    value: Option<String>,
    attributes: Option<HashMap<String, String>>,
    children: Vec<Node>,
}

pub fn parse_html(input: String) -> Node {
    let mut current_index = 0;
    let mut nodes = vec![];
    let input = input.replace("\n", "");
    let mut stack: Vec<Node> = Vec::new();

    while current_index < input.len() {
        let mut rest = &input[current_index..];
        if rest.starts_with('<') {
            let closing_index = rest.find('>').expect("malformed tag");
            let tag_content = &rest[1..closing_index];

            let mut node_name = "";
            let mut attribute_map = None;
            if tag_content.contains(' ') {
                let space_index = tag_content.find(' ').unwrap();
                node_name = &tag_content[..space_index];
                let attributes = &tag_content[space_index..];
                attribute_map = Some(HashMap::new());
                'attribute_loop: for attr in attributes.split(' ') {
                    let mut key_value = attr.split('=');
                    let key = key_value.next().unwrap_or("");
                    if key.is_empty() {
                        continue 'attribute_loop;
                    }
                    let value = key_value.next().unwrap_or("");
                    attribute_map
                        .as_mut()
                        .unwrap()
                        .insert(key.to_string(), value.to_string());
                }
            } else {
                node_name = tag_content;
            }

            let node_type = string_to_node_type(node_name);
            if rest.starts_with("</") {
                let last_node = stack.pop().expect("malformed html");
                if stack.is_empty() {
                    nodes.push(last_node);
                } else {
                    let parent = stack.last_mut().unwrap();
                    parent.children.push(last_node);
                }
            } else {
                let node = Node {
                    tag_name: Some(node_type),
                    value: None,
                    attributes: attribute_map,
                    children: vec![],
                };
                stack.push(node);
            }
            current_index += closing_index + 1;
            continue;
        }
        let next_opening_tag = rest.find('<').unwrap_or(input.len());
        let text = &rest[..next_opening_tag];
        if text.trim().len() == 0 {
            current_index += next_opening_tag;
            continue;
        }
        let parent = stack.last_mut().unwrap();
        parent.children.push(Node {
            tag_name: Some(Text),
            value: Some(text.to_string()),
            attributes: None,
            children: vec![],
        });
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

fn string_to_node_type(input: &str) -> NodeType {
    match input {
        "h1" => H1,
        "h2" => H2,
        "h3" => H3,
        "h4" => H4,
        "h5" => H5,
        "h6" => H6,
        "p" => P,
        "strong" => Strong,
        "em" => Em,
        "a" => A,
        _ => Text,
    }
}

fn main() {
    let my_html = r"<html><body><h1>Hello World</h1>
        <p class='test'>This is a paragraph</p>
        <p>This is <strong>another</strong> paragraph</p>
        </body></html>";
    let parsed = parse_html(my_html.to_string());
    println!("{:#?}", parsed);
}
