#[cfg(test)]
mod to_md_tests {
    use html2md_rs::{
        parser::safe_parse_html,
        structs::{Node, NodeType, ToMdConfig},
        to_md::{safe_from_html_to_md, safe_from_html_to_md_with_config},
    };

    pub trait PrintNode {
        fn print_node(&self);
    }

    impl PrintNode for Node {
        fn print_node(&self) {
            println!("{:#?}", self);
        }
    }
    trait StringPrintNode {
        fn print_node(&self);
    }

    impl StringPrintNode for String {
        fn print_node(&self) {
            match safe_parse_html(self.clone()) {
                Ok(node) => node.print_node(),
                Err(e) => println!("Error: {}", e),
            }
        }
    }

    #[test]
    fn simple_paragraph_with_text() {
        let input = "<p>hello</p>".to_string();
        let expected = "hello\n".to_string();
        assert_eq!(safe_from_html_to_md(input).unwrap(), expected);
    }

    #[test]
    fn multiple_headers() {
        let input = "<h1>hello</h1><h2>world</h2>".to_string();
        let expected = "# hello\n## world\n".to_string();
        assert_eq!(safe_from_html_to_md(input).unwrap(), expected);
    }

    #[test]
    fn paragraph_with_strong() {
        let input = "<p>hello <strong>world</strong></p>".to_string();
        let expected = "hello **world**\n".to_string();
        assert_eq!(safe_from_html_to_md(input).unwrap(), expected);
    }

    #[test]
    fn strong_header() {
        let input = "<h1><strong>hello</strong></h1>".to_string();
        let expected = "# **hello**\n".to_string();
        assert_eq!(safe_from_html_to_md(input).unwrap(), expected);
    }

    #[test]
    fn unordered_list() {
        let input = "<ul><li>hello</li><li>world</li></ul>".to_string();
        let expected = "- hello\n- world\n".to_string();
        assert_eq!(safe_from_html_to_md(input).unwrap(), expected);
    }

    #[test]
    fn ordered_list() {
        let input = "<ol><li>hello</li><li>world</li></ol>".to_string();
        let expected = "1. hello\n2. world\n".to_string();
        assert_eq!(safe_from_html_to_md(input).unwrap(), expected);
    }

    #[test]
    fn multiple_paragraphs() {
        let input = "<p>hello</p><p>world</p>".to_string();
        let expected = "hello\nworld\n".to_string();
        assert_eq!(safe_from_html_to_md(input).unwrap(), expected);
    }

    #[test]
    fn multiple_paragraphs_with_empty_paragraph() {
        let input = "<p>hello</p><p></p><p>world</p>".to_string();
        let expected = "hello\nworld\n".to_string();
        assert_eq!(safe_from_html_to_md(input).unwrap(), expected);
    }

    #[test]
    fn header_and_paragraph() {
        let input = "<h1>hello</h1><p></p><p>world</p>".to_string();
        let expected = "# hello\nworld\n".to_string();
        assert_eq!(safe_from_html_to_md(input).unwrap(), expected);
    }

    #[test]
    fn paragraph_with_link() {
        let input = "<p><a href=\"https://example.com\">hello</a></p>".to_string();
        let expected = "[hello](https://example.com)\n".to_string();
        assert_eq!(safe_from_html_to_md(input).unwrap(), expected);
    }

    #[test]
    fn code_block() {
        let input = "<pre><code class=\"language-rust\">
let x: i32 = 123;
let y: i32 = 456;
let z = x + y;
println!(\"{}\", z);
</code></pre>"
            .to_string();
        let expected = "```rust
let x: i32 = 123;
let y: i32 = 456;
let z = x + y;
println!(\"{}\", z);
```\n"
            .to_string();
        assert_eq!(safe_from_html_to_md(input).unwrap(), expected);
    }

    #[test]
    fn line_break() {
        let input = "<p>hello<br />world</p>".to_string();
        let expected = "hello  \nworld\n".to_string();
        assert_eq!(safe_from_html_to_md(input).unwrap(), expected);
    }

    #[test]
    fn blockquote() {
        let input = "<blockquote>
<p>hello</p>
<p>world</p>
<p>from</p>
<p>blockquote</p>
</blockquote>"
            .to_string();
        input.print_node();
        let expected = "> hello\n> world\n> from\n> blockquote\n".to_string();
        assert_eq!(safe_from_html_to_md(input).unwrap(), expected);
    }

    #[test]
    fn unknown_tag() {
        let input = "<unknown>hello</unknown>".to_string();
        let expected = "<unknown>hello</unknown>".to_string();
        assert_eq!(safe_from_html_to_md(input).unwrap(), expected);
    }

    #[test]
    fn list_in_list() {
        let input = "
<ul>
  <li>
  	<p>abc</p>
  	<ul>
  	  <li>
  	    <p>abc</p>
  	    <ol>
  	      <li>
  	        <p>123</p>
  	      </li>
  	    </ol>
  	  </li>
  	</ul>
  </li>
</ul>"
            .to_string();
        let expected = "- abc\n  - abc\n    1. 123\n".to_string();
        assert_eq!(safe_from_html_to_md(input).unwrap(), expected);
    }

    #[test]
    fn ol_start_attribute() {
        let input = "<ol start=\"3\"><li><p>hello</p></li><li><p>world</p></li></ol>".to_string();
        let expected = "3. hello\n4. world\n".to_string();
        assert_eq!(safe_from_html_to_md(input).unwrap(), expected);
    }

    #[test]
    fn comment() {
        let input = "<!-- hello -->".to_string();
        let expected = "<!-- hello -->".to_string();
        assert_eq!(safe_from_html_to_md(input).unwrap(), expected);
    }

    #[test]
    fn comments_in_p() {
        let input = "<p><!-- hello --></p>".to_string();
        let expected = "<!-- hello -->".to_string();
        assert_eq!(safe_from_html_to_md(input).unwrap(), expected);
    }

    #[test]
    fn unclosed_tag() {
        let input = "<p>hello".to_string();
        let expected = "hello\n".to_string();
        assert_eq!(safe_from_html_to_md(input).unwrap(), expected);
    }

    #[test]
    fn unclosed_tag_2() {
        let input = "<html><head><title>Test</title></head><body><p>hello</p>".to_string();
        input.print_node();
        let expected = "hello\n".to_string();
        assert_eq!(safe_from_html_to_md(input).unwrap(), expected);
    }

    #[test]
    fn ignore_rendering() {
        let input =
            "<div><span>don't render this</span><p>this should be rendered</p><div>render this</div></div>".to_string();
        let config = ToMdConfig {
            ignore_rendering: vec![NodeType::Unknown("span".to_string())],
        };
        let expected = "this should be rendered\nrender this".to_string();
        assert_eq!(
            safe_from_html_to_md_with_config(input, &config).unwrap(),
            expected
        );
    }
}
