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

use crate::subscription::YTSubscription;

use async_trait::async_trait;
use piped::RelatedStream;
use tf_core::ErrorStore;

const YOUTUBE_URL: &'static str = "https://www.youtube.com";

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

    fn thumbnail_url(&self) -> String {
        self.thumbnail_url.clone()
    }

    fn uploaded(&self) -> chrono::NaiveDateTime {
        self.uploaded
    }

    /// The default `image`-crate currently only supports webp as grayscale, therefore this has to be overwritten.
    fn convert_image(data: &[u8]) -> Option<image::DynamicImage> {
        let webp_decoder = webp::Decoder::new(&data);
        let webp_image = webp_decoder.decode();
        webp_image.map(|i| i.to_image())
    }
}

impl YTVideo {
    pub(crate) fn from_related_stream(
        _errors: &ErrorStore,
        v: RelatedStream,
        subscription: YTSubscription,
    ) -> Self {
        YTVideo {
            url: format!("{}/{}", YOUTUBE_URL, v.url),
            title: v.title,
            subscription,
            uploaded: v
                .uploaded_date
                .map(|d| tf_utils::timeago_parser(d).ok())
                .unwrap_or(None)
                .unwrap_or(chrono::NaiveDate::from_ymd(1, 1, 1).and_hms(0, 0, 0)),
            thumbnail_url: v.thumbnail,
        }
    }
}
