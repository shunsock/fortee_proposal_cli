use std::fs::File;
use std::io;
use std::io::Write;
use std::path::PathBuf;

pub fn write_string_to_file(content: &str, file_save_path: PathBuf) -> Result<bool, io::Error> {
    let mut file = File::create(file_save_path)?;
    write!(file, "{}", content)?;
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_write_string_to_file() {
        let test_file_path = PathBuf::from("test.txt");
        let test_content = "test content";

        let res = write_string_to_file(test_content, test_file_path.clone());
        assert_eq!(res.is_ok(), true);

        let read_res = fs::read_to_string(test_file_path.clone());
        // assert if read_res is not Ok
        assert_eq!(read_res.is_ok(), true);
        assert_eq!(read_res.unwrap(), test_content);

        fs::remove_file(test_file_path).unwrap();
    }
}
