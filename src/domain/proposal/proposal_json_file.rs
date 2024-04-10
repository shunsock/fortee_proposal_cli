use crate::infrastructure::file_path_provider::show_home_dir::show_home_dir;
use std::path::PathBuf;

/// ProposalJsonFile provides the file path of the proposal.json file
/// The file path is $HOME/.fortee/json/proposal.json
/// You MUST use this through ProposalJsonFile::new() to get the instance
pub struct ProposalJsonFile {
    pub(crate) path: PathBuf,
}

impl ProposalJsonFile {
    pub fn new() -> Self {
        let home_dir: String = show_home_dir();
        let file_path = PathBuf::from(home_dir)
            .join(".fortee")
            .join("json")
            .join("proposal.json");

        ProposalJsonFile { path: file_path }
    }

    pub fn get_file_path(&self) -> PathBuf {
        self.path.clone()
    }
}
