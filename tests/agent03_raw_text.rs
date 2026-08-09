use html2md_rs::{
    parser::{safe_parse_html, MalformedTagError, ParseHTMLError},
    structs::{
        AttributeValues,
        NodeType::{Comment, Script, Style, Text, Unknown, P},
    },
    to_md::safe_from_html_to_md,
};

#[test]
fn empty_script_body_is_one_empty_raw_text_node() {
    let parsed = safe_parse_html("<script></script>".to_string()).unwrap();

    assert_eq!(parsed.tag_name, Some(Script));
    assert_eq!(parsed.children.len(), 1);
    assert_eq!(parsed.children[0].tag_name, Some(Text));
    assert_eq!(parsed.children[0].value.as_deref(), Some(""));
}

#[test]
fn raw_text_body_preserves_all_edge_whitespace() {
    let body = "\t \r\nfirst\nlast \u{c}\t";
    let parsed = safe_parse_html(format!("<style>{body}</style>")).unwrap();

    assert_eq!(parsed.tag_name, Some(Style));
    assert_eq!(parsed.children[0].value.as_deref(), Some(body));
}

#[test]
fn raw_text_opening_attributes_are_parsed_before_literal_body() {
    let parsed = safe_parse_html(
        "<script type='application/ld+json' async>{\"tag\":\"<x>\"}</script>".to_string(),
    )
    .unwrap();
    let attributes = parsed.attributes.as_ref().unwrap();

    assert_eq!(
        attributes.get("type"),
        Some(AttributeValues::String("application/ld+json".to_string()))
    );
    assert_eq!(attributes.get("async"), Some(AttributeValues::Bool(true)));
    assert_eq!(
        parsed.children[0].value.as_deref(),
        Some("{\"tag\":\"<x>\"}")
    );
}

#[test]
fn self_closing_textarea_needs_no_raw_text_terminator() {
    let parsed = safe_parse_html("<textarea />".to_string()).unwrap();

    assert_eq!(parsed.tag_name, Some(Unknown("textarea".to_string())));
    assert!(parsed.explicitly_self_closing);
    assert!(parsed.children.is_empty());
    assert_eq!(
        safe_from_html_to_md("<textarea />".to_string()).unwrap(),
        "<textarea />"
    );
}

#[test]
fn raw_text_close_followed_by_comment_preserves_sibling_order() {
    let parsed =
        safe_parse_html("<script>body</script><!--boundary--><p>after</p>".to_string()).unwrap();

    assert_eq!(parsed.children.len(), 3);
    assert_eq!(parsed.children[0].tag_name, Some(Script));
    assert_eq!(parsed.children[1].tag_name, Some(Comment));
    assert_eq!(parsed.children[1].value.as_deref(), Some("boundary"));
    assert_eq!(parsed.children[2].tag_name, Some(P));
}

#[test]
fn raw_text_closing_requires_exact_name_boundary() {
    const BODY: &str = "a</script-foo>b</script:foo>c";
    let parsed = safe_parse_html(format!("<script>{BODY}</script>")).unwrap();

    assert_eq!(parsed.children[0].value.as_deref(), Some(BODY));
}

#[test]
fn mixed_case_raw_text_opening_is_recognized_and_normalized() {
    let parsed = safe_parse_html("<TeXtArEa><b>literal</b></textarea>".to_string()).unwrap();

    assert_eq!(parsed.tag_name, Some(Unknown("textarea".to_string())));
    assert_eq!(parsed.children.len(), 1);
    assert_eq!(parsed.children[0].tag_name, Some(Text));
    assert_eq!(parsed.children[0].value.as_deref(), Some("<b>literal</b>"));
}

#[test]
fn textarea_unknown_element_round_trips_literal_body() {
    let markdown = safe_from_html_to_md(
        "<textarea>*literal* &amp; <b>still literal</b></textarea>".to_string(),
    )
    .unwrap();

    assert_eq!(
        markdown,
        "<textarea>*literal* &amp; <b>still literal</b></textarea>"
    );
}

#[test]
fn unterminated_textarea_raw_text_returns_structured_error() {
    let input = "<textarea>literal <b>tag</b>".to_string();

    assert_eq!(
        safe_parse_html(input.clone()),
        Err(ParseHTMLError::MalformedTag(
            input,
            MalformedTagError::MissingClosingBracket(0)
        ))
    );
}

#[test]
fn raw_text_body_preserves_comment_and_cdata_markers_literally() {
    const BODY: &str = "<!--not a node--><![CDATA[<b>still text</b>]]>";
    let parsed = safe_parse_html(format!("<style>{BODY}</style>")).unwrap();

    assert_eq!(parsed.tag_name, Some(Style));
    assert_eq!(parsed.children.len(), 1);
    assert_eq!(parsed.children[0].tag_name, Some(Text));
    assert_eq!(parsed.children[0].value.as_deref(), Some(BODY));
}
