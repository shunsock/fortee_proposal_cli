use std::env;
use std::path::PathBuf;

pub fn get_file_path(
    directory_name: &String,
    file_base_name: &String,
    extension: &String,
) -> PathBuf {
    let home_dir: String = env::var("HOME").expect("HOME directory not found");

    PathBuf::from(home_dir)
        .join(".fortee")
        .join(directory_name)
        .join(format!("{}.{}", file_base_name, extension))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test whether the file path which exists is generated correctly
    /// If you have already installed the command, there are $HOME/.fortee/test/test.txt
    #[test]
    fn test_success_for_file_exists() {
        let directory_name: String = "test".to_string();
        let file_base_name: String = "test".to_string();
        let extension: String = "txt".to_string();

        let path: PathBuf = get_file_path(&directory_name, &file_base_name, &extension);

        // if path exists, it is correct
        assert!(path.exists());
    }
}
