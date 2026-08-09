use html2md_rs::{
    structs::{NodeType, ToMdConfig},
    to_md::{safe_from_html_to_md, safe_from_html_to_md_with_config},
};

#[test]
fn language_class_is_found_among_unrelated_classes() {
    let html = r#"<pre><code class="highlight language-rust numbered">fn main() {}</code></pre>"#;

    assert_eq!(
        safe_from_html_to_md(html.to_string()).unwrap(),
        "```rust\nfn main() {}\n```\n"
    );
}

#[test]
fn language_code_normalizes_one_leading_crlf() {
    let html = "<pre><code class='language-text'>\r\nfirst\r\nsecond</code></pre>";

    assert_eq!(
        safe_from_html_to_md(html.to_string()).unwrap(),
        "```text\nfirst\r\nsecond\n```\n"
    );
}

#[test]
fn code_block_uses_longer_fence_than_embedded_backticks() {
    let html = "<pre><code class='language-text'>alpha\n```\nomega</code></pre>";

    assert_eq!(
        safe_from_html_to_md(html.to_string()).unwrap(),
        "````text\nalpha\n```\nomega\n````\n"
    );
}

#[test]
fn pre_without_code_becomes_fenced_code_block() {
    let html = "<pre>line 1\n  line 2</pre>";

    assert_eq!(
        safe_from_html_to_md(html.to_string()).unwrap(),
        "```\nline 1\n  line 2\n```\n"
    );
}

#[test]
fn inline_code_uses_code_span_delimiters() {
    let html = "<p>Use <code>x + y</code> now.</p>";

    assert_eq!(
        safe_from_html_to_md(html.to_string()).unwrap(),
        "Use `x + y` now\\.\n"
    );

    let html = "<p><code> foo </code> / <code>   </code></p>";
    assert_eq!(
        safe_from_html_to_md(html.to_string()).unwrap(),
        "`  foo  ` / `   `\n"
    );
}

#[test]
fn inline_code_chooses_delimiter_around_embedded_backtick() {
    let html = "<p><code>a`b</code></p>";

    assert_eq!(safe_from_html_to_md(html.to_string()).unwrap(), "``a`b``\n");
}

#[test]
fn markup_inside_code_contributes_text_without_markdown_formatting() {
    let html = "<pre><code>before <strong>bold</strong> after</code></pre>";

    assert_eq!(
        safe_from_html_to_md(html.to_string()).unwrap(),
        "```\nbefore bold after\n```\n"
    );
}

#[test]
fn ignored_pre_discards_its_complete_code_subtree() {
    let html = "<p>before</p><pre><code>hidden</code></pre><p>after</p>";
    let config = ToMdConfig {
        ignore_rendering: vec![NodeType::Pre],
    };

    assert_eq!(
        safe_from_html_to_md_with_config(html.to_string(), &config).unwrap(),
        "before\nafter\n"
    );
}

#[test]
fn adjacent_code_blocks_keep_independent_fences() {
    let html = "<pre><code>alpha</code></pre><pre><code>beta</code></pre>";

    assert_eq!(
        safe_from_html_to_md(html.to_string()).unwrap(),
        "```\nalpha\n```\n```\nbeta\n```\n"
    );
}

#[test]
fn empty_code_block_keeps_valid_fence_pair() {
    let html = "<pre><code></code></pre>";

    assert_eq!(
        safe_from_html_to_md(html.to_string()).unwrap(),
        "```\n```\n"
    );
}
