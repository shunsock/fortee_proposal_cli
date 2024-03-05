use crate::infrastructure::extractor::extract_first_content_from_text::extract_first_content_from_text;

pub fn find_schedule(html_content: &str) -> String {
    let pattern = r#"<div[^"]*?class="[^"]*?schedule[^"]*?"[^>]*?>(.*?)</div>"#;

    extract_first_content_from_text(html_content, pattern).unwrap_or_else(|| "未定".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_schedule_test() {
        let html_content = r#"
            <div class="type mB-10 d-flex align-items-center">
            <div class="mR-10"><span class="badge-danger badge">採択</span></div>
            <div class="schedule mR-10">2024/03/07 17:30〜</div>
            <div class="track mR-20">Track A</div>
        "#;
        let schedule = find_schedule(html_content);
        assert_eq!(schedule, "2024/03/07 17:30〜");
    }
}
