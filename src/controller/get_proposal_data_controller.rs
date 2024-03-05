use crate::presentation::pretty_json_presenter::print_pretty_json;
use crate::presentation::terminal_message_presenter::ConsoleMessenger;
use crate::presentation::terminal_message_presenter::MessageType;
use crate::use_case::build_structured_information::builder::build_structured_proposal_information;
use crate::use_case::download_html_page::downloader::download_html_page;
use crate::use_case::download_og_image::downloader::download_og_image;

pub fn get_proposal_data_controller(url: &str) {
    /*
     * Download HTML page from the given URL
     */
    download_html_page(url);
    let success_to_download_html_message = ConsoleMessenger::new(
        "HTML page has been successfully downloaded.".to_string(),
        MessageType::Success,
    );
    success_to_download_html_message.show_message();

    /*
     * Extract structured information from the downloaded HTML page
     */
    let proposal = build_structured_proposal_information();
    let success_to_get_information_message = ConsoleMessenger::new(
        "Structured proposal information has been successfully created.".to_string(),
        MessageType::Success,
    );
    success_to_get_information_message.show_message();

    /*
     * Print the structured information
     */
    print_pretty_json(&proposal);

    /*
     * Download OG image from the structured information
     */
    let image_path = download_og_image(&proposal.og_image_url, &proposal.title);
    let og_image_message = ConsoleMessenger::new(
        format!("OG Image is saved: `{}`", image_path),
        MessageType::Warning,
    );
    og_image_message.show_message();
}
