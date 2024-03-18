use crate::infrastructure::fetcher::fetch_fortee_proposal_page::fetch_fortee_proposal_page;
use crate::infrastructure::file_path_provider::file_path_provider_trait::FilePathProviderTrait;
use crate::infrastructure::file_path_provider::html_file_path_provider::HtmlFilePathProvider;
use crate::infrastructure::writer::write_html_from_string::write_html_from_string;
use crate::presentation::terminal_message_presenter::ConsoleMessenger;
use crate::presentation::terminal_message_presenter::MessageType;
use std::path::PathBuf;

pub fn download_html_page(url: &str) {
    let error_message =
        ConsoleMessenger::new("Failed to fetch url".to_string(), MessageType::Failed);

    /* Fetch the html text */
    let http_response = fetch_fortee_proposal_page(url)
        .unwrap_or_else(|_| panic!("{}", error_message.supply_message()));
    let html_text = http_response;

    /* Save the html text to a file */
    let html_file_path_provider = HtmlFilePathProvider::new("proposal");
    let file_save_path: PathBuf = html_file_path_provider.get_path();

    write_html_from_string(&html_text, file_save_path)
        .unwrap_or_else(|_| panic!("{}", error_message.supply_message()));
}
