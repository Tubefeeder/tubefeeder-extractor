pub mod subscription;
pub mod video;

use std::collections::HashMap;
use std::str::FromStr;

use async_trait::async_trait;
use rusty_pipe::ParsingError;

pub(crate) struct Downloader;

#[async_trait]
impl rusty_pipe::Downloader for Downloader {
    async fn download(url: &str) -> Result<String, ParsingError> {
        log::debug!("Downloading youtube url {}", url);
        let resp = reqwest::get(url)
            .await
            .map_err(|er| ParsingError::DownloadError {
                cause: er.to_string(),
            })?;
        let body = resp
            .text()
            .await
            .map_err(|er| ParsingError::DownloadError {
                cause: er.to_string(),
            })?;
        log::trace!("Finished downloading and parsing");
        Ok(String::from(body))
    }

    async fn download_with_header(
        url: &str,
        header: HashMap<String, String>,
    ) -> Result<String, ParsingError> {
        log::debug!("Downloading youtube url {} with headers {:?}", url, header);
        let client = reqwest::Client::new();
        let res = client.get(url);
        let mut headers = reqwest::header::HeaderMap::new();
        for header in header {
            headers.insert(
                reqwest::header::HeaderName::from_str(&header.0).map_err(|e| e.to_string())?,
                header.1.parse().unwrap(),
            );
        }
        let res = res.headers(headers);
        let res = res.send().await.map_err(|er| er.to_string())?;
        let body = res.text().await.map_err(|er| er.to_string())?;
        log::trace!("Finished downloading and parsing");
        Ok(String::from(body))
    }

    fn eval_js(_script: &str) -> Result<String, String> {
        Ok("".to_owned())
    }
}
