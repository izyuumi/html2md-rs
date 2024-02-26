#[cfg(test)]
mod to_md_tests {
    use html2md_rs::to_md::from_html_to_md;

    #[test]
    fn simple_paragraph_with_text() {
        let input = "<p>hello</p>".to_string();
        let expected = "hello\n".to_string();
        assert_eq!(from_html_to_md(input), expected);
    }

    #[test]
    fn multiple_headers() {
        let input = "<h1>hello</h1><h2>world</h2>".to_string();
        let expected = "# hello\n## world\n".to_string();
        assert_eq!(from_html_to_md(input), expected);
    }

    #[test]
    fn paragraph_with_strong() {
        let input = "<p>hello <strong>world</strong></p>".to_string();
        let expected = "hello **world**\n".to_string();
        assert_eq!(from_html_to_md(input), expected);
    }

    #[test]
    fn strong_header() {
        let input = "<h1><strong>hello</strong></h1>".to_string();
        let expected = "# **hello**\n".to_string();
        assert_eq!(from_html_to_md(input), expected);
    }

    #[test]
    fn unordered_list() {
        let input = "<ul><li>hello</li><li>world</li></ul>".to_string();
        let expected = "- hello\n- world\n".to_string();
        assert_eq!(from_html_to_md(input), expected);
    }

    #[test]
    fn ordered_list() {
        let input = "<ol><li>hello</li><li>world</li></ol>".to_string();
        let expected = "1. hello\n2. world\n".to_string();
        assert_eq!(from_html_to_md(input), expected);
    }

    #[test]
    fn multiple_paragraphs() {
        let input = "<p>hello</p><p>world</p>".to_string();
        let expected = "hello\nworld\n".to_string();
        assert_eq!(from_html_to_md(input), expected);
    }

    #[test]
    fn multiple_paragraphs_with_empty_paragraph() {
        let input = "<p>hello</p><p></p><p>world</p>".to_string();
        let expected = "hello\nworld\n".to_string();
        assert_eq!(from_html_to_md(input), expected);
    }

    #[test]
    fn header_and_paragraph() {
        let input = "<h1>hello</h1><p></p><p>world</p>".to_string();
        let expected = "# hello\nworld\n".to_string();
        assert_eq!(from_html_to_md(input), expected);
    }

    #[test]
    fn header_and_paragraph() {
        let input = "<h1>hello</h1><p></p><p>world</p>".to_string();
        let expected = "# hello\nworld\n".to_string();
        assert_eq!(from_html_to_md(input), expected);
    }
    #[test]
    fn has_in_header() {
        let input = "<h1># hello</h1>",to_string();
        let expected = "# # hello\n", expected();
        assert_eq!(from_html_to_md(input), expected);
    }
    #[test]
    fn subheader_in_header() {
        let input = "<h1>## hello</h1>", to_string();
        let expected "# ## hello\n", expected();
        assert_eq!(from_html_to_md(input), expected);
    }
    #[test]
    fn header_in_subheader() {
        let input = "<h2># hello</h2>", to_string();
        let expected "## # hello\n", expected();
        assert_eq!(from_html_to_md(input), expected);
    }
}
