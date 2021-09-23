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
use gdk_pixbuf::gio::{MemoryInputStream, NONE_CANCELLABLE};
use gdk_pixbuf::Pixbuf;
use rusty_pipe::extractors::YTStreamInfoItemExtractor;
use tf_core::ErrorStore;

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
        let response = client.get(&self.thumbnail_url).send().await;
        log::debug!(
            "Got response for thumbnail for youtube video {}",
            self.title
        );

        if response.is_err() {
            log::debug!(
                "Failed getting thumbnail for youtube video {}, use default",
                self.title
            );
            self.default_thumbnail(filename, width, height);
            return;
        }

        let parsed = response.unwrap().bytes().await;

        if parsed.is_err() {
            log::debug!(
                "Failed getting thumbnail for youtube video {}, use default",
                self.title
            );
            self.default_thumbnail(filename, width, height);
            return;
        }

        let parsed_bytes = parsed.unwrap();

        let glib_bytes = glib::Bytes::from(&parsed_bytes.to_vec());

        let stream = MemoryInputStream::from_bytes(&glib_bytes);

        log::debug!(
            "Finished Getting thumbnail for youtube video {}",
            self.title
        );
        let pixbuf = Pixbuf::from_stream_at_scale(&stream, width, height, true, NONE_CANCELLABLE);
        if let Ok(pixbuf) = pixbuf {
            let _ = pixbuf.savev(filename, "png", &[]);
        } else {
            self.default_thumbnail(filename, width, height);
        }
    }
}

impl YTVideo {
    pub(crate) fn from_extractor(
        _errors: &ErrorStore,
        v: YTStreamInfoItemExtractor,
        subscription: YTSubscription,
    ) -> Self {
        YTVideo {
            url: v.url().unwrap_or("".to_string()),
            title: v.name().unwrap_or("".to_string()),
            subscription,
            uploaded: v
                .upload_date()
                .unwrap_or(chrono::NaiveDate::from_num_days_from_ce(0).and_hms(0, 0, 0)),
            thumbnail_url: v
                .thumbnails()
                .map(|v| v.get(0).map(|t| t.url.clone()))
                .unwrap_or(None)
                .unwrap_or("".to_string()),
        }
    }
}
