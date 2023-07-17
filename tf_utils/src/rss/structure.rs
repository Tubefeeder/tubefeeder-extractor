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

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Rss {
    pub channel: Channel,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Channel {
    pub title: String,
    #[serde(rename = "itunes/author")]
    #[serde(default)]
    pub itunes_author: String,
    #[serde(rename = "item")]
    pub items: Vec<Item>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    #[serde(rename = "media/title")]
    #[serde(default)]
    pub media_title: String,
    #[serde(rename = "itunes/title")]
    #[serde(default)]
    pub itunes_title: String,

    pub link: String,
    #[serde(with = "rss_date_format")]
    pub pub_date: chrono::NaiveDateTime,

    #[serde(rename = "media/thumbnail")]
    #[serde(default)]
    pub media_thumbnail: Vec<MediaThumbnail>,
    #[serde(rename = "itunes/image")]
    #[serde(default)]
    pub itunes_image: ItunesImage,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MediaThumbnail {
    pub url: String,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ItunesImage {
    pub href: String,
}

mod rss_date_format {
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
