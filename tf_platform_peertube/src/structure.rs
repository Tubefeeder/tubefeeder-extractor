use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct Rss {
    pub(crate) channel: Channel,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Channel {
    #[serde(rename = "item")]
    pub(crate) items: Vec<Item>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Item {
    #[serde(rename = "media/title")]
    pub(crate) title: String,
    pub(crate) link: String,
    #[serde(with = "peertube_date_format")]
    pub(crate) pub_date: chrono::NaiveDateTime,
    #[serde(rename = "media/thumbnail")]
    pub(crate) thumbnail: Thumbnail,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Thumbnail {
    pub(crate) url: String,
}

mod peertube_date_format {
    use chrono::NaiveDateTime;
    use serde::{self, Deserialize, Deserializer};

    const FORMAT: &'static str = "%a, %d %b %Y %H:%M:%S %Z";

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}
