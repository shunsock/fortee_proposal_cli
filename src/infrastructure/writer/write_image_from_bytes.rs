use bytes::Bytes;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub fn write_image_from_bytes(save_file_path: PathBuf, buffer: Bytes) -> Result<bool, String> {
    let mut file = match File::create(save_file_path.clone()) {
        Ok(file) => file,
        Err(e) => return Err(format!("[Error] Failed to create file: {}", e)),
    };

    match file.write_all(&buffer) {
        Ok(_) => Ok(true),
        Err(e) => Err(format!("[Error] Failed to write file: {}", e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::Bytes;

    #[test]
    fn test_write_image_from_bytes() {
        let test_file_path = PathBuf::from("test.png");
        let test_content = Bytes::from("test content");

        let res = write_image_from_bytes(test_file_path.clone(), test_content);
        assert_eq!(res, Ok(true));

        std::fs::remove_file(test_file_path).unwrap();
    }
}
