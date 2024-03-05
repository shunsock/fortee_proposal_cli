extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ProposalModel {
    pub(crate) title: String,
    pub(crate) schedule: String,
    pub(crate) track: String,
    pub(crate) speaker: String,
    pub(crate) og_image_url: String,
}
