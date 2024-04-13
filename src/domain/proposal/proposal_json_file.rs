use super::proposal_data_model::ProposalDataModel;
use crate::infrastructure::file_path_provider::show_home_dir::show_home_dir;
use crate::infrastructure::reader::read_file::read_file_as_string;
use crate::infrastructure::writer::write_string_to_file::write_string_to_file;
use std::path::PathBuf;

pub fn show_proposal_json_file_path() -> PathBuf {
    let home_dir: String = show_home_dir();
    PathBuf::from(home_dir)
        .join(".fortee")
        .join("json")
        .join("proposal.json")
}

/// ProposalJsonFile provides the file path of the proposal.json file
/// The file path is $HOME/.fortee/json/proposal.json
/// You MUST use this through ProposalJsonFile::new() to get the instance
pub struct ProposalJsonFile {
    path: PathBuf,
    value: ProposalDataModel,
}

impl ProposalJsonFile {
    pub fn new() -> Self {
        let file_path: PathBuf = show_proposal_json_file_path();

        /*
         * Read the file as a string
         */
        let json: String = match read_file_as_string(file_path.clone()) {
            Ok(json) => json,
            Err(e) => {
                panic!("Failed to read the file: {:?}", e);
            }
        };

        /*
         * Deserialize the JSON string to a ProposalDataModel
         */
        let proposal_data: ProposalDataModel = match serde_json::from_str(&json) {
            Ok(value) => value,
            Err(e) => {
                panic!("Failed to deserialize the JSON: {:?}", e);
            }
        };

        ProposalJsonFile {
            path: file_path,
            value: proposal_data,
        }
    }

    pub fn get_file_path(&self) -> PathBuf {
        self.path.clone()
    }

    pub fn get_pretty_json_string(&self) -> Result<String, String> {
        let pretty_json_string = match serde_json::to_string_pretty(&self.value) {
            Ok(pretty_json_string) => pretty_json_string,
            Err(e) => {
                return Err(format!("Failed to serialize the JSON: {:?}", e));
            }
        };

        Ok(pretty_json_string)
    }

    pub fn get_proposal_data(&self) -> &ProposalDataModel {
        &self.value
    }
}

pub struct ProposalJsonFileWriter {
    file_path: PathBuf,
    value: ProposalDataModel,
}

impl ProposalJsonFileWriter {
    pub fn new(value: ProposalDataModel) -> Self {
        let file_path: PathBuf = show_proposal_json_file_path();
        ProposalJsonFileWriter { file_path, value }
    }

    pub fn write(&self) -> Result<bool, String> {
        let file_path: PathBuf = self.file_path.clone();

        /*
         * Serialize the value to a JSON string
         */
        let json_string: String = match serde_json::to_string(&self.value) {
            Ok(json_string) => json_string,
            Err(e) => {
                panic!("Failed to serialize the value: {:?}", e);
            }
        };

        /*
         *  Write the JSON string to the file
         */
        let write_result = write_string_to_file(&json_string, file_path);
        match write_result {
            Ok(_) => Ok(true),
            Err(e) => Err(e.to_string()),
        }
    }
}
