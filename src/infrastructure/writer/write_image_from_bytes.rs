use bytes::Bytes;
use std::fs::File;
use std::io::Write;

pub fn write_image_from_bytes(save_file_path: &str, buffer: Bytes) {
    let mut file = File::create(save_file_path).expect("[Error] Failed to create file");
    file.write_all(&buffer)
        .expect("[Error] Failed to save image");
}
