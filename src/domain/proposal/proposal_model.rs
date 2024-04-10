extern crate serde;
extern crate serde_json;

use crate::presentation::send_message::{send_message_to_console, RunningStatus};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ProposalModel {
    pub(crate) title: String,
    pub(crate) schedule: String,
    pub(crate) track: String,
    pub(crate) speaker: String,
    pub(crate) og_image_url: String,
}

impl ProposalModel {
    pub fn new(
        title: String,
        schedule: String,
        track: String,
        speaker: String,
        og_image_url: String,
    ) -> ProposalModel {
        ProposalModel {
            title,
            schedule,
            track,
            speaker,
            og_image_url,
        }
    }
    pub fn print(&self) {
        let json_formatted_string = serde_json::to_string_pretty(self);
        match json_formatted_string {
            Ok(json) => {
                send_message_to_console(
                    RunningStatus::Success,
                    &format!("Show Proposal Information: \n{}", json),
                );
            }
            Err(e) => {
                send_message_to_console(RunningStatus::Failed, &format!("Error: {}", e));
            }
        }
    }
}
