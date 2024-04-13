use crate::domain::proposal::proposal_image_file::ProposalImageFileWriter;
use crate::infrastructure::fetcher::fetch_og_image::{fetch_og_image, ImageFetcherResult};
use std::path::PathBuf;

pub fn download_og_image(url: &str) -> Result<PathBuf, String> {
    let image_fetcher_result: ImageFetcherResult = match fetch_og_image(url) {
        Ok(result) => result,
        Err(e) => return Err(format!("Error: {}", e)),
    };

    let proposal_image_file_writer: ProposalImageFileWriter = ProposalImageFileWriter::new(
        image_fetcher_result.bytes_format_image.clone(),
        &image_fetcher_result.file_extension,
    );

    match proposal_image_file_writer.write() {
        Ok(_) => Ok(proposal_image_file_writer.get_path()),
        Err(e) => Err(format!("Error: {}", e)),
    }
}
