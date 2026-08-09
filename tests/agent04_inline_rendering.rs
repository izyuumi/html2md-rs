use html2md_rs::{
    structs::{Node, NodeType},
    to_md::{safe_from_html_to_md, to_md},
};

#[test]
fn heading_levels_three_through_six_use_matching_atx_prefixes() {
    let markdown =
        safe_from_html_to_md("<h3>three</h3><h4>four</h4><h5>five</h5><h6>six</h6>".to_string())
            .unwrap();

    assert_eq!(markdown, "### three\n#### four\n##### five\n###### six\n");
}

#[test]
fn heading_combines_plain_strong_and_emphasized_content() {
    let markdown =
        safe_from_html_to_md("<h4>plain <strong>bold</strong>\n    <em>soft</em></h4>".to_string())
            .unwrap();

    assert_eq!(markdown, "#### plain **bold** *soft*\n");
}

#[test]
fn strong_can_contain_emphasis_without_losing_delimiter_order() {
    let markdown =
        safe_from_html_to_md("<p><strong>bold <em>and italic</em> ending</strong></p>".to_string())
            .unwrap();

    assert_eq!(markdown, "**bold *and italic* ending**\n");
}

#[test]
fn adjacent_rich_paragraphs_end_at_single_newline_boundaries() {
    let markdown = safe_from_html_to_md(
        "<div><p>first <em>one</em></p><p><strong>second</strong> two</p></div>".to_string(),
    )
    .unwrap();

    assert_eq!(markdown, "first *one*\n**second** two\n");
}

#[test]
fn line_and_thematic_breaks_emit_distinct_markdown_separators() {
    let markdown = safe_from_html_to_md("<p>top<br>bottom</p><hr><p>end</p>".to_string()).unwrap();

    assert_eq!(markdown, "top  \nbottom\n***\nend\n");
}

#[test]
fn direct_blockquote_text_receives_quote_prefix() {
    let markdown =
        safe_from_html_to_md("<blockquote>quoted text</blockquote>".to_string()).unwrap();

    assert_eq!(markdown, "> quoted text");
}

#[test]
fn comments_preserve_source_order_inside_parent_and_at_root() {
    let markdown =
        safe_from_html_to_md("lead<!-- root --><p>before<!-- inner -->after</p>".to_string())
            .unwrap();

    assert_eq!(markdown, "lead<!-- root -->before<!-- inner -->after\n");
}

#[test]
fn empty_supported_inline_elements_still_emit_their_delimiters() {
    let markdown =
        safe_from_html_to_md("<p>left<strong></strong>mid<em></em>right</p>".to_string()).unwrap();

    assert_eq!(markdown, "left****mid**right\n");
}

#[test]
fn mixed_inline_content_renders_in_source_order() {
    let markdown = safe_from_html_to_md(
        "<p>plain <strong>bold</strong>, <em>soft</em>, <a href='/go'>link</a>.</p>".to_string(),
    )
    .unwrap();

    assert_eq!(markdown, "plain **bold**, *soft*, [link](/go)\\.\n");

    let markdown = safe_from_html_to_md(
        "<p><code>a</code> <code>b</code> <span>c</span> <input></p>".to_string(),
    )
    .unwrap();
    assert_eq!(markdown, "`a` `b` <span>c</span> <input />\n");
}

#[test]
fn synthetic_root_renders_multiple_children_without_adding_a_wrapper() {
    let root = Node::new(
        None,
        None,
        None,
        None,
        vec![
            Node::new(
                Some(NodeType::Text),
                Some("left".to_string()),
                None,
                None,
                Vec::new(),
            ),
            Node::new(
                Some(NodeType::Em),
                None,
                None,
                None,
                vec![Node::new(
                    Some(NodeType::Text),
                    Some("center".to_string()),
                    None,
                    None,
                    Vec::new(),
                )],
            ),
            Node::new(
                Some(NodeType::Text),
                Some("right".to_string()),
                None,
                None,
                Vec::new(),
            ),
        ],
    );

    assert_eq!(to_md(root), "left*center*right");
}
