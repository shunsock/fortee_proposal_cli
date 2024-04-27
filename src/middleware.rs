use crate::controller::get_proposal_data_controller::GetProposalDataDto;
use crate::infrastructure::validator::validate_url::validate_fortee_proposal_page_url;
use crate::presentation::send_message::{send_message_to_console, RunningStatus};
use crate::Args as Arg;

pub fn middleware_for_get_proposal_data(arg: &Arg) -> GetProposalDataDto {
    // Validate the URL
    match validate_fortee_proposal_page_url(&arg.url) {
        Ok(_) => {}
        Err(e) => {
            let error_message = format!("URL is invalid: {}", e);
            send_message_to_console(RunningStatus::Failed, &error_message);
            std::process::exit(1);
        }
    }
    let url: String = arg.url.clone();

    // Check if the user wants to output the OG image
    let output_og_image = match &arg.output_og_image {
        Some(true) => true,
        Some(false) => false,
        None => false,
    };

    GetProposalDataDto {
        url,
        output_og_image,
    }
}
