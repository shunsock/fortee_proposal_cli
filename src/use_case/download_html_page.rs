use crate::infrastructure::fetcher::fetch_fortee_proposal_page::fetch_fortee_proposal_page;
use crate::infrastructure::file_path_provider::file_path_provider_trait::FilePathProviderTrait;
use crate::infrastructure::file_path_provider::html_file_path_provider::HtmlFilePathProvider;
use crate::infrastructure::writer::write_html_from_string::write_html_from_string;
use crate::presentation::send_message::send_message_as_string;
use crate::presentation::send_message::RunningStatus;
use std::path::PathBuf;

pub fn download_html_page(url: &str) {
    /* Fetch the html text */
    let http_response = fetch_fortee_proposal_page(url).unwrap_or_else(|_| {
        panic!(
            "{}",
            send_message_as_string(RunningStatus::Failed, "Failed to fetch url")
        )
    });
    let html_text = http_response;

    /* Save the html text to a file */
    let html_file_path_provider = HtmlFilePathProvider::new("proposal");
    let file_save_path: PathBuf = html_file_path_provider.get_path();

    write_html_from_string(&html_text, file_save_path).unwrap_or_else(|_| {
        panic!(
            "{}",
            send_message_as_string(RunningStatus::Failed, "Failed to write html to file")
        )
    });
}
