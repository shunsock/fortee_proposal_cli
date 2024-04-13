use crate::infrastructure::file_path_provider::show_home_dir::show_home_dir;
use crate::infrastructure::reader::read_file::read_file_as_string;
use crate::infrastructure::writer::write_string_to_file::write_string_to_file;
use std::path::PathBuf;

/// ProposalJsonFile provides the file path of the proposal.json file
/// The file path is $HOME/.fortee/html/proposal.html
/// You MUST use this through ProposalJsonFile::new() to get the instance
pub struct ProposalHtml {
    html: String,
}

fn show_proposal_html_file_path() -> PathBuf {
    let home_dir = show_home_dir();
    let file_path: PathBuf = PathBuf::from(home_dir)
        .join(".fortee")
        .join("html")
        .join("proposal.html");

    file_path
}

impl ProposalHtml {
    pub fn new() -> Self {
        let file_path: PathBuf = show_proposal_html_file_path();
        let html: Result<String, String> = read_file_as_string(file_path);
        match html {
            Ok(html_value) => ProposalHtml { html: html_value },
            Err(e) => panic!("Error: {}", e),
        }
    }

    pub fn get_html(&self) -> String {
        self.html.clone()
    }
}

pub struct ProposalHtmlFileWriter {
    path: PathBuf,
    html: String,
}

impl ProposalHtmlFileWriter {
    pub fn new(html_string: String) -> Self {
        let file_path: PathBuf = show_proposal_html_file_path();

        ProposalHtmlFileWriter {
            path: file_path,
            html: html_string,
        }
    }

    pub fn write(&self) -> Result<bool, String> {
        let file_path: PathBuf = self.path.clone();
        let res = write_string_to_file(&self.html, file_path.clone());

        match res {
            Ok(_) => Ok(true),
            Err(e) => Err(format!("Error: {}", e)),
        }
    }
}
