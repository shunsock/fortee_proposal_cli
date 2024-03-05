use crate::infrastructure::writer::write_image_from_bytes::write_image_from_bytes;
use crate::presentation::terminal_message_presenter::ConsoleMessenger;
use crate::presentation::terminal_message_presenter::MessageType;

pub fn download_og_image(url: &str, file_base_name: &str) -> String {
    let error_message = ConsoleMessenger::new(
        "Failed to download image data".to_string(),
        MessageType::Failed,
    );

    // fetch image data from url
    let response = reqwest::blocking::get(url)
        .unwrap_or_else(|_| panic!("{}", error_message.supply_message()));

    // check file extension
    let content_type = response.headers().get("content-type").unwrap();
    let content_type_str = content_type.to_str().unwrap();
    let file_extension = match content_type_str {
        "image/jpeg" => "jpg",
        "image/png" => "png",
        "image/gif" => "gif",
        _ => "jpg",
    };

    // get image data as bytes
    let buffer = response
        .bytes()
        .unwrap_or_else(|_| panic!("{}", error_message.supply_message()));

    // location to save
    let save_file_path = show_file_path(file_base_name, file_extension, "data/proposal/image");

    write_image_from_bytes(&save_file_path, buffer);

    save_file_path
}

fn show_file_path(file_base_name: &str, file_extension: &str, save_directory: &str) -> String {
    format!("{}/{}.{}", save_directory, file_base_name, file_extension)
}
