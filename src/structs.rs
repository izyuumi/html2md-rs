use std::{collections::HashMap, str::FromStr};

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
    Pre,
    Code,
    Hr,
    Br,
    Text,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseNodeTypeError;

impl FromStr for NodeType {
    type Err = ParseNodeTypeError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        use NodeType::*;
        let node_type = match input.to_lowercase().as_str() {
            "h1" => H1,
            "h2" => H2,
            "h3" => H3,
            "h4" => H4,
            "h5" => H5,
            "h6" => H6,
            "p" => P,
            "div" => Div,
            "strong" => Strong,
            "em" => Em,
            "a" => A,
            "ul" => Ul,
            "ol" => Ol,
            "li" => Li,
            "pre" => Pre,
            "code" => Code,
            "hr" => Hr,
            "br" => Br,
            _ => return Err(ParseNodeTypeError),
        };
        Ok(node_type)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Node {
    pub tag_name: Option<NodeType>,
    pub value: Option<String>,
    pub attributes: Option<HashMap<String, String>>,
    pub children: Vec<Node>,
}
