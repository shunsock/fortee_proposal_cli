use regex::Regex;

pub fn find_track(html_content: &str) -> Option<String> {
    // Compile the regex pattern. Be careful with the pattern; let's escape properly and fix it!
    let pattern = r#"<div[^"]*?class="[^"]*?track[^"]*?"[^>]*?>(.*?)</div>"#;
    let re = Regex::new(pattern).expect("Failed to compile regex");

    // Search for the pattern in the given string.
    re.captures(html_content).and_then(|cap| {
        // If we find a match, return the first capture group, which is the content inside the div.
        cap.get(1).map(|match_| match_.as_str().to_string())
    })
}
