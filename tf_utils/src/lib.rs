pub mod rss;

use crate::rss::Rss;

use tf_core::{NetworkError, ParseError};

pub async fn parse_rss_from_url(
    url: &str,
    client: &reqwest::Client,
) -> Result<Rss, tf_core::Error> {
    let response = client.get(url.clone()).send().await;

    if response.is_err() {
        log::error!("Error getting {:?}", url);
        return Err(NetworkError(url.to_string()).into());
    }

    let body_res = response.unwrap().text().await;

    if body_res.is_err() {
        log::error!("Error getting {:?}", url);
        return Err(NetworkError(url.to_string()).into());
    }

    let body_parsable = body_res
        .unwrap()
        .replace("media:", "media/")
        .replace("itunes:", "itunes/");

    let rss_res: Result<Rss, quick_xml::de::DeError> = quick_xml::de::from_str(&body_parsable);

    if rss_res.is_err() {
        log::error!("Error parsing: {}", &rss_res.err().unwrap());
        return Err(ParseError(body_parsable).into());
    }

    Ok(rss_res.unwrap())
}
