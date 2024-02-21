use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum NodeType {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    P,
    Div,
    Strong,
    Em,
    A,
    Ul,
    Ol,
    Li,
    Text,
}

impl NodeType {
    pub fn from_str(input: &str) -> NodeType {
        match input {
            "h1" => NodeType::H1,
            "h2" => NodeType::H2,
            "h3" => NodeType::H3,
            "h4" => NodeType::H4,
            "h5" => NodeType::H5,
            "h6" => NodeType::H6,
            "p" => NodeType::P,
            "div" => NodeType::Div,
            "strong" => NodeType::Strong,
            "em" => NodeType::Em,
            "a" => NodeType::A,
            "ul" => NodeType::Ul,
            "ol" => NodeType::Ol,
            "li" => NodeType::Li,
            _ => NodeType::Text,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Node {
    pub tag_name: Option<NodeType>,
    pub value: Option<String>,
    pub attributes: Option<HashMap<String, String>>,
    pub children: Vec<Node>,
}
