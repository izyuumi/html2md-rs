#[cfg(test)]
mod parser_tests {
    use html2md_rs::{
        parser::{safe_parse_html, MalformedAttributeError, MalformedTagError, ParseHTMLError},
        structs::{Node, NodeType::*},
    };

    #[test]
    fn parse_simple_div_with_text() {
        let input = "<div>hello</div>".to_string();
        let expected = Node {
            tag_name: Some(Div),
            value: None,
            attributes: None,
            within_special_tag: None,
            children: vec![Node {
                tag_name: Some(Text),
                value: Some("hello".to_string()),
                attributes: None,
                within_special_tag: None,
                children: vec![],
            }],
        };
        assert_eq!(safe_parse_html(input).unwrap(), expected);
    }

    #[test]
    fn parse_multiple_headers() {
        let input = "<h1>hello</h1><h2>world</h2>".to_string();
        let expected = Node {
            tag_name: None,
            value: None,
            attributes: None,
            within_special_tag: None,
            children: vec![
                Node {
                    tag_name: Some(H1),
                    value: None,
                    attributes: None,
                    within_special_tag: None,
                    children: vec![Node {
                        tag_name: Some(Text),
                        value: Some("hello".to_string()),
                        attributes: None,
                        within_special_tag: None,
                        children: vec![],
                    }],
                },
                Node {
                    tag_name: Some(H2),
                    value: None,
                    attributes: None,
                    within_special_tag: None,
                    children: vec![Node {
                        tag_name: Some(Text),
                        value: Some("world".to_string()),
                        attributes: None,
                        within_special_tag: None,
                        children: vec![],
                    }],
                },
            ],
        };
        assert_eq!(safe_parse_html(input).unwrap(), expected);
    }

    #[test]
    fn parse_unordered_list() {
        let input = "<ul><li>hello</li><li>world</li></ul>".to_string();
        let expected = Node {
            tag_name: Some(Ul),
            children: vec![
                Node {
                    tag_name: Some(Li),
                    within_special_tag: Some(vec![Ul]),
                    children: vec![Node {
                        tag_name: Some(Text),
                        within_special_tag: Some(vec![Ul]),
                        value: Some("hello".to_string()),
                        ..Default::default()
                    }],
                    ..Default::default()
                },
                Node {
                    tag_name: Some(Li),
                    within_special_tag: Some(vec![Ul]),
                    children: vec![Node {
                        tag_name: Some(Text),
                        within_special_tag: Some(vec![Ul]),
                        value: Some("world".to_string()),
                        ..Default::default()
                    }],
                    ..Default::default()
                },
            ],
            ..Default::default()
        };
        assert_eq!(safe_parse_html(input).unwrap(), expected);
    }

    #[test]
    fn parse_ordered_list() {
        let input = "<ol><li>hello</li><li>world</li></ol>".to_string();
        let expected = Node {
            tag_name: Some(Ol),
            children: vec![
                Node {
                    tag_name: Some(Li),
                    within_special_tag: Some(vec![Ol]),
                    children: vec![Node {
                        tag_name: Some(Text),
                        value: Some("hello".to_string()),
                        within_special_tag: Some(vec![Ol]),
                        ..Default::default()
                    }],
                    ..Default::default()
                },
                Node {
                    tag_name: Some(Li),
                    within_special_tag: Some(vec![Ol]),
                    children: vec![Node {
                        tag_name: Some(Text),
                        value: Some("world".to_string()),
                        within_special_tag: Some(vec![Ol]),
                        ..Default::default()
                    }],
                    ..Default::default()
                },
            ],
            ..Default::default()
        };
        assert_eq!(safe_parse_html(input).unwrap(), expected);
    }

    #[test]
    fn self_closing_div() {
        let input = "<div />".to_string();
        let expected = Node {
            tag_name: Some(Div),
            value: None,
            attributes: None,
            within_special_tag: None,
            children: vec![],
        };
        assert_eq!(safe_parse_html(input).unwrap(), expected);
    }

    #[test]
    fn with_self_closing_div() {
        let input = "<div>hello</div>
<div />"
            .to_string();
        let expected = Node {
            tag_name: None,
            value: None,
            attributes: None,
            within_special_tag: None,
            children: vec![
                Node {
                    tag_name: Some(Div),
                    value: None,
                    attributes: None,
                    within_special_tag: None,
                    children: vec![Node {
                        tag_name: Some(Text),
                        value: Some("hello".to_string()),
                        attributes: None,
                        within_special_tag: None,
                        children: vec![],
                    }],
                },
                Node {
                    tag_name: Some(Div),
                    value: None,
                    attributes: None,
                    within_special_tag: None,
                    children: vec![],
                },
            ],
        };
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
    fn missing_quotation_mark() {
        let input = "<div><div class=hello></div></div>".to_string();
        assert_eq!(
            safe_parse_html(input),
            Err(ParseHTMLError::MalformedAttribute(
                "hello".to_string(),
                MalformedAttributeError::MissingQuotationMark(5)
            ))
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
}
