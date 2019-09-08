#![allow(dead_code)]

use std::time::Duration;

use reqwest;

pub mod parse;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn build_client() -> Result<reqwest::Client> {
    let client = reqwest::Client::builder()
        .gzip(true)
        .timeout(Duration::from_secs(10))
        .build()?;
    Ok(client)
}

/// Pulls the target twtxt.txt file from the specified URL.
///
/// # Examples
/// ```
/// # use rustwtxt::*;
/// let out = if let Ok(data) = pull_twtxt("https://some-url-here.ext/twtxt.txt") {
///     data
/// } else {
///     String::new()
/// };
/// ```
pub fn pull_twtxt(url: &str) -> Result<String> {
    let client = build_client()?;
    let res = client.get(url).send()?.text()?;
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_URL: &str = "https://gbmor.dev/twtxt.txt";

    #[test]
    fn test_build_client() {
        // Right now, just panic if it returns Err()
        build_client().unwrap();
    }
    #[test]
    fn test_pull_twtxt() {
        let res = pull_twtxt(TEST_URL).unwrap();
        eprintln!("{}", res);
        assert!(res.contains("gbmor"));
    }

    #[test]
    #[should_panic]
    fn test_bad_url() {
        pull_twtxt("https://example-some-fake-site-goes-here.com/some_fake_url.txt").unwrap();
    }
}
