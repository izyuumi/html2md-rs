use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
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
    Text,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Node {
    pub tag_name: Option<NodeType>,
    pub value: Option<String>,
    pub attributes: Option<HashMap<String, String>>,
    pub children: Vec<Node>,
}
