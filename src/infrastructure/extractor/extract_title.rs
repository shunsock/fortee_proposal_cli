use crate::infrastructure::extractor::extract_first_content_from_text::extract_first_content_from_text;

pub fn find_title(html_content: &str) -> String {
    let pattern = r#"<title>(.+) by [^\s|]+ \|"#;

    let title_optional = extract_first_content_from_text(html_content, pattern);

    match title_optional {
        Some(title) => title,
        None => panic!("Failed to extract title from the HTML file"),
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
        let schedule = find_title(html_content);
        assert_eq!(
            schedule,
            "プロポーザルに通したいのでプロポーザルのテキスト分析をします！"
        );
    }
}
