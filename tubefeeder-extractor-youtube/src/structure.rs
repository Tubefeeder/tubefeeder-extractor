extern crate quick_xml;
extern crate serde;

use chrono::NaiveDateTime;

use serde::Deserialize;

/// The youtube feed.
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct Feed {
    #[serde(rename = "entry")]
    pub entries: Vec<Entry>,
}

/// A single entry
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct Entry {
    pub title: String,
    pub author: Author,
    pub link: Link,
    #[serde(with = "date_serializer")]
    pub published: NaiveDateTime,
    #[serde(rename = "media_group")]
    pub media: Media,
}

/// The author of the video.
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct Author {
    pub name: String,
    pub uri: String,
}

/// The media information of the video. Only used for the thumbnail.
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct Media {
    #[serde(rename = "media_thumbnail")]
    pub thumbnail: Thumbnail,
}

/// The thumbnail link of the video.
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct Thumbnail {
    pub url: String,
}

/// The link to the video.
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct Link {
    pub href: String,
}

/// Deserializing `NativeDateTime`
mod date_serializer {
    use chrono::NaiveDateTime;

    use serde::{de::Error, Deserialize, Deserializer};

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<NaiveDateTime, D::Error> {
        let time: String = Deserialize::deserialize(deserializer)?;
        NaiveDateTime::parse_from_str(&time, "%Y-%m-%dT%H:%M:%S+00:00").map_err(D::Error::custom)
    }
}
