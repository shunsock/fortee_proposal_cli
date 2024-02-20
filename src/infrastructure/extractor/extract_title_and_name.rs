use regex::Regex;

use crate::infrastructure::types::title_and_speaker::ProposalTitleAndSpeaker;

pub fn extract_title_and_name(html_content: &str) -> Option<ProposalTitleAndSpeaker> {
    // Regex pattern to find the alt attribute and capture its value
    let pattern = r#"<title>(.+) by ([^\s|]+) \|"#;
    let re = Regex::new(pattern).unwrap();

    if let Some(captures) = re.captures(html_content) {
        let title = captures.get(1).map(|match_| match_.as_str().to_string());
        let speaker = captures.get(2).map(|match_| match_.as_str().to_string());

        let is_title_found: bool = title.is_some();
        let is_speaker_found: bool = speaker.is_some();
        if is_title_found && is_speaker_found {
            return Some(ProposalTitleAndSpeaker {
                title: title.unwrap(),
                name: speaker.unwrap(),
            });
        }
    }
    None
}
