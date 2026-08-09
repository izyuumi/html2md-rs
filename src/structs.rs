//! HTML tree and Markdown rendering configuration types.

use std::{collections::HashMap, fmt, fmt::Write as _};

/// Represents the different types of HTML elements that the library supports.
#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub enum NodeType {
    /// Document root (`html`).
    Html,
    /// Document metadata container (`head`).
    Head,
    /// Embedded CSS (`style`).
    Style,
    /// External resource link (`link`).
    Link,
    /// Embedded script (`script`).
    Script,
    /// Metadata (`meta`).
    Meta,
    /// Document title (`title`).
    Title,
    /// Document body (`body`).
    Body,
    /// Level-one heading (`h1`).
    H1,
    /// Level-two heading (`h2`).
    H2,
    /// Level-three heading (`h3`).
    H3,
    /// Level-four heading (`h4`).
    H4,
    /// Level-five heading (`h5`).
    H5,
    /// Level-six heading (`h6`).
    H6,
    /// Paragraph (`p`).
    P,
    /// Generic block container (`div`).
    Div,
    /// Strong importance (`strong`).
    Strong,
    /// Emphasis (`em`).
    Em,
    /// Link (`a`).
    A,
    /// Unordered list (`ul`).
    Ul,
    /// Ordered list (`ol`).
    Ol,
    /// List item (`li`).
    Li,
    /// Preformatted block (`pre`).
    Pre,
    /// Code (`code`).
    Code,
    /// Thematic break (`hr`).
    Hr,
    /// Line break (`br`).
    Br,
    /// Block quotation (`blockquote`).
    Blockquote,
    /// Plain text content.
    #[default]
    Text,
    /// HTML comment.
    Comment,
    /// Unsupported tag, retaining its normalized tag name.
    Unknown(String),
}

impl NodeType {
    /// Returns whether this tag affects descendant list or quote formatting.
    pub fn is_special_tag(&self) -> bool {
        use NodeType::*;
        matches!(self, Blockquote | Ul | Ol)
    }

    /// Converts an HTML tag name into its supported node type.
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

    pub(crate) fn is_phrasing(&self) -> bool {
        matches!(
            self,
            Self::Text
                | Self::Strong
                | Self::Em
                | Self::A
                | Self::Code
                | Self::Br
                | Self::Comment
                | Self::Unknown(_)
        )
    }
}

/// Represents a node in the HTML tree.
#[derive(Default)]
pub struct Node {
    /// Element type, or `None` for a synthetic container.
    pub tag_name: Option<NodeType>,
    /// Text or comment content stored by this node.
    pub value: Option<String>,
    /// Element attributes, when present.
    pub attributes: Option<Attributes>,
    /// Whether the element is explicitly self-closing or has HTML void-element semantics.
    pub self_closing: bool,
    /// Ancestor tags that affect descendant formatting.
    pub within_special_tag: Option<Vec<NodeType>>,
    /// Child nodes in source order.
    pub children: Vec<Node>,
}

impl fmt::Debug for Node {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn indent(formatter: &mut fmt::Formatter<'_>, depth: usize) -> fmt::Result {
            for _ in 0..depth {
                formatter.write_str("    ")?;
            }
            Ok(())
        }

        struct Frame<'a> {
            node: &'a Node,
            next_child: usize,
            opened: bool,
        }

        let pretty = formatter.alternate();
        let mut frames = Vec::with_capacity(32);
        frames.push(Frame {
            node: self,
            next_child: 0,
            opened: false,
        });

        while !frames.is_empty() {
            let depth = frames.len().saturating_sub(1);
            let Some(frame) = frames.last_mut() else {
                break;
            };

            if !frame.opened {
                if pretty {
                    formatter.write_str("Node {\n")?;
                    indent(formatter, depth + 1)?;
                    writeln!(formatter, "tag_name: {:?},", frame.node.tag_name)?;
                    indent(formatter, depth + 1)?;
                    writeln!(formatter, "value: {:?},", frame.node.value)?;
                    indent(formatter, depth + 1)?;
                    writeln!(formatter, "attributes: {:?},", frame.node.attributes)?;
                    indent(formatter, depth + 1)?;
                    writeln!(formatter, "self_closing: {:?},", frame.node.self_closing)?;
                    indent(formatter, depth + 1)?;
                    writeln!(
                        formatter,
                        "within_special_tag: {:?},",
                        frame.node.within_special_tag
                    )?;
                    indent(formatter, depth + 1)?;
                    formatter.write_str("children: [")?;
                    if !frame.node.children.is_empty() {
                        formatter.write_char('\n')?;
                    }
                } else {
                    write!(
                        formatter,
                        "Node {{ tag_name: {:?}, value: {:?}, attributes: {:?}, self_closing: {:?}, within_special_tag: {:?}, children: [",
                        frame.node.tag_name,
                        frame.node.value,
                        frame.node.attributes,
                        frame.node.self_closing,
                        frame.node.within_special_tag
                    )?;
                }
                frame.opened = true;
            }

            let next_child = frame.node.children.get(frame.next_child);
            if let Some(child) = next_child {
                if pretty {
                    indent(formatter, depth + 2)?;
                } else if frame.next_child > 0 {
                    formatter.write_str(", ")?;
                }
                frame.next_child += 1;
                frames.push(Frame {
                    node: child,
                    next_child: 0,
                    opened: false,
                });
                continue;
            }

            if pretty {
                if !frame.node.children.is_empty() {
                    indent(formatter, depth + 1)?;
                }
                formatter.write_str("],\n")?;
                indent(formatter, depth)?;
                formatter.write_char('}')?;
            } else {
                formatter.write_str("] }")?;
            }
            frames.pop();
            if pretty && !frames.is_empty() {
                formatter.write_str(",\n")?;
            }
        }

        Ok(())
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        fn same_node(left: &Node, right: &Node) -> bool {
            left.tag_name == right.tag_name
                && left.value == right.value
                && left.attributes == right.attributes
                && left.self_closing == right.self_closing
                && left.within_special_tag == right.within_special_tag
                && left.children.len() == right.children.len()
        }

        if !same_node(self, other) {
            return false;
        }

        let mut frames = Vec::with_capacity(32);
        frames.push((self, other, 0));

        while let Some((left, right, next_child)) = frames.last_mut() {
            let Some(left_child) = left.children.get(*next_child) else {
                frames.pop();
                continue;
            };
            let Some(right_child) = right.children.get(*next_child) else {
                return false;
            };
            *next_child += 1;

            if !same_node(left_child, right_child) {
                return false;
            }
            frames.push((left_child, right_child, 0));
        }

        true
    }
}

