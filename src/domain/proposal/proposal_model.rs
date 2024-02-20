extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ProposalModel {
    pub(crate) title: String,
    pub(crate) date: String,
    pub(crate) track: String,
    pub(crate) speaker: String,
    pub(crate) image_url: String,
}
