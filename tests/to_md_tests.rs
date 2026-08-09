#[cfg(test)]
mod to_md_tests {
    use html2md_rs::{
        parser::safe_parse_html,
        structs::{Node, NodeType, ToMdConfig},
        to_md::{safe_from_html_to_md, safe_from_html_to_md_with_config, to_md},
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
    fn code_block_no_leading_newline() {
        let input = "<pre><code class=\"language-rust\">println!(\"hi\");</code></pre>".to_string();
        let expected = "```rust\nprintln!(\"hi\");\n```\n".to_string();
        assert_eq!(safe_from_html_to_md(input).unwrap(), expected);
    }

    #[test]
    fn ordinary_text_decodes_entities_and_escapes_markdown() {
        let input = "<p>&amp; &lt; &gt; &quot; &apos; &nbsp; &#35; &#x21; * _ [ ]</p>".to_string();
        let expected = "\\& \\< \\> \" ' \u{a0} \\# \\! \\* \\_ \\[ \\]\n".to_string();
        assert_eq!(safe_from_html_to_md(input).unwrap(), expected);
    }

    #[test]
    fn decoded_entities_cannot_become_html_or_decode_twice_in_markdown() {
        let input = "<p>&lt;script&gt; &amp;lt;</p>".to_string();
        let expected = "\\<script\\> \\&lt;\n".to_string();
        assert_eq!(safe_from_html_to_md(input).unwrap(), expected);
    }

    #[test]
    fn code_and_unknown_html_text_stay_raw() {
        let input = "<pre><code>&amp; *code*</code></pre><widget>&amp; *html*</widget>".to_string();
        let expected = "```\n&amp; *code*\n```\n<widget>&amp; *html*</widget>".to_string();
        assert_eq!(safe_from_html_to_md(input).unwrap(), expected);
    }

    #[test]
    fn code_block_without_language_has_one_newline_before_closing_fence() {
        let input = "<pre><code>abc\n\n</code></pre>".to_string();
        let expected = "```\nabc\n```\n".to_string();
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
    fn unknown_tag_attribute_values_are_valid_html() {
        let input = r#"<widget title='a"b & c'>x</widget>"#.to_string();
        let expected = "<widget title=\"a&quot;b &amp; c\">x</widget>".to_string();
        assert_eq!(safe_from_html_to_md(input).unwrap(), expected);
    }

    #[test]
    fn unknown_tag_attribute_entities_are_decoded_once_then_escaped() {
        let input = "<widget title='a&quot;b &amp; c'>x</widget>".to_string();
        let expected = "<widget title=\"a&quot;b &amp; c\">x</widget>".to_string();
        assert_eq!(safe_from_html_to_md(input).unwrap(), expected);

        let input = "<widget title='&amp;quot;'>x</widget>".to_string();
        let expected = "<widget title=\"&amp;quot;\">x</widget>".to_string();
        assert_eq!(safe_from_html_to_md(input).unwrap(), expected);
    }

    #[test]
    fn childless_custom_tag_stays_paired() {
        let input = "<widget></widget>".to_string();
        let expected = "<widget></widget>".to_string();
        assert_eq!(safe_from_html_to_md(input).unwrap(), expected);
    }

    #[test]
    fn childless_html_void_tag_self_closes_and_preserves_attributes() {
        let input = "<img src=\"image.png\" />".to_string();
        let expected = "<img src=\"image.png\" />".to_string();
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
    fn ordered_list_start_saturates_instead_of_overflowing() {
        let input = "<ol start=\"999999999\"><li>one</li><li>two</li></ol>".to_string();
        let expected = "999999999. one\n999999999. two\n".to_string();
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

    #[test]
    fn ignore_rendering_config_applies_inside_lists() {
        let input = "<ul><li><strong>hidden</strong>shown</li></ul><ol><li><strong>hidden</strong>shown</li></ol>".to_string();
        let config = ToMdConfig {
            ignore_rendering: vec![NodeType::Strong],
        };
        let expected = "- shown\n1. shown\n".to_string();

        assert_eq!(
            safe_from_html_to_md_with_config(input, &config).unwrap(),
            expected
        );
    }

    #[test]
    fn script_and_style_children_do_not_render() {
        let input =
            "<p>before</p><script>script text</script><style>style text</style><p>after</p>"
                .to_string();
        let expected = "before\nafter\n".to_string();
        assert_eq!(safe_from_html_to_md(input).unwrap(), expected);
    }

    #[test]
    fn deeply_nested_nodes_render_without_recursion() {
        let mut node = Node::new(
            Some(NodeType::Text),
            Some("deep".to_string()),
            None,
            None,
            Vec::new(),
        );

        for _ in 0..10_000 {
            node = Node::new(Some(NodeType::Div), None, None, None, vec![node]);
        }

        assert_eq!(to_md(node), "deep");
    }
}
