use crate::domain::proposal::proposal_model::ProposalModel;
use serde_json;
use serde_json::Error;

pub fn show_json(proposal: &ProposalModel) -> Result<String, Error> {
    // TODO: move this function to pretty_json_presenter.rs
    let pretty_json = serde_json::to_string_pretty(proposal)?;
    Ok(pretty_json)
}
