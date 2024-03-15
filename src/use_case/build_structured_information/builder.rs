use std::path::Path;

use crate::infrastructure::extractor::extract_og_image_url::find_og_image_url;
use crate::infrastructure::extractor::extract_schedule::find_schedule;
use crate::infrastructure::extractor::extract_speaker::find_speaker;
use crate::infrastructure::extractor::extract_title::find_title;
use crate::infrastructure::extractor::extract_track::find_track;
use crate::infrastructure::reader::read_html::read_html;
use crate::infrastructure::writer::write_json_from_proposal::write_json_from_proposal;
use crate::presentation::terminal_message_presenter::ConsoleMessenger;
use crate::presentation::terminal_message_presenter::MessageType;

use crate::domain::proposal::proposal_model::ProposalModel;

pub fn build_structured_proposal_information() -> ProposalModel {
    let error_message = ConsoleMessenger::new(
        "Failed to get structured information from the HTML file".to_string(),
        MessageType::Failed,
    );

    /*
     * read html data in local
     */
    let html_text = read_html().unwrap_or_else(|_| panic!("{}", error_message.supply_message()));

    /*
     * extract title and speaker from html
     * 1. search html title tag which include information by regex
     * 2. extract title and speaker by regex
     */
    let title: String = find_title(&html_text);
    let speaker: String = find_speaker(&html_text);
    let schedule: String = find_schedule(&html_text);
    let track: String = find_track(&html_text);
    let og_image_url = find_og_image_url(&html_text);

    /*
     * create structured data from above results
     */
    let proposal = ProposalModel {
        title,
        schedule,
        track,
        speaker,
        og_image_url,
    };

    let file_path = Path::new("data/json/proposal.json");
    write_json_from_proposal(&proposal, file_path)
        .unwrap_or_else(|_| panic!("{}", error_message.supply_message()));

    proposal
}
