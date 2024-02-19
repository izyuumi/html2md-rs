#[cfg(test)]
mod to_md_tests {
    use html2md_rs::{parser::parse_html, to_md::to_md};

    #[test]
    fn simple_paragraph_with_text() {
        let input = "<p>hello</p>".to_string();
        let expected = "hello".to_string();
        assert_eq!(to_md(parse_html(input)), expected);
    }

    #[test]
    fn multiple_headers() {
        let input = "<h1>hello</h1><h2>world</h2>".to_string();
        let expected = "# hello\n## world\n".to_string();
        assert_eq!(to_md(parse_html(input)), expected);
    }

    #[test]
    fn paragraph_with_strong() {
        let input = "<p>hello <strong>world</strong></p>".to_string();
        let expected = "hello **world**".to_string();
        assert_eq!(to_md(parse_html(input)), expected);
    }

    #[test]
    fn strong_header() {
        let input = "<h1><strong>hello</strong></h1>".to_string();
        let expected = "# **hello**\n".to_string();
        assert_eq!(to_md(parse_html(input)), expected);
    }
}
