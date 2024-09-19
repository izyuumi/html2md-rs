// }

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
    Comment,
    Unknown(String),
}

impl NodeType {
    pub fn is_special_tag(&self) -> bool {
        use NodeType::*;
        matches!(self, Blockquote | Ul | Ol)
    }

    pub fn from_tag_str(input: &str) -> Self {
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
    pub attributes: Option<Attributes>,
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
        attributes: Option<Attributes>,
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

/// Represents the Attributes of an HTML element.
#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Attributes {
    pub(crate) id: Option<String>,
    pub(crate) class: Option<String>,
    pub(crate) attributes: HashMap<String, AttributeValues>,
}

impl Attributes {
    /// Creates a new Attributes struct from id, class and attributes
    pub fn new() -> Self {
        Attributes {
            id: None,
            class: None,
            attributes: HashMap::new(),
        }
    }

    /// Returns the attribute value of the key passed in
    pub fn get(&self, key: &str) -> Option<AttributeValues> {
        match key {
            "id" => self
                .id
                .as_ref()
                .map(|id| AttributeValues::String(id.clone())),
            "class" => self
                .class
                .as_ref()
                .map(|class| AttributeValues::String(class.clone())),
            _ => self.attributes.get(key).cloned(),
        }
    }

    /// Returns the id attribute of the element
    pub fn get_id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    /// Returns the class attribute of the element
    pub fn get_class(&self) -> Option<&String> {
        self.class.as_ref()
    }

    /// Returns the attributes of the element
    pub fn contains(&self, key: &str) -> bool {
        match key {
            "id" => self.id.is_some(),
            "class" => self.class.is_some(),
            _ => self.attributes.contains_key(key),
        }
    }

    /// Inserts a new attribute into the element with the key and value passed in
    pub fn insert(&mut self, key: String, value: AttributeValues) {
        match key.as_str() {
            "id" => self.id = Some(value.to_string()),
            "class" => self.class = Some(value.to_string()),
            _ => {
                self.attributes.insert(key, value);
            }
        }
    }

    /// Returns whether the element attributes are empty
    pub fn is_empty(&self) -> bool {
        self.id.is_none() && self.class.is_none() && self.attributes.is_empty()
    }

    /// Inserts attributes into the element from a tuple vector
    pub fn from(vec: Vec<(String, AttributeValues)>) -> Self {
        let mut attributes = Attributes::new();
        for (key, value) in vec {
            attributes.insert(key, value);
        }
        attributes
    }
}

/// Represents the different types of attribute values that the library supports.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AttributeValues {
    /// Represents a string attribute value.
    String(String),
    /// Represents a boolean attribute value.
    Bool(bool),
    /// Represents an integer attribute value.
    Number(i32),
}

impl std::fmt::Display for AttributeValues {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AttributeValues::String(value) => write!(f, "{}", value),
            AttributeValues::Bool(value) => write!(f, "{}", value),
            AttributeValues::Number(value) => write!(f, "{}", value),
        }
    }
}

#[derive(Debug, Default)]
pub struct ToMdConfig {
    pub ignore_rendering: Vec<NodeType>,
}
