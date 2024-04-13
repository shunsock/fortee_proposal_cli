use crate::domain::proposal::proposal_json::ProposalJson;
use crate::presentation::send_message::send_message_to_console;
use crate::presentation::send_message::RunningStatus;
use crate::use_case::build_proposal_json_file_from_html::build_structured_proposal_information;
use crate::use_case::download_fortee_proposal_page_html::download_html_page;
use crate::use_case::download_og_image::download_og_image;

pub fn get_proposal_data_controller(url: &str) {
    /*
     * Download HTML page from the given URL
     */
    download_html_page(url);
    send_message_to_console(
        RunningStatus::Success,
        "HTML page has been successfully downloaded.",
    );

    /*
     * Extract structured information from the downloaded HTML page
     */
    let write_proposal_result: Result<bool, String> =
        build_structured_proposal_information(url.to_string());
    match write_proposal_result {
        Ok(_) => {}
        Err(e) => {
            send_message_to_console(RunningStatus::Failed, e.as_str());
            return;
        }
    }
    send_message_to_console(
        RunningStatus::Success,
        "Structured proposal information has been successfully created.",
    );

    /*
     * Print the structured information
     */
    let proposal = ProposalJson::new();
    let pretty_json = match proposal.get_pretty_json_string() {
        Ok(pretty_json) => pretty_json,
        Err(e) => {
            send_message_to_console(RunningStatus::Failed, e.as_str());
            return;
        }
    };
    send_message_to_console(
        RunningStatus::Success,
        format!("\n {}", &pretty_json).as_str(),
    );

    /*
     * Write information to access the proposal.json file
     */
    let proposal_json_file = ProposalJson::new();
    let file_path = proposal_json_file.get_file_path();
    send_message_to_console(
        RunningStatus::Notice,
        format!(
            "you can get data by running: cp {} path/your/directory",
            file_path.to_string_lossy()
        )
        .as_str(),
    );

    /*
     * Download OG image from the structured information
     */
    let og_image_url: &str = proposal.get_proposal_data().get_og_image_url();
    let image_path = match download_og_image(og_image_url) {
        Ok(image_path) => {
            send_message_to_console(
                RunningStatus::Success,
                "OG Image has been successfully downloaded.",
            );
            image_path
        }
        Err(_) => {
            send_message_to_console(
                RunningStatus::Failed,
                "Failed to download OG Image. Please check the URL.",
            );
            return;
        }
    };

    /*
     * Show how to get the downloaded image for users
     */
    send_message_to_console(
        RunningStatus::Notice,
        format!(
            "you can get data by running: cp {} path/your/directory",
            image_path.to_string_lossy()
        )
        .as_str(),
    );
}
