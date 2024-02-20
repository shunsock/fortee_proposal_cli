use crate::infrastructure::fetcher::fetch_fortee_proposal_page::fetch_fortee_proposal_page;
use crate::infrastructure::writer::write_html_from_string::write_html_from_string;
use crate::presentation::terminal_message_presenter::ConsoleMessager;
use crate::presentation::terminal_message_presenter::MessageType;

pub fn download_html_page(url: &str) {
    let error_message =
        ConsoleMessager::new("Failed to fetch url".to_string(), MessageType::Failed);

    let http_response = fetch_fortee_proposal_page(url)
        .unwrap_or_else(|_| panic!("{}", error_message.supply_message()));
    let html_text = http_response;

    write_html_from_string(&html_text)
        .unwrap_or_else(|_| panic!("{}", error_message.supply_message()));
}
