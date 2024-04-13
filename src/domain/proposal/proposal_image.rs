use crate::infrastructure::file_path_provider::show_home_dir::show_home_dir;
use crate::infrastructure::writer::write_image_from_bytes::write_image_from_bytes;
use bytes::Bytes;
use std::path::PathBuf;

pub struct ProposalImageFileWriter {
    path: PathBuf,
    value: Bytes,
}

fn show_proposal_image_file_path(file_extension: &str) -> PathBuf {
    let home_dir = show_home_dir();
    let file_path: PathBuf = PathBuf::from(home_dir)
        .join(".fortee")
        .join("html")
        .join(file_extension);

    file_path
}

impl ProposalImageFileWriter {
    pub fn new(byte_format_image: Bytes, file_extension: &str) -> Self {
        let file_path: PathBuf = show_proposal_image_file_path(file_extension);

        ProposalImageFileWriter {
            path: file_path,
            value: byte_format_image,
        }
    }

    pub fn get_path(&self) -> PathBuf {
        self.path.clone()
    }

    pub fn write(&self) -> Result<bool, String> {
        match write_image_from_bytes(self.path.clone(), self.value.clone()) {
            Ok(_) => Ok(true),
            Err(e) => Err(format!("[Error] Failed to write file: {}", e)),
        }
    }
}
