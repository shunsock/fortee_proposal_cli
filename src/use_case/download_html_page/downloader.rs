use crate::infrastructure::fetcher::fetch_fortee_proposal_page::fetch_fortee_proposal_page;
use crate::infrastructure::writer::write_html_from_string::write_html_from_string;
use crate::presentation::terminal_message_presenter::ConsoleMessenger;
use crate::presentation::terminal_message_presenter::MessageType;

use std::env;
use std::path::PathBuf;

pub fn download_html_page(url: &str) {
    let error_message =
        ConsoleMessenger::new("Failed to fetch url".to_string(), MessageType::Failed);

    let http_response = fetch_fortee_proposal_page(url)
        .unwrap_or_else(|_| panic!("{}", error_message.supply_message()));
    let html_text = http_response;

    let home_directory = env::var("HOME").expect("Failed to get home directory");
    let file_save_path = PathBuf::from(home_directory)
        .join(".fortee")
        .join("html")
        .join("proposal.html");

    write_html_from_string(&html_text, file_save_path)
        .unwrap_or_else(|_| panic!("{}", error_message.supply_message()));
}
