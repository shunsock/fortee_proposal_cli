extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ProposalDataModel {
    title: String,
    schedule: String,
    track: String,
    speaker: String,
    proposal_page_url: String,
    og_image_url: String,
}

impl ProposalDataModel {
    pub fn new(
        title: String,
        schedule: String,
        track: String,
        speaker: String,
        proposal_page_url: String,
        og_image_url: String,
    ) -> Self {
        ProposalDataModel {
            title,
            schedule,
            track,
            speaker,
            proposal_page_url,
            og_image_url,
        }
    }

    pub fn get_og_image_url(&self) -> &str {
        &self.og_image_url
    }
}
