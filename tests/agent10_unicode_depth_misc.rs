use html2md_rs::{
    parser::{safe_parse_html, MalformedTagError, ParseHTMLError},
    structs::{Node, NodeType},
    to_md::{safe_from_html_to_md, to_md},
};

#[test]
fn utf8_encoding_length_boundaries_survive_parse_and_render() {
    let text = "\u{7f}\u{80}\u{7ff}\u{800}\u{ffff}\u{10000}\u{10ffff}";
    let input = format!("<p>{text}</p>");
    let parsed = safe_parse_html(input.clone()).unwrap();

    assert_eq!(parsed.children[0].value.as_deref(), Some(text));
    assert_eq!(safe_from_html_to_md(input).unwrap(), format!("{text}\n"));
}

#[test]
fn combining_marks_variation_selectors_and_zwj_sequences_are_not_normalized() {
    let text =
        "café cafe\u{301} ☕\u{fe0f} 👩\u{1f3fd}\u{200d}\u{1f4bb} 🏳\u{fe0f}\u{200d}\u{1f308}";

    assert_eq!(
        safe_from_html_to_md(format!("<p>{text}</p>")).unwrap(),
        format!("{text}\n")
    );
}

#[test]
fn mixed_scripts_and_emoji_preserve_order_across_inline_boundaries() {
    let input = "<p>日本語<strong>English 😀</strong>العربية<em>हिन्दी</em>한국어</p>";

    assert_eq!(
        safe_from_html_to_md(input.to_string()).unwrap(),
        "日本語**English 😀**العربية*हिन्दी*한국어\n"
    );
}

#[test]
fn deeply_nested_inline_elements_parse_and_render_iteratively() {
    const DEPTH: usize = 1_024;
    let input = format!(
        "{}深🌊{}",
        "<strong>".repeat(DEPTH),
        "</strong>".repeat(DEPTH)
    );
    let expected = format!("{}深🌊{}", "**".repeat(DEPTH), "**".repeat(DEPTH));

    assert_eq!(safe_from_html_to_md(input).unwrap(), expected);
}

#[test]
fn wide_paragraph_sibling_tree_preserves_every_child_in_source_order() {
    const WIDTH: usize = 512;
    let mut input = String::new();
    let mut expected = String::new();
    for index in 0..WIDTH {
        input.push_str(&format!("<p>行{index}🙂</p>"));
        expected.push_str(&format!("行{index}🙂\n"));
    }

    assert_eq!(safe_from_html_to_md(input).unwrap(), expected);
}

#[test]
fn top_level_text_blocks_comment_and_emoji_keep_source_order() {
    let input = "始🌱<p>段落</p><!--注--><h2>見出し</h2>終🌙";

    assert_eq!(
        safe_from_html_to_md(input.to_string()).unwrap(),
        "始🌱段落\n<!--注-->## 見出し\n終🌙"
    );
}

#[test]
fn empty_input_has_an_empty_synthetic_root_and_empty_markdown() {
    let parsed = safe_parse_html(String::new()).unwrap();

    assert_eq!(parsed, Node::default());
    assert_eq!(to_md(parsed), "");
    assert_eq!(safe_from_html_to_md(String::new()).unwrap(), "");
}

#[test]
fn multibyte_prefix_reports_byte_offset_for_truncated_tag() {
    assert_eq!(
        safe_parse_html("東京😀<article".to_string()),
        Err(ParseHTMLError::MalformedTag(
            "<article".to_string(),
            MalformedTagError::MissingClosingBracket(10),
        ))
    );
}

#[test]
fn bom_and_nul_text_survive_parse_and_render() {
    let text = "\u{feff}left\0右";
    let parsed = safe_parse_html(format!("<p>{text}</p>")).unwrap();

    assert_eq!(parsed.children[0].value.as_deref(), Some(text));
    assert_eq!(to_md(parsed), format!("{text}\n"));
}

#[test]
fn unicode_custom_tag_name_is_normalized_without_losing_body() {
    let parsed = safe_parse_html("<ΔΕΛΤΑ>中🙂</ΔΕΛΤΑ>".to_string()).unwrap();

    assert_eq!(
        parsed.tag_name,
        Some(NodeType::Unknown("δελτα".to_string()))
    );
    assert_eq!(parsed.children[0].value.as_deref(), Some("中🙂"));
    assert_eq!(to_md(parsed), "<δελτα>中🙂</δελτα>");
}
