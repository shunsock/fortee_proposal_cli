use std::path::Path;

use crate::infrastructure::extractor::extract_first_content_from_text::extract_first_content_from_text;
use crate::infrastructure::extractor::extract_schedule::find_schedule;
use crate::infrastructure::extractor::extract_title_and_name::extract_title_and_name;
use crate::infrastructure::extractor::extract_track::find_track;
use crate::infrastructure::reader::read_html::read_html;
use crate::infrastructure::writer::write_json_from_proposal::write_json_from_proposal;
use crate::presentation::terminal_message_presenter::ConsoleMessager;
use crate::presentation::terminal_message_presenter::MessageType;

use crate::domain::proposal::proposal_model::ProposalModel;
use crate::infrastructure::types::title_and_speaker::ProposalTitleAndSpeaker;

pub fn build_structured_proposal_information() -> ProposalModel {
    let error_message = ConsoleMessager::new(
        "Failed to get structured information from the HTML file".to_string(),
        MessageType::Failed,
    );

    /*
     * read html data in local
     */
    let html_text = read_html().unwrap_or_else(|_| panic!("{}", error_message.supply_message()));

    /*
     * extract og-image url from html
     */
    let pattern = r#"https://fortee.jp/.+/proposal/og-image/[^"]+"#;
    let image_url = extract_first_content_from_text(&html_text, pattern);

    let is_image_found: bool = image_url.is_some();
    if !is_image_found {
        panic!("{}", error_message.supply_message());
    }

    /*
     * extract title and speaker from html
     * 1. search html title tag which include information by regex
     * 2. extract title and speaker by regex
     */
    let pattern = r#"<title>.+(トーク).+fortee.jp</title>"#;
    let title_tag = extract_first_content_from_text(&html_text, pattern);

    let is_title_tag_found: bool = title_tag.is_some();
    if !is_title_tag_found {
        panic!("{}", error_message.supply_message());
    };
    let proposal_title_and_speaker: ProposalTitleAndSpeaker =
        extract_title_and_name(&html_text).unwrap();

    /*
     * Extract schedule div content from html
     */
    let schedule: String = find_schedule(&html_text).unwrap_or_else(|| "未定".to_string());

    let track_name: String = find_track(&html_text).unwrap_or_else(|| "未定".to_string());

    /*
     * create structured data from above results
     */
    let proposal = ProposalModel {
        title: proposal_title_and_speaker.title,
        date: schedule,
        track: track_name,
        speaker: proposal_title_and_speaker.name,
        image_url: image_url.unwrap(),
    };

    let file_path = Path::new("data/proposal/proposal.json");
    write_json_from_proposal(&proposal, file_path)
        .unwrap_or_else(|_| panic!("{}", error_message.supply_message()));

    proposal
}
