use crate::infrastructure::extractor::extract_first_content_from_text::extract_first_content_from_text;

pub fn find_speaker(html_content: &str) -> Option<String> {
    let pattern = r#"<title>.+ by ([^\s|]+) \|"#;
    extract_first_content_from_text(html_content, pattern)
}