impl Eq for Node {}

impl Clone for Node {
    fn clone(&self) -> Self {
        fn without_children(source: &Node) -> Node {
            Node {
                tag_name: source.tag_name.clone(),
                value: source.value.clone(),
                attributes: source.attributes.clone(),
                self_closing: source.self_closing,
                within_special_tag: source.within_special_tag.clone(),
                children: Vec::with_capacity(source.children.len()),
            }
        }

        struct Frame<'a> {
            source: &'a Node,
            cloned: Node,
            next_child: usize,
        }

        impl<'a> Frame<'a> {
            fn new(source: &'a Node) -> Self {
                Self {
                    source,
                    cloned: without_children(source),
                    next_child: 0,
                }
            }
        }

        let mut frames = Vec::with_capacity(32);
        frames.push(Frame::new(self));

        loop {
            let next_child = {
                let Some(frame) = frames.last_mut() else {
                    return without_children(self);
                };
                frame.source.children.get(frame.next_child).map(|child| {
                    frame.next_child += 1;
                    child
                })
            };

            if let Some(child) = next_child {
                if child.children.is_empty() {
                    let Some(parent) = frames.last_mut() else {
                        return without_children(self);
                    };
                    parent.cloned.children.push(without_children(child));
                } else {
                    frames.push(Frame::new(child));
                }
                continue;
            }

            let Some(completed) = frames.pop().map(|frame| frame.cloned) else {
                return without_children(self);
            };
            if let Some(parent) = frames.last_mut() {
                parent.cloned.children.push(completed);
            } else {
                return completed;
            }
        }
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        let mut nodes = std::mem::take(&mut self.children);
        while let Some(mut node) = nodes.pop() {
            nodes.append(&mut node.children);
        }
    }
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
            let count = within_special_tag
                .iter()
                .filter(|tag| ul_or_ol.contains(tag))
                .count();
            if count > 0 {
                " ".repeat((count - 1) * 2)
            } else {
                String::new()
            }
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
            self_closing: false,
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
            "id" => self.id.as_ref().map(|id| AttributeValues::from(id.clone())),
            "class" => self
                .class
                .as_ref()
                .map(|class| AttributeValues::from(class.clone())),
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

    /// Return the href attribute of the element
    pub fn get_href(&self) -> Option<String> {
        self.get("href").and_then(|value| match value {
            AttributeValues::String(href) => Some(href),
            _ => None,
        })
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

impl AttributeValues {
    /// Creates a new `AttributeValues` instance from a value that can be converted into `AttributeValues`.
    ///
    /// This function uses the `Into` trait to allow for flexible type conversion.
    ///
    /// # Examples
    ///
    /// ```
    /// # use html2md_rs::structs::AttributeValues;
    /// let string_value = AttributeValues::from("Hello, world!");
    /// let bool_value = AttributeValues::from(true);
    /// let number_value = AttributeValues::from(42);
    /// ```
    pub fn from<T: Into<AttributeValues>>(value: T) -> Self {
        value.into()
    }
}

impl From<String> for AttributeValues {
    /// Converts a `String` into an `AttributeValues::String`.
    fn from(value: String) -> Self {
        AttributeValues::String(value)
    }
}

impl From<&str> for AttributeValues {
    /// Converts a string slice (`&str`) into an `AttributeValues::String`.
    fn from(value: &str) -> Self {
        AttributeValues::String(value.to_string())
    }
}

impl From<bool> for AttributeValues {
    /// Converts a `bool` into an `AttributeValues::Bool`.
    fn from(value: bool) -> Self {
        AttributeValues::Bool(value)
    }
}

impl From<i32> for AttributeValues {
    /// Converts an `i32` into an `AttributeValues::Number`.
    fn from(value: i32) -> Self {
        AttributeValues::Number(value)
    }
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

/// Controls how nodes are rendered to Markdown.
#[derive(Debug, Default)]
pub struct ToMdConfig {
    /// Node types whose complete subtrees should be omitted from Markdown output.
    pub ignore_rendering: Vec<NodeType>,
}
