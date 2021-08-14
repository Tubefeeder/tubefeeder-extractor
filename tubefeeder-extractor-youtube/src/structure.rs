/*
 * Copyright 2021 Julian Schmidhuber <github@schmiddi.anonaddy.com>
 *
 * This file is part of Tubefeeder-extractor.
 *
 * Tubefeeder-extractor is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Tubefeeder-extractor is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Tubefeeder-extractor.  If not, see <https://www.gnu.org/licenses/>.
 */

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
