use html2md_rs::{parser::safe_parse_html, structs::AttributeValues, to_md::safe_from_html_to_md};

#[test]
fn boolean_attribute_is_true_and_renders_bare() {
    let parsed = safe_parse_html("<input disabled>".to_string()).unwrap();
    let attributes = parsed.attributes.as_ref().unwrap();

    assert_eq!(
        attributes.get("disabled"),
        Some(AttributeValues::Bool(true))
    );
    assert_eq!(
        safe_from_html_to_md("<input disabled>".to_string()).unwrap(),
        "<input disabled />"
    );
}

#[test]
fn empty_double_quoted_attribute_value_is_preserved() {
    let parsed = safe_parse_html(r#"<div data-label=""></div>"#.to_string()).unwrap();
    let attributes = parsed.attributes.as_ref().unwrap();

    assert_eq!(
        attributes.get("data-label"),
        Some(AttributeValues::String("".to_string()))
    );
}

#[test]
fn empty_single_quoted_id_is_visible_through_public_accessor() {
    let parsed = safe_parse_html("<div id=''></div>".to_string()).unwrap();
    let attributes = parsed.attributes.as_ref().unwrap();

    assert_eq!(attributes.get_id().map(String::as_str), Some(""));
}

#[test]
fn single_quoted_href_can_contain_double_quotes() {
    let parsed =
        safe_parse_html(r#"<a href='https://example.test/"quoted"'>link</a>"#.to_string()).unwrap();
    let attributes = parsed.attributes.as_ref().unwrap();

    assert_eq!(
        attributes.get_href(),
        Some("https://example.test/\"quoted\"".to_string())
    );
}

#[test]
fn double_quoted_attribute_value_can_contain_apostrophe() {
    let parsed = safe_parse_html(r#"<div title="reader's choice"></div>"#.to_string()).unwrap();
    let attributes = parsed.attributes.as_ref().unwrap();

    assert_eq!(
        attributes.get("title"),
        Some(AttributeValues::String("reader's choice".to_string()))
    );
}

#[test]
fn unquoted_attribute_value_is_preserved() {
    let parsed = safe_parse_html("<div data-mode=compact></div>".to_string()).unwrap();
    let attributes = parsed.attributes.as_ref().unwrap();

    assert_eq!(
        attributes.get("data-mode"),
        Some(AttributeValues::String("compact".to_string()))
    );
}

#[test]
fn equals_signs_inside_unquoted_attribute_value_are_preserved() {
    let parsed = safe_parse_html("<div data-query=a=b=c></div>".to_string()).unwrap();
    let attributes = parsed.attributes.as_ref().unwrap();

    assert_eq!(
        attributes.get("data-query"),
        Some(AttributeValues::String("a=b=c".to_string()))
    );
}

#[test]
fn whitespace_around_equals_is_accepted() {
    let parsed = safe_parse_html("<div data-layout = 'wide'></div>".to_string()).unwrap();
    let attributes = parsed.attributes.as_ref().unwrap();

    assert_eq!(
        attributes.get("data-layout"),
        Some(AttributeValues::String("wide".to_string()))
    );
}

#[test]
fn unicode_id_and_class_are_visible_through_public_accessors() {
    let parsed = safe_parse_html("<div id='東京' class=投稿カード></div>".to_string()).unwrap();
    let attributes = parsed.attributes.as_ref().unwrap();

    assert_eq!(attributes.get_id().map(String::as_str), Some("東京"));
    assert_eq!(
        attributes.get_class().map(String::as_str),
        Some("投稿カード")
    );
}

#[test]
fn unquoted_ordered_list_start_controls_rendered_numbering() {
    let markdown =
        safe_from_html_to_md("<ol start=7><li>first</li><li>second</li></ol>".to_string()).unwrap();

    assert_eq!(markdown, "7. first\n8. second\n");
}
