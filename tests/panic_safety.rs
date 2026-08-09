use html2md_rs::{parser::safe_parse_html, to_md::safe_from_html_to_md};
use std::io::Write;
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
fn safe_apis_do_not_panic_on_generated_utf8_input() {
    const ALPHABET: [char; 16] = [
        '<', '>', '/', '!', '-', '=', '\'', '"', '&', ';', ' ', '\0', 'é', '日', '😀', '\n',
    ];
    let mut state = 0x4d59_5df4_d0f3_3173_u64;

    for case in 0..10_000 {
        let length = case % 128;
        let mut input = String::with_capacity(length);
        for _ in 0..length {
            state = state
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1);
            input.push(ALPHABET[(state as usize) % ALPHABET.len()]);
        }

        assert!(
            catch_unwind(|| safe_parse_html(input.clone())).is_ok(),
            "safe_parse_html panicked for generated case {case}: {input:?}"
        );
        assert!(
            catch_unwind(|| safe_from_html_to_md(input.clone())).is_ok(),
            "safe_from_html_to_md panicked for generated case {case}: {input:?}"
        );
    }
}

#[test]
fn converts_and_drops_deep_elements_without_overflowing() {
    let empty = safe_parse_html(String::new()).unwrap();
    assert_eq!(
        format!("{empty:?}"),
        "Node { tag_name: None, value: None, attributes: None, explicitly_self_closing: false, within_special_tag: None, children: [] }"
    );
    assert_eq!(
        format!("{empty:#?}"),
        "Node {\n    tag_name: None,\n    value: None,\n    attributes: None,\n    explicitly_self_closing: false,\n    within_special_tag: None,\n    children: [],\n}"
    );

    let ordinary = safe_parse_html(
        "<ol start='2'><li><strong id='x'>one</strong></li><li>two</li></ol>".to_string(),
    )
    .unwrap();
    assert_eq!(ordinary.clone(), ordinary);

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
    let node = safe_parse_html(input).unwrap();
    let cloned = node.clone();

    assert!(node == cloned);
    write!(&mut std::io::sink(), "{node:?}").unwrap();

    let mut cursor = &cloned;
    for _ in 0..DROP_DEPTH {
        assert_eq!(cursor.children.len(), 1);
        cursor = &cursor.children[0];
    }
    assert_eq!(cursor.value.as_deref(), Some("x"));

    drop((node, cloned));
}
