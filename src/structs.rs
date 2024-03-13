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
    Pre,
    Code,
    Hr,
    Br,
    Blockquote,
    Text,
    Unknown(String),
}

impl NodeType {
    pub fn is_special_tag(&self) -> bool {
        use NodeType::*;
        match self {
            Blockquote => true,
            _ => false,
        }
    }

    pub fn from_str(input: &str) -> Self {
        use NodeType::*;
        match input.to_lowercase().as_str() {
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
            "blockquote" => Blockquote,
            unknown => Unknown(unknown.to_string()),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Node {
    pub tag_name: Option<NodeType>,
    pub value: Option<String>,
    pub attributes: Option<HashMap<String, String>>,
    pub within_special_tag: Option<Vec<NodeType>>,
    pub children: Vec<Node>,
}

pub trait PrintNode {
    fn print_node(&self);
}

impl PrintNode for Node {
    fn print_node(&self) {
        println!("{:#?}", self);
    }
}
