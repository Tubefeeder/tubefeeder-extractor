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

use crate::Subscription;

use async_trait::async_trait;
use gdk_pixbuf::{Colorspace, Pixbuf};

#[cfg(test)]
use {crate::mock::MockSubscription, mockall::predicate::*, mockall::*};

/// A [`Video`] that can come from any website.
#[async_trait]
pub trait Video:
    Clone + std::hash::Hash + std::cmp::Eq + std::marker::Send + std::marker::Sync
{
    /// The [Subscription] of type of this video.
    type Subscription: Subscription;

    /// The url that can be used to play the [Video].
    fn url(&self) -> String;

    /// The title of the [Video].
    fn title(&self) -> String;

    /// The time of video upload.
    fn uploaded(&self) -> chrono::NaiveDateTime;

    /// The subscription uploading the [Video].
    fn subscription(&self) -> Self::Subscription;

    /// The url of the [Videos](Video) thumbnail.
    fn thumbnail_url(&self) -> String;

    /// Save the thumbnail of the [Video] into a file at the given path.
    ///
    /// The image should be fetched using the given [reqwest::Client] and it should
    /// be saved with the given width and height.
    ///
    /// When not overwritten it will default to create a transparent picture.
    async fn thumbnail_with_client<P: AsRef<Path> + Send>(
        &self,
        client: &reqwest::Client,
        filename: P,
        width: i32,
        height: i32,
    ) {
        let thumbnail_url = self.thumbnail_url();
        log::debug!("Getting thumbnail from url {}", thumbnail_url);
        let response = client.get(&thumbnail_url).send().await;

        if response.is_err() {
            log::error!(
                "Failed getting thumbnail for url {}, use default",
                thumbnail_url
            );
            self.default_thumbnail(filename, width, height);
            return;
        }

        let parsed = response.unwrap().bytes().await;

        if parsed.is_err() {
            log::error!(
                "Failed getting thumbnail for url {}, use default",
                thumbnail_url
            );
            self.default_thumbnail(filename, width, height);
            return;
        }

        let parsed_bytes = parsed.unwrap();

        let glib_bytes = glib::Bytes::from(&parsed_bytes.to_vec());

        let stream = gio::MemoryInputStream::from_bytes(&glib_bytes);

        let pixbuf =
            Pixbuf::from_stream_at_scale(&stream, width, height, true, gio::NONE_CANCELLABLE);
        if let Ok(pixbuf) = pixbuf {
            let _ = pixbuf.savev(filename, "png", &[]);
        } else {
            self.default_thumbnail(filename, width, height);
        }
    }

    /// Save the default image similar to [Video::thumbnail_with_client].
    fn default_thumbnail<P: AsRef<Path>>(&self, filename: P, width: i32, height: i32) {
        let pixbuf =
            Pixbuf::new(Colorspace::Rgb, true, 8, width, height).expect("Could not create empty");
        pixbuf.fill(0);
        let _ = pixbuf.savev(filename, "png", &[]);
    }

    /// Save the thumbnail of the [Video] into a file at the given path.
    ///
    /// The image will be fetched using [reqwest::Client::new]. It will
    /// be saved with the given width and height.
    ///
    /// When not overwritten it will default to create a transparent picture.
    async fn thumbnail<P: AsRef<Path> + Send>(&self, filename: P, width: i32, height: i32) {
        self.thumbnail_with_client(&reqwest::Client::new(), filename, width, height)
            .await
    }
}

#[cfg(test)]
mock! {
    pub(crate) Video {}

    impl Clone for Video {
        fn clone(&self) -> Self;
    }

    impl Video for Video {
        type Subscription = MockSubscription;

        fn url(&self) -> String;
        fn title(&self) -> String;
        fn uploaded(&self) -> chrono::NaiveDateTime;
        fn subscription(&self) -> MockSubscription;
        fn thumbnail_url(&self) -> String;
    }
}

#[cfg(test)]
impl std::hash::Hash for MockVideo {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.uploaded().hash(state);
    }
}

#[cfg(test)]
impl PartialEq for MockVideo {
    fn eq(&self, other: &Self) -> bool {
        self.uploaded().eq(&other.uploaded())
    }
}
#[cfg(test)]
impl Eq for MockVideo {}
