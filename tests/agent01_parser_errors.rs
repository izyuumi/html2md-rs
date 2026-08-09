use html2md_rs::{
    parser::{safe_parse_html, MalformedAttributeError, MalformedTagError, ParseHTMLError},
    to_md::safe_from_html_to_md,
};

#[test]
fn truncated_opening_tag_preserves_fragment_and_offset() {
    assert_eq!(
        safe_parse_html("<br/><article".to_string()),
        Err(ParseHTMLError::MalformedTag(
            "<article".to_string(),
            MalformedTagError::MissingClosingBracket(5),
        ))
    );
}

#[test]
fn truncated_declaration_preserves_fragment_and_offset() {
    assert_eq!(
        safe_parse_html("<br/><!".to_string()),
        Err(ParseHTMLError::MalformedTag(
            "<!".to_string(),
            MalformedTagError::MissingClosingBracket(5),
        ))
    );
}

#[test]
fn truncated_comment_preserves_fragment_and_offset() {
    assert_eq!(
        safe_parse_html("<br/><!-- unfinished".to_string()),
        Err(ParseHTMLError::MalformedTag(
            "<!-- unfinished".to_string(),
            MalformedTagError::MissingClosingBracket(5),
        ))
    );
}

#[test]
fn truncated_cdata_preserves_fragment_and_offset() {
    assert_eq!(
        safe_parse_html("<br/><![CDATA[payload".to_string()),
        Err(ParseHTMLError::MalformedTag(
            "<![CDATA[payload".to_string(),
            MalformedTagError::MissingClosingBracket(5),
        ))
    );
}

#[test]
fn truncated_doctype_preserves_fragment_and_offset() {
    assert_eq!(
        safe_parse_html("<br/><!doctype html".to_string()),
        Err(ParseHTMLError::MalformedTag(
            "<!doctype html".to_string(),
            MalformedTagError::MissingClosingBracket(5),
        ))
    );
}

#[test]
fn stray_closing_tag_error_propagates_through_markdown_api() {
    assert_eq!(
        safe_from_html_to_md("<br/></section>".to_string()),
        Err(ParseHTMLError::MalformedTag(
            "</section>".to_string(),
            MalformedTagError::MissingClosingBracket(5),
        ))
    );
}

#[test]
fn whitespace_only_tag_name_preserves_tag_content() {
    assert_eq!(
        safe_parse_html("<br/>< >".to_string()),
        Err(ParseHTMLError::MalformedTag(
            " ".to_string(),
            MalformedTagError::MissingTagName(5),
        ))
    );
}

#[test]
fn unterminated_attribute_quote_styles_report_missing_quotation() {
    assert_eq!(
        (
            safe_parse_html("<br/><a title=\"unfinished>".to_string()),
            safe_parse_html("<br/><a title='unfinished>".to_string()),
        ),
        (
            Err(ParseHTMLError::MalformedAttribute(
                "unfinished".to_string(),
                MalformedAttributeError::MissingQuotationMark(5),
            )),
            Err(ParseHTMLError::MalformedAttribute(
                "unfinished".to_string(),
                MalformedAttributeError::MissingQuotationMark(5),
            )),
        )
    );
}

#[test]
fn missing_attribute_name_preserves_attribute_fragment() {
    assert_eq!(
        safe_parse_html("<br/><div =value>".to_string()),
        Err(ParseHTMLError::MalformedAttribute(
            "=value".to_string(),
            MalformedAttributeError::MissingAttributeName(5),
        ))
    );
}

#[test]
fn missing_attribute_value_preserves_attribute_fragment() {
    assert_eq!(
        safe_parse_html("<br/><div data=>".to_string()),
        Err(ParseHTMLError::MalformedAttribute(
            "data=".to_string(),
            MalformedAttributeError::MissingAttributeValue(5),
        ))
    );
}
