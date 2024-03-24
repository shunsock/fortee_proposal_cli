use crate::domain::proposal::proposal_model::ProposalModel;
use crate::infrastructure::extractor::extract_og_image_url::find_og_image_url;
use crate::infrastructure::extractor::extract_schedule::find_schedule;
use crate::infrastructure::extractor::extract_speaker::find_speaker;
use crate::infrastructure::extractor::extract_title::find_title;
use crate::infrastructure::extractor::extract_track::find_track;
use crate::infrastructure::file_path_provider::file_path_provider_trait::FilePathProviderTrait;
use crate::infrastructure::file_path_provider::html_file_path_provider::HtmlFilePathProvider;
use crate::infrastructure::file_path_provider::json_file_path_provider::JsonFilePathProvider;
use crate::infrastructure::reader::read_html::read_html;
use crate::infrastructure::writer::write_json_from_proposal::write_json_from_proposal;
use crate::presentation::terminal_message_presenter::ConsoleMessenger;
use crate::presentation::terminal_message_presenter::MessageType;
use crate::presentation::send_message::RunningStatus;
use crate::presentation::send_message::send_message;

pub fn build_structured_proposal_information() -> ProposalModel {
    let error_message = ConsoleMessenger::new(
        "Failed to get structured information from the HTML file".to_string(),
        MessageType::Failed,
    );

    /*
     * read html data in local
     */
    let html_path_provider = HtmlFilePathProvider::new("proposal");
    let html_path = html_path_provider.get_path();

    let html_text = read_html(html_path)
        .unwrap_or_else(|_| panic!(
            "{}",
            error_message.supply_message()
        ));

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
    
    send_message(
        RunningStatus::Success,
        "Successfully get structured information from the HTML file"
    );

    /*
     * write structured data to json file
     */
    let json_path_provider = JsonFilePathProvider::new("proposal");
    let file_path = json_path_provider.get_path();

    write_json_from_proposal(&proposal, file_path)
        .unwrap_or_else(|_| panic!("{}", error_message.supply_message()));

    proposal
}
