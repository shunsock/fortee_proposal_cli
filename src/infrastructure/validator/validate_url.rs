use regex::Regex;
use std::error::Error;

/// Validate the URL of the Fortee proposal page.
/// ex. https://fortee.jp/example-site/proposal/abc
/// ```
/// use crate::infrastructure::validator::validate_fortee_proposal_page_url;
///
/// let url = "https://fortee.jp/example-site/proposal/abc";
/// let result = validate_fortee_proposal_page_url(url);
/// assert!(result.is_ok());
/// ```
pub(crate) fn validate_fortee_proposal_page_url(url: &str) -> Result<bool, Box<dyn Error>> {
    // url pattern: https://fortee.jp/{conference_name}/proposal/abc...
    let url_pattern = r"^https://fortee\.jp/[a-z0-9\-]+/proposal/[a-z0-9\-]+";
    let re = Regex::new(url_pattern)?;

    if !re.is_match(url) {
        Err("URL does not start with the required pattern".into())
    } else {
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_url() {
        let valid_urls = vec![
            "https://fortee.jp/phpcon-2023/proposal/36cd0677-f513-4b63-944a-8255a372f466",
            "https://fortee.jp/phperkaigi-2024/proposal/456b328a-ab14-4301-b536-0d3c5fd237cc",
            "https://fortee.jp/yapc-hiroshima-2024/proposal/0c8c9df5-8db4-40a9-b3ea-97ea0c823ae7",
        ];

        for url in valid_urls {
            assert!(
                validate_fortee_proposal_page_url(url).is_ok(),
                "Expected URL to be valid: {}",
                url
            );
        }
    }

    #[test]
    fn test_invalid_url() {
        let invalid_urls = vec![
            "http://fortee.jp/example-site/proposal/abc",     // http
            "https://fortee.jpexample-site/proposal/abc",     // no slash
            "https://fortee.com/123-testing/proposal/abc",    // different domain
            "https://fortee.jp/example-site/notproposal/abc", // not proposal
            "fortee.jp/example-site/proposal/abc",            // no protocol
        ];

        for url in invalid_urls {
            assert!(
                validate_fortee_proposal_page_url(url).is_err(),
                "Expected URL to be invalid: {}",
                url
            );
        }
    }
}
