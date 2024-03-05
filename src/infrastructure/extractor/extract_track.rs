use crate::infrastructure::extractor::extract_first_content_from_text::extract_first_content_from_text;

pub fn find_track(html_content: &str) -> String {
    let pattern = r#"<div[^"]*?class="[^"]*?track[^"]*?"[^>]*?>(.*?)</div>"#;

    extract_first_content_from_text(html_content, pattern).unwrap_or_else(|| "未定".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_schedule_test() {
        let html_content = r#"
            <div class="track mR-20">ｸﾘｴｲﾃｨﾌﾞｽﾀｼﾞｵ</div>
        "#;
        let schedule = find_track(html_content);
        assert_eq!(schedule, "ｸﾘｴｲﾃｨﾌﾞｽﾀｼﾞｵ");
    }
}
