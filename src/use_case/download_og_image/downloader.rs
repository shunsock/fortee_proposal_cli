use crate::infrastructure::fetcher::fetch_og_image::fetch_og_image;
use crate::infrastructure::writer::write_image_from_bytes::write_image_from_bytes;
use crate::presentation::terminal_message_presenter::ConsoleMessenger;
use crate::presentation::terminal_message_presenter::MessageType;

use std::env;
use std::path::PathBuf;

pub fn download_og_image(url: &str, file_base_name: &str) -> Result<PathBuf, ConsoleMessenger> {
    let error_message = ConsoleMessenger::new(
        "Failed to download image data".to_string(),
        MessageType::Failed,
    );

    let image_fetcher_result = fetch_og_image(url);
    match image_fetcher_result {
        Ok(image_fetcher_result) => {
            let home_dir = env::var("HOME").expect("HOME directory not found");
            let save_file_path =
                PathBuf::from(home_dir)
                    .join(".fortee")
                    .join("image")
                    .join(format!(
                        "{}.{}",
                        file_base_name, &image_fetcher_result.file_extension
                    ));

            let saved_file_path =
                write_image_from_bytes(save_file_path, image_fetcher_result.bytes_format_image);
            Ok(saved_file_path)
        }
        Err(_) => Err(error_message),
    }
}
