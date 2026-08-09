use html2md_rs::{parser::safe_parse_html, to_md::safe_from_html_to_md};
use std::panic::catch_unwind;

#[test]
fn safe_apis_do_not_panic_on_malformed_or_non_ascii_input() {
    let inputs = [
        "",
        "<",
        "<!",
        "<!DOCTYPE html",
        "</p>",
        "<div class=\"unterminated>",
        "<div =value>",
        "é<",
        "<p>こんにちは 😀</p>",
    ];

    for input in inputs {
        assert!(
            catch_unwind(|| safe_parse_html(input.to_string())).is_ok(),
            "safe_parse_html panicked for {input:?}"
        );
        assert!(
            catch_unwind(|| safe_from_html_to_md(input.to_string())).is_ok(),
            "safe_from_html_to_md panicked for {input:?}"
        );
    }
}

#[test]
fn converts_and_drops_deep_elements_without_overflowing() {
    const RENDER_DEPTH: usize = 10_000;
    let input = format!(
        "{}x{}",
        "<div>".repeat(RENDER_DEPTH),
        "</div>".repeat(RENDER_DEPTH)
    );

    assert_eq!(safe_from_html_to_md(input).unwrap(), "x");

    const DROP_DEPTH: usize = 100_000;
    let input = format!(
        "{}x{}",
        "<div>".repeat(DROP_DEPTH),
        "</div>".repeat(DROP_DEPTH)
    );
    drop(safe_parse_html(input).unwrap());
}
