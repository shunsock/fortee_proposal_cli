use crate::infrastructure::file_path_provider::file_path_provider_trait::FilePathProviderTrait;
use crate::infrastructure::file_path_provider::json_file_path_provider::JsonFilePathProvider;
use std::path::PathBuf;

pub struct ProposalJsonFile {
    directory_name: String,
}

impl ProposalJsonFile {
    pub fn new() -> Self {
        ProposalJsonFile {
            directory_name: "proposal".to_string(),
        }
    }

    pub fn get_file_path(&self) -> PathBuf {
        let path_provider = JsonFilePathProvider::new(&self.directory_name);
        path_provider.get_path()
    }
}
