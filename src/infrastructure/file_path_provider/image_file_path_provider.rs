use super::file_path_provider_trait::FilePathProviderTrait;
use super::get_file_path::get_file_path;
use std::path::PathBuf;

pub(crate) struct ImageFilePathProvider {
    directory_name: String,
    file_base_name: String,
    extension: String,
}

impl ImageFilePathProvider {
    pub fn new(file_base_name: &str, extension: &str) -> Self {
        let directory_name: &str = "image";

        ImageFilePathProvider {
            directory_name: directory_name.to_string(),
            file_base_name: file_base_name.to_string(),
            extension: extension.to_string(),
        }
    }
}

impl FilePathProviderTrait for ImageFilePathProvider {
    fn get_path(&self) -> PathBuf {
        get_file_path(&self.directory_name, &self.file_base_name, &self.extension)
    }
}
