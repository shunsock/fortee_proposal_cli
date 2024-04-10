use std::fs::File;
use std::io;
use std::io::Write;
use std::path::PathBuf;

use crate::domain::proposal::proposal_model::ProposalModel;

pub fn write_json_from_proposal(proposal: &ProposalModel, file_path: PathBuf) -> io::Result<()> {
    // Serialize the proposal struct to a JSON string
    let serialized = serde_json::to_string(proposal).expect("Failed to serialize proposal");

    // Create a file and write the serialized string to it
    let mut file = File::create(file_path)?;
    file.write_all(serialized.as_bytes())?;

    Ok(())
}
