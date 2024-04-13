use crate::domain::proposal::proposal_html::ProposalHtmlFileWriter;
use crate::infrastructure::fetcher::fetch_fortee_proposal_page::fetch_fortee_proposal_page;
use crate::presentation::send_message::send_message_as_string;
use crate::presentation::send_message::RunningStatus;

pub fn download_html_page(url: &str) {
    /* Fetch the html text */
    let http_response = fetch_fortee_proposal_page(url);
    let html_text = match http_response {
        Ok(text) => text,
        Err(e) => {
            panic!(
                "{}",
                send_message_as_string(RunningStatus::Failed, &format!("Error: {}", e))
            )
        }
    };

    /* Save the html text to a file */
    let html_file_path_provider: ProposalHtmlFileWriter = ProposalHtmlFileWriter::new(html_text);
    let save_file_result: Result<bool, String> = html_file_path_provider.write();
    match save_file_result {
        Ok(_) => {
            println!(
                "{}",
                send_message_as_string(
                    RunningStatus::Success,
                    "Successfully saved the html to a file",
                )
            );
        }
        Err(e) => {
            panic!("{}", send_message_as_string(RunningStatus::Failed, &e));
        }
    }
}
