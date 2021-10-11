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

use crate::Subscription;

use async_trait::async_trait;
use image::DynamicImage;

#[cfg(test)]
use {crate::mock::MockSubscription, mockall::predicate::*, mockall::*};

/// A [`Video`] that can come from any website.
/// A video needs to be:
///
/// - [std::clone::Clone]: Should just be derived.
/// - [std::marker::Sync]: Already implemented by default if you do not make anything weird.
/// - [std::marker::Send]: Similar to Sync probably also implemented by default.
/// - [std::cmp::Eq]: Do only compare identifying information, e.g. id but not name if that can be reused.
/// - [std::hash::Hash]: Hash only identifying information, similar to [std::cmp::PartialEq].
/// - [std::convert::Into<Vec<String>>]: Serialize into a vec of strings. Do only serialize identifying information.
/// - [std::convert::TryFrom<Vec<String>>]: Deserialize information similar to serialization.
#[async_trait]
pub trait Video:
    Clone
    + std::hash::Hash
    + std::cmp::Eq
    + std::marker::Send
    + std::marker::Sync
    + Into<Vec<String>>
    + std::convert::TryFrom<Vec<String>>
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

    /// Get the thumbnail of the [Video].
    ///
    /// The image should be fetched using the given [reqwest::Client].
    ///
    /// When not overwritten it will fetch the thumbnail from [Video::thumbnail_url] and guess the format.
    async fn thumbnail_with_client(&self, client: &reqwest::Client) -> image::DynamicImage {
        let thumbnail_url = self.thumbnail_url();
        log::debug!("Getting thumbnail from url {}", thumbnail_url);
        let response = client.get(&thumbnail_url).send().await;

        if response.is_err() {
            log::error!(
                "Failed getting thumbnail for url {}, use default",
                thumbnail_url
            );
            return self.default_thumbnail();
        }

        let parsed = response.unwrap().bytes().await;

        if parsed.is_err() {
            log::error!(
                "Failed getting thumbnail for url {}, use default",
                thumbnail_url
            );
            return self.default_thumbnail();
        }

        let parsed_bytes = parsed.unwrap();

        if let Some(image) = <Self as Video>::convert_image(&parsed_bytes) {
            image
        } else {
            self.default_thumbnail()
        }
    }

    /// Try to convert image bytes into a usable [DynamicImage].
    ///
    /// By default the format will be guessed.
    fn convert_image(data: &[u8]) -> Option<DynamicImage> {
        image::load_from_memory(&data).ok()
    }

    /// Get the default thumbnail, if not overwritten a transparent 1 by 1 pixel image.
    fn default_thumbnail(&self) -> DynamicImage {
        DynamicImage::new_rgba8(1, 1)
    }

    /// Get the thumbnail of the [Video].
    ///
    /// The image will be fetched using [reqwest::Client::new].
    ///
    /// When not overwritten it will default to create a transparent picture.
    async fn thumbnail(&self) -> DynamicImage {
        self.thumbnail_with_client(&reqwest::Client::new()).await
    }
}

#[cfg(test)]
mock! {
    pub(crate) Video {}

    impl Clone for Video {
        fn clone(&self) -> Self;
    }

    impl std::convert::TryFrom<Vec<String>> for Video {
        type Error = ();
        fn try_from(_vec: Vec<String>) -> Result<Self, ()> {
            Err(())
        }
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

#[cfg(test)]
impl std::convert::From<MockVideo> for Vec<String> {
    fn from(_v: MockVideo) -> Self {
        vec![]
    }
}
