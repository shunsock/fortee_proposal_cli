use std::path::PathBuf;

pub trait FilePathProviderTrait {
    fn get_path(&self) -> PathBuf;
}
