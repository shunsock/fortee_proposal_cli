use crate::domain::proposal::proposal_model::ProposalModel;
use crate::presentation::send_message::{send_message_to_console, RunningStatus};

pub fn print_pretty_json(proposal: &ProposalModel) {
    let pretty_json = serde_json::to_string_pretty(proposal);
    match pretty_json {
        Ok(json) => {
            send_message_to_console(RunningStatus::Success, &json);
        }
        Err(e) => {
            send_message_to_console(RunningStatus::Failed, &format!("Error: {}", e));
        }
    }
}
