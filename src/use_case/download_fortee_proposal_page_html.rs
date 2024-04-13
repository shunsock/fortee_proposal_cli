use crate::domain::proposal::proposal_html::ProposalHtmlFileWriter;
use crate::infrastructure::fetcher::fetch_fortee_proposal_page::fetch_fortee_proposal_page;
use crate::presentation::send_message::send_message_to_console;
use crate::presentation::send_message::RunningStatus;
use crate::presentation::send_message::RunningStatus::Failed;

pub fn download_html_page(url: &str) -> Result<bool, String> {
    /* Fetch the html text */
    let http_response = fetch_fortee_proposal_page(url);
    let html_text = match http_response {
        Ok(text) => text,
        Err(e) => {
            send_message_to_console(Failed, e.to_string().as_str());
            return Err(e.to_string());
        }
    };

    /* Save the html text to a file */
    let html_file_path_provider: ProposalHtmlFileWriter = ProposalHtmlFileWriter::new(html_text);
    match html_file_path_provider.write() {
        Ok(_) => {
            send_message_to_console(
                RunningStatus::Success,
                "Successfully saved the html to a file",
            );
        }
        Err(e) => {
            send_message_to_console(Failed, e.to_string().as_str());
            return Err(e.to_string());
        }
    };

    Ok(true)
}
