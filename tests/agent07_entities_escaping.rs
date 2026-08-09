use html2md_rs::to_md::safe_from_html_to_md;

#[test]
fn adjacent_decimal_and_hex_numeric_entities_decode_to_unicode_scalars() {
    let markdown = safe_from_html_to_md("<p>A&#128512;&#x1F642;Z</p>".to_string()).unwrap();

    assert_eq!(markdown, "A😀🙂Z\n");
}

#[test]
fn uppercase_hex_marker_and_leading_zeroes_are_accepted() {
    let markdown = safe_from_html_to_md("<p>&#X00041;&#00066;</p>".to_string()).unwrap();

    assert_eq!(markdown, "AB\n");
}

#[test]
fn invalid_numeric_entities_remain_literal_text() {
    let markdown =
        safe_from_html_to_md("<p>&#x110000; &#55296; &#xZZ; &#x;</p>".to_string()).unwrap();

    assert_eq!(
        markdown,
        "\\&\\#x110000; \\&\\#55296; \\&\\#xZZ; \\&\\#x;\n"
    );
}

#[test]
fn unknown_and_wrong_case_named_entities_remain_literal_text() {
    let markdown = safe_from_html_to_md("<p>&NotARealEntity; &AMP;</p>".to_string()).unwrap();

    assert_eq!(markdown, "\\&NotARealEntity; \\&AMP;\n");
}

#[test]
fn entity_like_text_without_semicolon_is_not_decoded() {
    let markdown = safe_from_html_to_md("<p>&amp and &#42</p>".to_string()).unwrap();

    assert_eq!(markdown, "\\&amp and \\&\\#42\n");
}

#[test]
fn double_encoded_entity_is_decoded_exactly_once() {
    let markdown = safe_from_html_to_md("<p>&amp;amp; &amp;copy;</p>".to_string()).unwrap();

    assert_eq!(markdown, "\\&amp; \\&copy;\n");
}

#[test]
fn entity_derived_markdown_delimiters_are_escaped() {
    let markdown = safe_from_html_to_md(
        "<p>&#96;code&#96; &#42;em&#42; &#91;x&#93;&#40;y&#41;</p>".to_string(),
    )
    .unwrap();

    assert_eq!(markdown, "\\`code\\` \\*em\\* \\[x\\]\\(y\\)\n");
}

#[test]
fn unicode_text_and_non_breaking_space_survive_entity_decoding() {
    let markdown =
        safe_from_html_to_md("<p>東京&nbsp;cafe\u{301} 😀 &#x1F642;</p>".to_string()).unwrap();

    assert_eq!(markdown, "東京\u{a0}cafe\u{301} 😀 🙂\n");
}

#[test]
fn code_block_keeps_named_numeric_and_unknown_entities_raw() {
    let markdown =
        safe_from_html_to_md("<pre><code>&amp; &#42; &unknown;</code></pre>".to_string()).unwrap();

    assert_eq!(markdown, "```\n&amp; &#42; &unknown;\n```\n");
}

#[test]
fn html_comment_keeps_entities_and_markdown_punctuation_raw() {
    let markdown = safe_from_html_to_md("<!-- &amp; &#42; *literal* -->".to_string()).unwrap();

    assert_eq!(markdown, "<!-- &amp; &#42; *literal* -->");
}
