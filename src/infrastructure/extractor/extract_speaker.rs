use crate::infrastructure::extractor::extract_first_content_from_text::extract_first_content_from_text;

pub fn find_speaker(html_content: &str) -> Result<String, String> {
    let pattern = r#"<title>.+ by ([^\s|]+) \|"#;

    match extract_first_content_from_text(html_content, pattern) {
        Some(speaker) => Ok(speaker),
        None => Err("Error: Failed to extract speaker from the HTML file".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_schedule_test() {
        let html_content = r#"
            <title>プロポーザルに通したいのでプロポーザルのテキスト分析をします！ by shunsock | トーク | PHPカンファレンス北海道2024 #phpcondo - fortee.jp</title>
        "#;
        let schedule = find_speaker(html_content);
        assert_eq!(schedule, Ok("shunsock".to_string()));
    }
}
