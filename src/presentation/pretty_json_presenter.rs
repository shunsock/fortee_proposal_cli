use crate::domain::proposal::proposal_model::ProposalModel;
use crate::presentation::show_json::show_json;

pub fn print_pretty_json(proposal: &ProposalModel) {
    let pretty_json = show_json(proposal).unwrap();
    println!("{}", pretty_json);
}
