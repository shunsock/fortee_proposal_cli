extern crate serde;
extern crate serde_json;

use crate::domain::proposal::proposal_json_file::ProposalJsonFile;
use crate::presentation::send_message::{send_message_to_console, RunningStatus};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

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

    pub fn write_as_json(&self) -> Result<bool, String> {
        let json = serde_json::to_string_pretty(self).unwrap_or_else(|_| {
            panic!("Failed to convert the proposal information to JSON format")
        });

        let file_path: PathBuf = ProposalJsonFile::new().path;
        let res = std::fs::write(file_path, json);
        match res {
            Ok(_) => Ok(true),
            Err(e) => Err(format!("Error: {}", e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print() {
        let title: String = "title".to_string();
        let schedule: String = "schedule".to_string();
        let track: String = "track".to_string();
        let speaker: String = "speaker".to_string();
        let og_image_url: String = "og_image_url".to_string();

        let proposal = ProposalModel::new(title, schedule, track, speaker, og_image_url);

        proposal.print();
    }

    #[test]
    fn test_write_as_json() {
        let title: String = "title".to_string();
        let schedule: String = "schedule".to_string();
        let track: String = "track".to_string();
        let speaker: String = "speaker".to_string();
        let og_image_url: String = "og_image_url".to_string();

        let proposal = ProposalModel::new(title, schedule, track, speaker, og_image_url);

        let res = proposal.write_as_json();
        match res {
            Ok(_) => {}
            Err(e) => {
                panic!("{}", e);
            }
        }
    }
}
