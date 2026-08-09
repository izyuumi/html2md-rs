#[cfg(test)]
mod parser_tests {
    use html2md_rs::{
        parser::{safe_parse_html, MalformedTagError, ParseHTMLError},
        structs::{AttributeValues, Attributes, Node, NodeType::*},
        to_md::to_md,
    };

    #[test]
    fn parse_simple_div_with_text() {
        let input = "<div>hello</div>".to_string();
        let expected = Node::new(
            Some(Div),
            None,
            None,
            None,
            vec![Node::new(
                Some(Text),
                Some("hello".to_string()),
                None,
                None,
                vec![],
            )],
        );
        assert_eq!(safe_parse_html(input).unwrap(), expected);
    }

    #[test]
    fn parse_multiple_headers() {
        let input = "<h1>hello</h1><h2>world</h2>".to_string();
        let expected = Node::new(
            None,
            None,
            None,
            None,
            vec![
                Node::new(
                    Some(H1),
                    None,
                    None,
                    None,
                    vec![Node::new(
                        Some(Text),
                        Some("hello".to_string()),
                        None,
                        None,
                        vec![],
                    )],
                ),
                Node::new(
                    Some(H2),
                    None,
                    None,
                    None,
                    vec![Node::new(
                        Some(Text),
                        Some("world".to_string()),
                        None,
                        None,
                        vec![],
                    )],
                ),
            ],
        );
        assert_eq!(safe_parse_html(input).unwrap(), expected);
    }

    #[test]
    fn parse_unordered_list() {
        let input = "<ul><li>hello</li><li>world</li></ul>".to_string();
        let expected = Node::new(
            Some(Ul),
            None,
            None,
            None,
            vec![
                Node::new(
                    Some(Li),
                    None,
                    None,
                    Some(vec![Ul]),
                    vec![Node::new(
                        Some(Text),
                        Some("hello".to_string()),
                        None,
                        Some(vec![Ul]),
                        Vec::new(),
                    )],
                ),
                Node::new(
                    Some(Li),
                    None,
                    None,
                    Some(vec![Ul]),
                    vec![Node::new(
                        Some(Text),
                        Some("world".to_string()),
                        None,
                        Some(vec![Ul]),
                        Vec::new(),
                    )],
                ),
            ],
        );
        assert_eq!(safe_parse_html(input).unwrap(), expected);
    }

    #[test]
    fn parse_ordered_list() {
        let input = "<ol><li>hello</li><li>world</li></ol>".to_string();
        let expected = Node::new(
            Some(Ol),
            None,
            None,
            None,
            vec![
                Node::new(
                    Some(Li),
                    None,
                    None,
                    Some(vec![Ol]),
                    vec![Node::new(
                        Some(Text),
                        Some("hello".to_string()),
                        None,
                        Some(vec![Ol]),
                        Vec::new(),
                    )],
                ),
                Node::new(
                    Some(Li),
                    None,
                    None,
                    Some(vec![Ol]),
                    vec![Node::new(
                        Some(Text),
                        Some("world".to_string()),
                        None,
                        Some(vec![Ol]),
                        Vec::new(),
                    )],
                ),
            ],
        );
        assert_eq!(safe_parse_html(input).unwrap(), expected);
    }

    #[test]
    fn self_closing_div() {
        let input = "<div />".to_string();
        let expected = Node::new(Some(Div), None, None, None, vec![]).with_explicit_self_closing();
        assert_eq!(safe_parse_html(input).unwrap(), expected);
    }

    #[test]
    fn tracks_explicit_slash_separately_from_void_semantics() {
        let implicit_void = safe_parse_html("<img>".to_string()).unwrap();
        let explicit_void = safe_parse_html("<img />".to_string()).unwrap();

        assert!(!implicit_void.explicitly_self_closing);
        assert!(explicit_void.explicitly_self_closing);
        assert_eq!(to_md(implicit_void), "<img />");
        assert_eq!(to_md(explicit_void), "<img />");
    }

    #[test]
    fn tracks_explicitly_self_closing_div_among_siblings() {
        let input = "<div>hello</div>
<div />"
            .to_string();
        let expected = Node::new(
            None,
            None,
            None,
            None,
            vec![
                Node::new(
                    Some(Div),
                    None,
                    None,
                    None,
                    vec![Node::new(
                        Some(Text),
                        Some("hello".to_string()),
                        None,
                        None,
                        vec![],
                    )],
                ),
                Node::new(Some(Div), None, None, None, vec![]).with_explicit_self_closing(),
            ],
        );
        assert_eq!(safe_parse_html(input).unwrap(), expected);
    }

