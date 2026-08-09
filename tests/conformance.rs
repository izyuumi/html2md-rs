use html2md_rs::{
    structs::{NodeType, ToMdConfig},
    to_md::{safe_from_html_to_md, safe_from_html_to_md_with_config},
};

#[test]
fn decodes_common_and_numeric_character_references() {
    let input = "<p>&amp; &lt; &gt; &quot; &apos; &nbsp; &#35; &#x1F600;</p>".to_string();

    assert_eq!(
        safe_from_html_to_md(input).unwrap(),
        "\\& \\< \\> \" ' \u{a0} \\# 😀\n"
    );
}

#[test]
fn escapes_commonmark_punctuation_in_text() {
    let input = r"<p>\ ` * _ [ ] ( ) # + - . ! > | ~</p>".to_string();

    assert_eq!(
        safe_from_html_to_md(input).unwrap(),
        "\\\\ \\` \\* \\_ \\[ \\] \\( \\) \\# \\+ \\- \\. \\! \\> \\| \\~\n"
    );
}

#[test]
fn rendering_config_applies_inside_lists() {
    let input = "<ul><li><strong>hidden</strong>visible</li></ul>".to_string();
    let config = ToMdConfig {
        ignore_rendering: vec![NodeType::Strong],
    };

    assert_eq!(
        safe_from_html_to_md_with_config(input, &config).unwrap(),
        "- visible\n"
    );
}

#[test]
fn fenced_code_has_stable_newlines_and_literal_content() {
    let input = "<pre><code class=\"language-text\">\n*literal* &amp;\n\n</code></pre>".to_string();

    assert_eq!(
        safe_from_html_to_md(input).unwrap(),
        "```text\n*literal* &amp;\n```\n"
    );
}
