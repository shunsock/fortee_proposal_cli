use crate::infrastructure::extractor::extract_first_content_from_text::extract_first_content_from_text;

pub fn find_og_image_url(html_content: &str) -> String {
    let pattern = r#"(https://fortee.jp/.+/proposal/og-image/[^"]+)"#;

    extract_first_content_from_text(html_content, pattern).unwrap_or_else(|| "未定".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_schedule_test() {
        let html_content = r#"
            <div><img src="https://fortee.jp/phpcon-hokkaido-2024/proposal/og-image/27161196-7076-4bd3-91a5-7a674fa90d51.png" class="sW-100p bdrs-10" alt=""></div>
        "#;
        let schedule = find_og_image_url(html_content);
        assert_eq!(schedule, "https://fortee.jp/phpcon-hokkaido-2024/proposal/og-image/27161196-7076-4bd3-91a5-7a674fa90d51.png");
    }
}
