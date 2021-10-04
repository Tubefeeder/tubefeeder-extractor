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
pub(crate) struct Rss {
    pub(crate) channel: Channel,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Channel {
    pub(crate) title: String,
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
