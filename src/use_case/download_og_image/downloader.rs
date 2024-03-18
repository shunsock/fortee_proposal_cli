use crate::infrastructure::fetcher::fetch_og_image::fetch_og_image;
use crate::infrastructure::file_path_provider::file_path_provider_trait::FilePathProviderTrait;
use crate::infrastructure::file_path_provider::image_file_path_provider::ImageFilePathProvider;
use crate::infrastructure::writer::write_image_from_bytes::write_image_from_bytes;
use crate::presentation::terminal_message_presenter::ConsoleMessenger;
use crate::presentation::terminal_message_presenter::MessageType;
use std::path::PathBuf;

pub fn download_og_image(url: &str, file_base_name: &str) -> Result<PathBuf, ConsoleMessenger> {
    let error_message = ConsoleMessenger::new(
        "Failed to download image data".to_string(),
        MessageType::Failed,
    );

    let image_fetcher_result = fetch_og_image(url);

    match image_fetcher_result {
        Ok(image_fetcher_result) => {
            /* get path to save our image */
            let image_file_path_provider =
                ImageFilePathProvider::new(file_base_name, &image_fetcher_result.file_extension);
            let save_file_path = image_file_path_provider.get_path();

            /* save image */
            let saved_file_path =
                write_image_from_bytes(save_file_path, image_fetcher_result.bytes_format_image);

            Ok(saved_file_path)
        }
        Err(_) => Err(error_message),
    }
}
