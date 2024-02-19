use html2md_rs::{
    parser::*,
    structs::{
        Node,
        NodeType::{self, *},
    },
};

#[test]
fn parse_simple_div_with_text() {
    let input = "<div>hello</div>".to_string();
    let expected = Node {
        tag_name: Some(Div),
        value: None,
        attributes: None,
        children: vec![Node {
            tag_name: Some(Text),
            value: Some("hello".to_string()),
            attributes: None,
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
        children: vec![
            Node {
                tag_name: Some(H1),
                value: None,
                attributes: None,
                children: vec![Node {
                    tag_name: Some(Text),
                    value: Some("hello".to_string()),
                    attributes: None,
                    children: vec![],
                }],
            },
            Node {
                tag_name: Some(H2),
                value: None,
                attributes: None,
                children: vec![Node {
                    tag_name: Some(Text),
                    value: Some("world".to_string()),
                    attributes: None,
                    children: vec![],
                }],
            },
        ],
    };
    assert_eq!(parse_html(input), expected);
}
