#[cfg(test)]

mod parser_tests {
    use html2md_rs::{
        parser::{
            parse_html, safe_parse_html, MalformedAttributeError, MalformedTagError,
            ParseHTMLTypeError,
        },
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
        assert_eq!(parse_html(input), expected);
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
        assert_eq!(parse_html(input), expected);
    }

    #[test]
    fn parse_unordered_list() {
        let input = "<ul><li>hello</li><li>world</li></ul>".to_string();
        let expected = Node {
            tag_name: Some(Ul),
            value: None,
            attributes: None,
            within_special_tag: None,
            children: vec![
                Node {
                    tag_name: Some(Li),
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
                    tag_name: Some(Li),
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
        assert_eq!(parse_html(input), expected);
    }

    #[test]
    fn parse_ordered_list() {
        let input = "<ol><li>hello</li><li>world</li></ol>".to_string();
        let expected = Node {
            tag_name: Some(Ol),
            value: None,
            attributes: None,
            within_special_tag: None,
            children: vec![
                Node {
                    tag_name: Some(Li),
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
                    tag_name: Some(Li),
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
        assert_eq!(parse_html(input), expected);
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
        assert_eq!(parse_html(input), expected);
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
        assert_eq!(parse_html(input), expected);
    }

    #[test]
    fn missing_closing_bracket() {
        let input = "<div>hello</div><div".to_string();
        assert_eq!(
            safe_parse_html(input),
            Err(ParseHTMLTypeError::MalformedTag(
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
            Err(ParseHTMLTypeError::MalformedTag(
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
            Err(ParseHTMLTypeError::MalformedAttribute(
                "hello".to_string(),
                MalformedAttributeError::MissingQuotationMark(5)
            ))
        );
    }
}
