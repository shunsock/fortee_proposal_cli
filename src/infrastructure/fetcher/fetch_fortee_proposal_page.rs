use reqwest::Error;

#[tokio::main]
pub async fn fetch_fortee_proposal_page(url: &str) -> Result<String, Error> {
    let response: String = reqwest::get(url).await?.text().await?;

    return Ok(response);
}
