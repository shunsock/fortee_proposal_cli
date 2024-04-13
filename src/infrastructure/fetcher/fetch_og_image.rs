use bytes::Bytes;
use reqwest::Error;

pub struct ImageFetcherResult {
    pub(crate) bytes_format_image: Bytes,
    pub(crate) file_extension: String,
}

#[tokio::main]
pub async fn fetch_og_image(url: &str) -> Result<ImageFetcherResult, Error> {
    let response = reqwest::blocking::get(url)?;

    // Check content type and get image data as bytes
    match response.headers().get("content-type") {
        Some(content_type) => {
            let content_type_str = content_type.to_str().unwrap();
            let file_extension = content_type_to_file_extension(content_type_str).to_string();
            let bytes_format_image: Bytes = response.bytes()?;

            Ok(ImageFetcherResult {
                bytes_format_image,
                file_extension,
            })
        }
        None => {
            panic!("Failed to get content type");
        }
    }
}

fn content_type_to_file_extension(content_type: &str) -> &str {
    match content_type {
        "image/jpeg" => "jpg",
        "image/png" => "png",
        "image/gif" => "gif",
        _ => "jpg",
    }
}
