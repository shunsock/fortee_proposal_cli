use crate::domain::proposal::proposal_model::ProposalModel;
use serde_json;
use serde_json::Error;

pub fn show_json(proposal: &ProposalModel) -> Result<String, Error> {
    let pretty_json = serde_json::to_string_pretty(proposal)?;
    Ok(pretty_json)
}
