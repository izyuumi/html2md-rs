# html2md-rs

Parses HTML and converts it to Markdown.

## Usage

```rust
use html2md_rs::to_md::safe_from_html_to_md;

let html = "<h1>Hello, World!</h1>".to_string();
let markdown = safe_from_html_to_md(html)?;

assert_eq!(markdown, "# Hello, World\\!\n");
# Ok::<(), html2md_rs::parser::ParseHTMLError>(())
```

## Markdown Convention

Output targets the [CommonMark specification](https://spec.commonmark.org/0.31.2/).

## Supported HTML tags

See [`NodeType`](https://docs.rs/html2md-rs/latest/html2md_rs/structs/enum.NodeType.html) for supported tags. Unsupported tags are retained as `NodeType::Unknown(String)`.

## Scope and limitations

This crate implements a pragmatic HTML subset, not a browser-grade HTML5 parser.

- Recognized malformed input is returned as `ParseHTMLError` by the safe APIs instead of panicking.
- HTML5 implied end-tag rules are not implemented; explicit source nesting determines the tree.
- Supported named character references are `amp`, `lt`, `gt`, `quot`, `apos`, and `nbsp`; valid decimal and hexadecimal numeric references are also decoded. Unknown entities are preserved. Entity-derived `<` and `&` are Markdown-escaped to preserve literal text.
- Markdown-significant punctuation in ordinary text is escaped. Text inside code and unknown elements is kept literal.

## License

Licensed under the MIT License. See [LICENSE](./LICENSE).