    #[test]
    fn missing_closing_bracket() {
        let input = "<div>hello</div><div".to_string();
        assert_eq!(
            safe_parse_html(input),
            Err(ParseHTMLError::MalformedTag(
                "<div".to_string(),
                MalformedTagError::MissingClosingBracket(16)
            ))
        );
    }

    #[test]
    fn missing_tag_name() {
        let input = "<>".to_string();
        assert_eq!(
            safe_parse_html(input),
            Err(ParseHTMLError::MalformedTag(
                "".to_string(),
                MalformedTagError::MissingTagName(0)
            ))
        );
    }

    #[test]
    fn unterminated_doctype_returns_error() {
        let input = "<!DOCTYPE html".to_string();
        assert_eq!(
            safe_parse_html(input.clone()),
            Err(ParseHTMLError::MalformedTag(
                input,
                MalformedTagError::MissingClosingBracket(0)
            ))
        );
    }

    #[test]
    fn doctype_is_case_insensitive() {
        assert_eq!(
            safe_parse_html("<!dOcTyPe html><p>x</p>".to_string()).unwrap(),
            Node::new(
                Some(P),
                None,
                None,
                None,
                vec![Node::new(
                    Some(Text),
                    Some("x".to_string()),
                    None,
                    None,
                    Vec::new(),
                )],
            )
        );
    }

    #[test]
    fn cdata_is_skipped() {
        assert_eq!(
            safe_parse_html("<![CDATA[<p>ignored</p>]]><p>x</p>".to_string()).unwrap(),
            Node::new(
                Some(P),
                None,
                None,
                None,
                vec![Node::new(
                    Some(Text),
                    Some("x".to_string()),
                    None,
                    None,
                    Vec::new(),
                )],
            )
        );

        let input = "<![CDATA[unterminated".to_string();
        assert_eq!(
            safe_parse_html(input.clone()),
            Err(ParseHTMLError::MalformedTag(
                input,
                MalformedTagError::MissingClosingBracket(0)
            ))
        );
    }

    #[test]
    fn non_ascii_attribute_before_self_closing_tag_keeps_siblings() {
        let parsed = safe_parse_html("<img alt=\"é\"/><p>x</p>".to_string()).unwrap();

        assert_eq!(parsed.tag_name, None);
        assert_eq!(parsed.children.len(), 2);
        assert_eq!(
            parsed.children[0].tag_name,
            Some(Unknown("img".to_string()))
        );
        assert_eq!(parsed.children[1].tag_name, Some(P));
    }

    #[test]
    fn raw_text_elements_preserve_body_and_resume_parsing() {
        let input = concat!(
            "<script>if (a < b) x = '</not-script>';</SCRIPT>",
            "<style>.x::before { content: '<b>'; }</StYlE>",
            "<textarea><b>literal</b></TEXTAREA>",
            "<title>A < B</TiTlE>",
            "<p>x</p>"
        )
        .to_string();
        let parsed = safe_parse_html(input).unwrap();

        assert_eq!(parsed.children.len(), 5);
        assert_eq!(parsed.children[0].tag_name, Some(Script));
        assert_eq!(
            parsed.children[0].children[0].value.as_deref(),
            Some("if (a < b) x = '</not-script>';")
        );
        assert_eq!(parsed.children[1].tag_name, Some(Style));
        assert_eq!(
            parsed.children[1].children[0].value.as_deref(),
            Some(".x::before { content: '<b>'; }")
        );
        assert_eq!(
            parsed.children[2].tag_name,
            Some(Unknown("textarea".to_string()))
        );
        assert_eq!(
            parsed.children[2].children[0].value.as_deref(),
            Some("<b>literal</b>")
        );
        assert_eq!(parsed.children[3].tag_name, Some(Title));
        assert_eq!(
            parsed.children[3].children[0].value.as_deref(),
            Some("A < B")
        );
        assert_eq!(parsed.children[4].tag_name, Some(P));
    }

    #[test]
    fn raw_text_close_scan_handles_many_false_candidates() {
        let body = "</".repeat(4_096);
        let parsed = safe_parse_html(format!("<script>{body}</ScRiPt \n\t><p>x</p>")).unwrap();

        assert_eq!(parsed.children.len(), 2);
        assert_eq!(
            parsed.children[0].children[0].value.as_deref(),
            Some(body.as_str())
        );
        assert_eq!(parsed.children[1].tag_name, Some(P));
    }

    #[test]
    fn unterminated_raw_text_returns_error() {
        let input = "<script>if (a < b)".to_string();
        assert_eq!(
            safe_parse_html(input.clone()),
            Err(ParseHTMLError::MalformedTag(
                input,
                MalformedTagError::MissingClosingBracket(0)
            ))
        );
    }

