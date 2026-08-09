use html2md_rs::{
    parser::safe_parse_html,
    structs::NodeType::{Comment, Text, Unknown},
    to_md::{safe_from_html_to_md, to_md},
};

#[test]
fn mixed_case_custom_name_is_normalized_without_losing_unicode_text() {
    let input = "<My-Widget>東京 🚀</MY-WIDGET>".to_string();
    let parsed = safe_parse_html(input.clone()).unwrap();

    assert_eq!(parsed.tag_name, Some(Unknown("my-widget".to_string())));
    assert_eq!(
        safe_from_html_to_md(input).unwrap(),
        "<my-widget>東京 🚀</my-widget>"
    );
}

#[test]
fn custom_attributes_render_in_stable_canonical_order() {
    let markdown = safe_from_html_to_md(
        "<x-card z='last' class='hero' id='main' a='first'></x-card>".to_string(),
    )
    .unwrap();

    assert_eq!(
        markdown,
        "<x-card id=\"main\" class=\"hero\" a=\"first\" z=\"last\"></x-card>"
    );
}

#[test]
fn boolean_and_empty_custom_attributes_remain_distinct() {
    let markdown =
        safe_from_html_to_md("<x-toggle hidden data-empty='' selected></x-toggle>".to_string())
            .unwrap();

    assert_eq!(
        markdown,
        "<x-toggle data-empty=\"\" hidden selected></x-toggle>"
    );
}

#[test]
fn custom_attribute_escaping_is_html_safe_and_unicode_lossless() {
    let markdown =
        safe_from_html_to_md(r#"<x-note data-label='5 < 7 & "東京" > 3'>ok</x-note>"#.to_string())
            .unwrap();

    assert_eq!(
        markdown,
        "<x-note data-label=\"5 &lt; 7 &amp; &quot;東京&quot; &gt; 3\">ok</x-note>"
    );
}

#[test]
fn unknown_node_children_retain_source_order() {
    let parsed = safe_parse_html(
        "<x-row>first<!--middle--><x-cell>second</x-cell>third</x-row>".to_string(),
    )
    .unwrap();

    assert_eq!(parsed.children.len(), 4);
    assert_eq!(parsed.children[0].tag_name, Some(Text));
    assert_eq!(parsed.children[0].value.as_deref(), Some("first"));
    assert_eq!(parsed.children[1].tag_name, Some(Comment));
    assert_eq!(parsed.children[1].value.as_deref(), Some("middle"));
    assert_eq!(
        parsed.children[2].tag_name,
        Some(Unknown("x-cell".to_string()))
    );
    assert_eq!(parsed.children[3].tag_name, Some(Text));
    assert_eq!(parsed.children[3].value.as_deref(), Some("third"));
}

#[test]
fn nested_custom_elements_round_trip_with_boundaries_intact() {
    let markdown = safe_from_html_to_md(
        "<outer-box><middle-box><inner-box>payload</inner-box></middle-box></outer-box>"
            .to_string(),
    )
    .unwrap();

    assert_eq!(
        markdown,
        "<outer-box><middle-box><inner-box>payload</inner-box></middle-box></outer-box>"
    );
}

#[test]
fn self_closing_and_explicit_empty_custom_elements_remain_distinct() {
    let self_closing = safe_parse_html("<x-icon />".to_string()).unwrap();
    let explicit_empty = safe_parse_html("<x-icon></x-icon>".to_string()).unwrap();

    assert!(self_closing.explicitly_self_closing);
    assert!(!explicit_empty.explicitly_self_closing);
    assert_eq!(to_md(self_closing), "<x-icon />");
    assert_eq!(to_md(explicit_empty), "<x-icon></x-icon>");
}

#[test]
fn void_html_element_inside_custom_element_stays_self_closing() {
    let markdown =
        safe_from_html_to_md("<x-picture>before<img src='photo.png'>after</x-picture>".to_string())
            .unwrap();

    assert_eq!(
        markdown,
        "<x-picture>before<img src=\"photo.png\" />after</x-picture>"
    );
}

#[test]
fn custom_element_text_keeps_markdown_and_entity_source_literal() {
    let markdown = safe_from_html_to_md(
        "<x-markup># title *literal* [text](url) &amp; &#42;</x-markup>".to_string(),
    )
    .unwrap();

    assert_eq!(
        markdown,
        "<x-markup># title *literal* [text](url) &amp; &#42;</x-markup>"
    );
}

#[test]
fn supported_children_inside_custom_element_convert_in_source_order() {
    let markdown = safe_from_html_to_md(
        "<x-card><strong>bold</strong> then <em>italic</em></x-card>".to_string(),
    )
    .unwrap();

    assert_eq!(markdown, "<x-card>**bold** then *italic*</x-card>");
}
