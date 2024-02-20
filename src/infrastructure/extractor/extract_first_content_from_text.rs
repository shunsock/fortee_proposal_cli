use regex::Regex;

pub fn extract_first_content_from_text(html_content: &str, pattern: &str) -> Option<String> {
    // compile the regex pattern
    let re = Regex::new(pattern).unwrap();

    // search for the first match in the given text
    re.find(html_content) // Find the first match in the given text
        .map(|mat| mat.as_str().to_string()) // Convert the match to a String if found
}
