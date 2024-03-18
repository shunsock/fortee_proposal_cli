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
