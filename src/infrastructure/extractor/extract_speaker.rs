use crate::infrastructure::extractor::extract_first_content_from_text::extract_first_content_from_text;

pub fn find_speaker(html_content: &str) -> String {
    let pattern = r#"<title>.+ by ([^\s|]+) \|"#;

    let speaker_optional = extract_first_content_from_text(html_content, pattern);

    match speaker_optional {
        Some(speaker) => speaker,
        None => panic!("Failed to extract speaker from the HTML file"),
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
        assert_eq!(schedule, "shunsock");
    }
}
