use crate::domain::proposal::proposal_data_model::ProposalDataModel;
use crate::domain::proposal::proposal_html::ProposalHtml;
use crate::domain::proposal::proposal_json::ProposalJsonFileWriter;
use crate::infrastructure::extractor::extract_og_image_url::find_og_image_url;
use crate::infrastructure::extractor::extract_schedule::find_schedule;
use crate::infrastructure::extractor::extract_speaker::find_speaker;
use crate::infrastructure::extractor::extract_title::find_title;
use crate::infrastructure::extractor::extract_track::find_track;
use crate::presentation::send_message::send_message_to_console;
use crate::presentation::send_message::RunningStatus;

pub fn build_structured_proposal_information() -> Result<bool, String> {
    /*
     * read html data in local
     */
    let proposal_html_file: ProposalHtml = ProposalHtml::new();
    let html_text: String = proposal_html_file.get_html();

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
    let proposal = ProposalDataModel::new(title, schedule, track, speaker, og_image_url);

    send_message_to_console(
        RunningStatus::Success,
        "Successfully get structured information from the HTML file",
    );

    /*
     * write structured data to json file
     */
    let proposal_json_writer = ProposalJsonFileWriter::new(proposal);
    let res: Result<bool, String> = proposal_json_writer.write();
    match res {
        Ok(_) => {
            send_message_to_console(
                RunningStatus::Success,
                "Successfully write structured information to the JSON file",
            );
            Ok(true)
        }
        Err(e) => {
            send_message_to_console(RunningStatus::Failed, &format!("Error: {}", e));
            Err(e)
        }
    }
}
