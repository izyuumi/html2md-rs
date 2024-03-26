//! This module contains enums and structs used in the library.

use std::collections::HashMap;

/// Represents the different types of HTML elements that the library supports.
#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub enum NodeType {
    Html,
    Head,
    Style,
    Link,
    Script,
    Meta,
    Title,
    Body,
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
    #[default]
    Text,
    Unknown(String),
}

impl NodeType {
    pub fn is_special_tag(&self) -> bool {
        use NodeType::*;
        match self {
            Blockquote | Ul | Ol => true,
            _ => false,
        }
    }

    pub fn from_str(input: &str) -> Self {
        use NodeType::*;
        match input.to_lowercase().as_str() {
            "html" => Html,
            "head" => Head,
            "style" => Style,
            "link" => Link,
            "script" => Script,
            "meta" => Meta,
            "title" => Title,
            "body" => Body,
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

/// Represents a node in the HTML tree.
#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Node {
    pub tag_name: Option<NodeType>,
    pub value: Option<String>,
    pub attributes: Option<HashMap<String, String>>,
    pub within_special_tag: Option<Vec<NodeType>>,
    pub children: Vec<Node>,
}

impl Node {
    /// Checks whether the node is within any of the special tags passed in
    pub fn is_in_special_tag(&self, tags: &[NodeType]) -> bool {
        if let Some(within_special_tag) = &self.within_special_tag {
            within_special_tag.iter().any(|tag| tags.contains(tag))
        } else {
            false
        }
    }

    /// Returns the leading spaces if there is any
    /// This is used to format the output of the unordered and ordered lists
    pub fn leading_spaces(&self) -> String {
        let ul_or_ol = &[NodeType::Ul, NodeType::Ol];
        if let Some(within_special_tag) = &self.within_special_tag {
            " ".repeat(
                (within_special_tag
                    .iter()
                    .filter(|tag| ul_or_ol.contains(tag))
                    .count()
                    - 1)
                    * 2,
            )
        } else {
            String::new()
        }
    }

    /// Creates a new Node from tag_name, value, attributes, within_special_tag and children
    pub fn new(
        tag_name: Option<NodeType>,
        value: Option<String>,
        attributes: Option<HashMap<String, String>>,
        within_special_tag: Option<Vec<NodeType>>,
        children: Vec<Node>,
    ) -> Self {
        Node {
            tag_name,
            value,
            attributes,
            within_special_tag,
            children,
        }
    }
}
