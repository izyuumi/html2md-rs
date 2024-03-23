//! A library for safely parsing HTML and converting it to Markdown.
//!
//!
//! ## Example
//!
//! ```rust
//! use html2md_rs::to_md::from_html_to_md;
//!
//! let html = "<h1>Hello World</h1>".to_string();
//! let parsed = from_html_to_md(html);
//!
//! assert_eq!(parsed, "# Hello World\n");
//! ```
//!
//! ## Supported HTML Elements
//!
//! The list of supported HTML elements is in the [structs::NodeType](structs/enum.NodeType.html) enum.
//!
//! ## HTML Attributes
//!
//! By default, the library parses all attributes of an HTML element as a HashMap.
//!
//! ## Markdown Convention
//!
//! This library follows the [CommonMark Spec](https://spec.commonmark.org/0.31.2/).
//!
//! ## License
//!
//! This library is licensed under the MIT license. Check the [GitHub repository](https://github.com/izyuumi/html2md-rs) for more information.

pub mod parser;
pub mod structs;
pub mod to_md;