    #[test]
    fn quoted_attribute_values_support_both_quote_styles_and_empty_values() {
        let parsed = safe_parse_html(
            "<a href='' title='say \"hi\"' data-note=\"it's fine\"></a>".to_string(),
        )
        .unwrap();
        let attributes = parsed.attributes.as_ref().unwrap();

        assert_eq!(attributes.get("href"), Some(AttributeValues::from("")));
        assert_eq!(
            attributes.get("title"),
            Some(AttributeValues::from("say \"hi\""))
        );
        assert_eq!(
            attributes.get("data-note"),
            Some(AttributeValues::from("it's fine"))
        );
    }

    #[test]
    fn list_in_list() {
        let input = "
<ul>
  <li>
    <p>abc</p>
	<ul>
	  <li>
	    <p>abc</p>
	    <ol>
	      <li>
	        <p>123</p>
	      </li>
	    </ol>
	  </li>
	</ul>
  </li>
</ul>"
            .to_string();
        let expected = Node::new(
            Some(Ul),
            None,
            None,
            None,
            vec![Node::new(
                Some(Li),
                None,
                None,
                Some(vec![Ul]),
                vec![
                    Node::new(
                        Some(P),
                        None,
                        None,
                        Some(vec![Ul]),
                        vec![Node::new(
                            Some(Text),
                            Some("abc".to_string()),
                            None,
                            Some(vec![Ul]),
                            vec![],
                        )],
                    ),
                    Node::new(
                        Some(Ul),
                        None,
                        None,
                        Some(vec![Ul]),
                        vec![Node::new(
                            Some(Li),
                            None,
                            None,
                            Some(vec![Ul, Ul]),
                            vec![
                                Node::new(
                                    Some(P),
                                    None,
                                    None,
                                    Some(vec![Ul, Ul]),
                                    vec![Node::new(
                                        Some(Text),
                                        Some("abc".to_string()),
                                        None,
                                        Some(vec![Ul, Ul]),
                                        vec![],
                                    )],
                                ),
                                Node::new(
                                    Some(Ol),
                                    None,
                                    None,
                                    Some(vec![Ul, Ul]),
                                    vec![Node::new(
                                        Some(Li),
                                        None,
                                        None,
                                        Some(vec![Ul, Ul, Ol]),
                                        vec![Node::new(
                                            Some(P),
                                            None,
                                            None,
                                            Some(vec![Ul, Ul, Ol]),
                                            vec![Node::new(
                                                Some(Text),
                                                Some("123".to_string()),
                                                None,
                                                Some(vec![Ul, Ul, Ol]),
                                                vec![],
                                            )],
                                        )],
                                    )],
                                ),
                            ],
                        )],
                    ),
                ],
            )],
        );
        assert_eq!(safe_parse_html(input).unwrap(), expected);
    }

    #[test]
    fn equal_in_attribute_value() {
        let input = "<div class=\"hello=world\"></div>".to_string();
        let mut attributes = Attributes::new();
        attributes.insert("class".to_string(), AttributeValues::from("hello=world"));
        let expected = Node::new(Some(Div), None, Some(attributes), None, Vec::new());
        assert_eq!(safe_parse_html(input).unwrap(), expected);
    }

    // https://github.com/izyuumi/html2md-rs/issues/21
    #[test]
    fn issue_21() {
        let input =
            "<meta http-equiv=\"content-type\" content=\"text/html; charset=utf-8\">".to_string();
        let mut attributes = Attributes::new();
        attributes.insert(
            "http-equiv".to_string(),
            AttributeValues::from("content-type"),
        );
        attributes.insert(
            "content".to_string(),
            AttributeValues::from("text/html; charset=utf-8"),
        );
        let expected = Node::new(Some(Meta), None, Some(attributes), None, Vec::new());
        assert_eq!(safe_parse_html(input).unwrap(), expected);
    }

    // https://github.com/izyuumi/html2md-rs/issues/23
    #[test]
    fn issue_23() {
        let input = "<form id=\"search\" role=\"search\" action=/search></form>".to_string();
        let mut attributes = Attributes::new();
        attributes.insert("id".to_string(), AttributeValues::from("search"));
        attributes.insert("role".to_string(), AttributeValues::from("search"));
        attributes.insert("action".to_string(), AttributeValues::from("/search"));
        let expected = Node::new(
            Some(Unknown("form".to_string())),
            None,
            Some(attributes),
            None,
            Vec::new(),
        );
        assert_eq!(safe_parse_html(input).unwrap(), expected);
    }
}
