use crate::infrastructure::extractor::extract_first_content_from_text::extract_first_content_from_text;

pub fn find_track(html_content: &str) -> Option<String> {
    // Compile the regex pattern. Be careful with the pattern; let's escape properly and fix it!
    let pattern = r#"<div[^"]*?class="[^"]*?track[^"]*?"[^>]*?>(.*?)</div>"#;

    extract_first_content_from_text(html_content, pattern)
}
