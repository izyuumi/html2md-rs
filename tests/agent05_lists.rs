use html2md_rs::{
    structs::{NodeType, ToMdConfig},
    to_md::{safe_from_html_to_md, safe_from_html_to_md_with_config},
};

#[test]
fn empty_unordered_and_ordered_lists_emit_nothing() {
    let markdown = safe_from_html_to_md("<ul></ul><ol></ol>".to_string()).unwrap();

    assert_eq!(markdown, "");
}

#[test]
fn zero_start_numbers_multiple_ordered_items_from_zero() {
    let markdown = safe_from_html_to_md(
        "<ol start=\"0\"><li>zero</li><li>one</li><li>two</li></ol>".to_string(),
    )
    .unwrap();

    assert_eq!(markdown, "0. zero\n1. one\n2. two\n");
}

#[test]
fn negative_start_falls_back_to_one() {
    let markdown = safe_from_html_to_md(
        "<ol start=\"-2\"><li>minus two</li><li>minus one</li></ol>".to_string(),
    )
    .unwrap();

    assert_eq!(markdown, "1. minus two\n2. minus one\n");
}

#[test]
fn overflowing_numeric_start_falls_back_to_one() {
    let markdown = safe_from_html_to_md(
        "<ol start=\"9999999999999999999999999999999999999999\"><li>item</li></ol>".to_string(),
    )
    .unwrap();

    assert_eq!(markdown, "1. item\n");

    let markdown = safe_from_html_to_md(
        "<ol start=\"999999999\"><li>last</li><li>clamped</li></ol>".to_string(),
    )
    .unwrap();
    assert_eq!(markdown, "999999999. last\n999999999. clamped\n");
}

#[test]
fn invalid_text_start_falls_back_to_one() {
    let markdown =
        safe_from_html_to_md("<ol start=\"later\"><li>item</li></ol>".to_string()).unwrap();

    assert_eq!(markdown, "1. item\n");
}

#[test]
fn nested_ordered_and_unordered_lists_indent_two_spaces_per_level() {
    let markdown = safe_from_html_to_md(
        "<ol><li><p>outer</p><ul><li><p>middle</p><ol><li><p>inner</p></li></ol></li></ul></li></ol>"
            .to_string(),
    )
    .unwrap();

    assert_eq!(markdown, "1. outer\n  - middle\n    1. inner\n");
}

#[test]
fn paragraph_children_render_after_list_markers_without_blank_lines() {
    let markdown = safe_from_html_to_md(
        "<ul><li><p>first paragraph</p></li><li><p>second paragraph</p></li></ul>".to_string(),
    )
    .unwrap();

    assert_eq!(markdown, "- first paragraph\n- second paragraph\n");
}

#[test]
fn inline_emphasis_is_preserved_inside_list_items() {
    let markdown = safe_from_html_to_md(
        "<ul><li><em>soft</em> and <strong>bold</strong></li></ul>".to_string(),
    )
    .unwrap();

    assert_eq!(markdown, "- *soft* and **bold**\n");
}

#[test]
fn rendering_config_suppresses_subtrees_inside_nested_lists() {
    let config = ToMdConfig {
        ignore_rendering: vec![NodeType::Em],
    };
    let markdown = safe_from_html_to_md_with_config(
        "<ul><li><p>outer</p><ol><li><p>before<em>hidden<strong>too</strong></em>after</p></li></ol></li></ul>"
            .to_string(),
        &config,
    )
    .unwrap();

    assert_eq!(markdown, "- outer\n  1. beforeafter\n");
}

#[test]
fn mixed_list_siblings_preserve_unicode_and_reset_numbering() {
    let markdown = safe_from_html_to_md(
        "<ol start=\"2\"><li>寿司🍣</li></ol><ul><li>café</li><li>東京</li></ul><ol><li>끝</li></ol>"
            .to_string(),
    )
    .unwrap();

    assert_eq!(markdown, "2. 寿司🍣\n- café\n- 東京\n1. 끝\n");
}
