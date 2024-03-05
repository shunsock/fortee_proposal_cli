use regex::Regex;

pub fn extract_first_content_from_text(html_content: &str, pattern: &str) -> Option<String> {
    // compile the regex pattern
    let re = Regex::new(pattern).unwrap();

    // search for the first match in the given text
    re.captures(html_content)
        .and_then(|cap| cap.get(1).map(|match_| match_.as_str().to_string()))
}
