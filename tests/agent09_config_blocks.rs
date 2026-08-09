use html2md_rs::{
    structs::{Node, NodeType, ToMdConfig},
    to_md::{safe_from_html_to_md_with_config, to_md_with_config},
};

#[test]
fn ignored_html_root_discards_complete_document() {
    let config = ToMdConfig {
        ignore_rendering: vec![NodeType::Html],
    };

    assert_eq!(
        safe_from_html_to_md_with_config(
            "<html><body><h1>hidden</h1><p>also hidden</p></body></html>".to_string(),
            &config,
        )
        .unwrap(),
        ""
    );
}

#[test]
fn synthetic_root_propagates_config_to_block_children() {
    let root = Node::new(
        None,
        None,
        None,
        None,
        vec![
            Node::new(
                Some(NodeType::H2),
                None,
                None,
                None,
                vec![Node::new(
                    Some(NodeType::Text),
                    Some("hidden".to_string()),
                    None,
                    None,
                    Vec::new(),
                )],
            ),
            Node::new(
                Some(NodeType::P),
                None,
                None,
                None,
                vec![Node::new(
                    Some(NodeType::Text),
                    Some("visible".to_string()),
                    None,
                    None,
                    Vec::new(),
                )],
            ),
        ],
    );
    let config = ToMdConfig {
        ignore_rendering: vec![NodeType::H2],
    };

    assert_eq!(to_md_with_config(root, &config), "visible\n");
}

#[test]
fn ignored_nested_div_discards_complete_mixed_subtree() {
    let config = ToMdConfig {
        ignore_rendering: vec![NodeType::Div],
    };

    assert_eq!(
        safe_from_html_to_md_with_config(
            "<p>before</p><div><h2>hidden</h2><blockquote><p>also hidden</p></blockquote></div><p>after</p>"
                .to_string(),
            &config,
        )
        .unwrap(),
        "before\nafter\n"
    );
}

#[test]
fn ignored_paragraph_inside_blockquote_leaves_no_quote_artifacts() {
    let config = ToMdConfig {
        ignore_rendering: vec![NodeType::P],
    };

    assert_eq!(
        safe_from_html_to_md_with_config(
            "<blockquote><p>hidden</p></blockquote><div>visible</div>".to_string(),
            &config,
        )
        .unwrap(),
        "visible"
    );
}

#[test]
fn ignored_blockquote_discards_nested_blocks_but_keeps_following_paragraph() {
    let config = ToMdConfig {
        ignore_rendering: vec![NodeType::Blockquote],
    };

    assert_eq!(
        safe_from_html_to_md_with_config(
            "<blockquote><h3>hidden</h3><p>also hidden</p></blockquote><p>visible</p>".to_string(),
            &config,
        )
        .unwrap(),
        "visible\n"
    );
}

#[test]
fn ignored_heading_discards_nested_link_without_heading_newline() {
    let config = ToMdConfig {
        ignore_rendering: vec![NodeType::H3],
    };

    assert_eq!(
        safe_from_html_to_md_with_config(
            "<p>before</p><h3><a href='/hidden'>hidden</a></h3><p>after</p>".to_string(),
            &config,
        )
        .unwrap(),
        "before\nafter\n"
    );
}

#[test]
fn ignored_link_discards_nested_image_and_label() {
    let config = ToMdConfig {
        ignore_rendering: vec![NodeType::A],
    };

    assert_eq!(
        safe_from_html_to_md_with_config(
            "<p>before<a href='/hidden'><img src='hidden.png'>hidden</a>after</p>".to_string(),
            &config,
        )
        .unwrap(),
        "beforeafter\n"
    );
}

#[test]
fn ignored_image_node_disappears_without_disturbing_paragraph() {
    let config = ToMdConfig {
        ignore_rendering: vec![NodeType::Unknown("img".to_string())],
    };

    assert_eq!(
        safe_from_html_to_md_with_config(
            "<p>before<img src='hidden.png' alt='hidden'>after</p>".to_string(),
            &config,
        )
        .unwrap(),
        "beforeafter\n"
    );
}

#[test]
fn ignored_list_items_do_not_leave_list_markers() {
    let config = ToMdConfig {
        ignore_rendering: vec![NodeType::Li],
    };

    assert_eq!(
        safe_from_html_to_md_with_config(
            "<ul><li>hidden</li><li><p>also hidden</p></li></ul>".to_string(),
            &config,
        )
        .unwrap(),
        ""
    );
}

#[test]
fn multiple_ignored_types_propagate_into_pre_and_neighbor_blocks() {
    let config = ToMdConfig {
        ignore_rendering: vec![NodeType::Code, NodeType::H2, NodeType::A],
    };

    assert_eq!(
        safe_from_html_to_md_with_config(
            "<h2>hidden</h2><pre><code>hidden code</code></pre><p>before<a href='/hidden'>hidden link</a>after</p>"
                .to_string(),
            &config,
        )
        .unwrap(),
        "beforeafter\n"
    );
}
