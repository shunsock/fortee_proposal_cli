use bytes::Bytes;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub fn write_image_from_bytes(save_file_path: PathBuf, buffer: Bytes) -> PathBuf {
    let mut file = File::create(save_file_path.clone()).expect("[Error] Failed to create file");

    file.write_all(&buffer)
        .expect("[Error] Failed to save image");

    save_file_path
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
        assert_eq!(res, test_file_path);

        std::fs::remove_file(test_file_path).unwrap();
    }
}