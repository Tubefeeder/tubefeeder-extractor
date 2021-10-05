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

use std::path::Path;

use crate::subscription::YTSubscription;

use async_trait::async_trait;
use piped::RelatedStream;
use tf_core::ErrorStore;

const PIPED_URL: &'static str = "https://piped.kavin.rocks";
const USER_AGENT: &'static str =
    "Mozilla/5.0 (Windows NT 10.0; rv:78.0) Gecko/20100101 Firefox/78.0";

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct YTVideo {
    pub(crate) url: String,
    pub(crate) title: String,
    pub(crate) uploaded: chrono::NaiveDateTime,
    pub(crate) subscription: YTSubscription,
    pub(crate) thumbnail_url: String,
}

impl YTVideo {
    pub fn new<T: AsRef<str>>(
        url: T,
        title: T,
        uploaded: chrono::NaiveDateTime,
        subscription: YTSubscription,
        thumbnail_url: T,
    ) -> Self {
        Self {
            url: url.as_ref().to_owned(),
            title: title.as_ref().to_owned(),
            uploaded,
            subscription,
            thumbnail_url: thumbnail_url.as_ref().to_owned(),
        }
    }

    pub fn thumbnail_url(&self) -> String {
        self.thumbnail_url.clone()
    }
}

#[async_trait]
impl tf_core::Video for YTVideo {
    type Subscription = YTSubscription;

    fn url(&self) -> String {
        self.url.clone()
    }

    fn title(&self) -> String {
        self.title.clone()
    }

    fn subscription(&self) -> Self::Subscription {
        self.subscription.clone()
    }

    fn uploaded(&self) -> chrono::NaiveDateTime {
        self.uploaded
    }

    async fn thumbnail_with_client<P: AsRef<Path> + Send>(
        &self,
        client: &reqwest::Client,
        filename: P,
        width: i32,
        height: i32,
    ) {
        log::debug!("Getting thumbnail for youtube video {}", self.title);
        log::debug!("Getting thumbnail for youtube url {}", self.thumbnail_url);
        let response = client
            .get(&self.thumbnail_url)
            .header("User-Agent", USER_AGENT)
            .send()
            .await;
        log::debug!(
            "Got response for thumbnail for youtube video {}",
            self.title
        );

        if response.is_err() {
            log::error!(
                "Failed getting thumbnail for youtube video {}, use default",
                self.title
            );
            self.default_thumbnail(filename, width, height);
            return;
        }

        let parsed = response.unwrap().bytes().await;

        if parsed.is_err() {
            log::error!(
                "Failed getting thumbnail for youtube video {}, use default",
                self.title
            );
            self.default_thumbnail(filename, width, height);
            return;
        }

        let parsed_bytes = parsed.unwrap();

        let webp_decoder = webp::Decoder::new(&parsed_bytes);
        let webp_image = webp_decoder.decode();
        let dynamic_image = webp_image.map(|i| i.to_image());

        let rgba_image = dynamic_image.map(|i| i.to_rgba8());

        if let Some(image) = rgba_image {
            let _ = image.save(filename);
        } else {
            self.default_thumbnail(filename, width, height);
        }
    }
}

impl YTVideo {
    pub(crate) fn from_related_stream(
        _errors: &ErrorStore,
        v: RelatedStream,
        subscription: YTSubscription,
    ) -> Self {
        YTVideo {
            url: format!("{}/{}", PIPED_URL, v.url),
            title: v.title,
            subscription,
            // TODO: Date
            uploaded: v
                .uploaded_date
                .map(|d| timeago_parser(d).ok())
                .unwrap_or(None)
                .unwrap_or(chrono::NaiveDate::from_ymd(1, 1, 1).and_hms(0, 0, 0)),
            thumbnail_url: v.thumbnail,
        }
    }
}

// TODO: Move to util crate
/// Parse textual upload date (e.g. `4 months ago`) to a approximate date.
fn timeago_parser<S: AsRef<str>>(date: S) -> Result<chrono::NaiveDateTime, tf_core::ParseError> {
    let duration_ago = parse_duration::parse(date.as_ref())
        .map_err(|_| tf_core::ParseError("Parsing date".to_string()))?;
    return Ok(
        chrono::Local::now().naive_local() - chrono::Duration::from_std(duration_ago).unwrap()
    );
}
