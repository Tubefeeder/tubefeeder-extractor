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

use std::{
    path::Path,
    sync::{Arc, Mutex},
};

use tf_core::{ExpandedVideo, Observable, Video};

use crate::{AnySubscription, Platform};

/// A [Video] coming from any [Platform].
#[derive(Clone)]
pub enum AnyVideo {
    #[cfg(feature = "youtube")]
    Youtube(Arc<Mutex<ExpandedVideo<tf_yt::YTVideo>>>),
    #[cfg(test)]
    Test(Arc<Mutex<ExpandedVideo<tf_test::TestVideo>>>),
}

impl AnyVideo {
    /// The url of the [AnyVideo].
    pub fn url(&self) -> String {
        match self {
            #[cfg(feature = "youtube")]
            AnyVideo::Youtube(yt) => yt.lock().unwrap().url(),
            #[cfg(test)]
            AnyVideo::Test(test) => test.lock().unwrap().url(),
        }
    }

    /// The title of the [AnyVideo].
    pub fn title(&self) -> String {
        match self {
            #[cfg(feature = "youtube")]
            AnyVideo::Youtube(yt) => yt.lock().unwrap().title(),
            #[cfg(test)]
            AnyVideo::Test(test) => test.lock().unwrap().title(),
        }
    }

    /// The date of upload of the [AnyVideo].
    pub fn uploaded(&self) -> chrono::NaiveDateTime {
        match self {
            #[cfg(feature = "youtube")]
            AnyVideo::Youtube(yt) => yt.lock().unwrap().uploaded(),
            #[cfg(test)]
            AnyVideo::Test(test) => test.lock().unwrap().uploaded(),
        }
    }

    /// The [AnySubscription] of the [AnyVideo].
    ///
    /// The [Platform] of the [AnyVideo] and [AnySubscription] will always match.
    pub fn subscription(&self) -> AnySubscription {
        match self {
            #[cfg(feature = "youtube")]
            AnyVideo::Youtube(yt) => yt.lock().unwrap().subscription().into(),
            #[cfg(test)]
            AnyVideo::Test(test) => test.lock().unwrap().subscription().into(),
        }
    }

    /// Save the thumbnail of the [AnyVideo] into a file at the given path.
    ///
    /// The image should be fetched using the given [reqwest::Client] and it should
    /// be saved with the given width and height.
    ///
    /// When not overwritten it will default to the default thumbnails ot the [Platform].
    pub async fn thumbnail_with_client<P: AsRef<Path> + Send>(
        &self,
        client: &reqwest::Client,
        filename: P,
        width: i32,
        height: i32,
    ) {
        match self {
            #[cfg(feature = "youtube")]
            AnyVideo::Youtube(yt) => {
                yt.lock()
                    .unwrap()
                    .thumbnail_with_client(client, filename, width, height)
                    .await
            }
            #[cfg(test)]
            AnyVideo::Test(test) => {
                test.lock()
                    .unwrap()
                    .thumbnail_with_client(client, filename, width, height)
                    .await
            }
        }
    }

    /// Save the thumbnail of the [AnyVideo] into a file at the given path.
    ///
    /// The image will be fetched using the default [reqwest::Client::new] and it will
    /// be saved with the given width and height.
    pub async fn thumbnail<P: AsRef<Path> + Send>(&self, filename: P, width: i32, height: i32) {
        self.thumbnail_with_client(&reqwest::Client::new(), filename, width, height)
            .await
    }

    /// Set the playing status of the [AnyVideo] to playing.
    pub fn play(&self) {
        match self {
            #[cfg(feature = "youtube")]
            AnyVideo::Youtube(yt) => yt.lock().unwrap().play(),
            #[cfg(test)]
            AnyVideo::Test(test) => test.lock().unwrap().play(),
        }
    }

    /// Set the playing status of the [AnyVideo] to stopped.
    pub fn stop(&self) {
        match self {
            #[cfg(feature = "youtube")]
            AnyVideo::Youtube(yt) => yt.lock().unwrap().stop(),
            #[cfg(test)]
            AnyVideo::Test(test) => test.lock().unwrap().stop(),
        }
    }

    /// Gets the playing status of the [AnyVideo].
    pub fn playing(&self) -> bool {
        match self {
            #[cfg(feature = "youtube")]
            AnyVideo::Youtube(yt) => yt.lock().unwrap().playing(),
            #[cfg(test)]
            AnyVideo::Test(test) => test.lock().unwrap().playing(),
        }
    }

    /// Get the [Platform] where the [AnyVideo] was uploaded.
    pub fn platform(&self) -> Platform {
        match self {
            #[cfg(feature = "youtube")]
            AnyVideo::Youtube(_yt) => Platform::Youtube,
            #[cfg(test)]
            AnyVideo::Test(_test) => Platform::Test,
        }
    }
}

impl Observable<tf_core::VideoEvent> for AnyVideo {
    fn attach(
        &mut self,
        observer: std::sync::Weak<Mutex<Box<dyn tf_core::Observer<tf_core::VideoEvent> + Send>>>,
    ) {
        match self {
            #[cfg(feature = "youtube")]
            AnyVideo::Youtube(yt) => yt.lock().unwrap().attach(observer),
            #[cfg(test)]
            AnyVideo::Test(test) => test.lock().unwrap().attach(observer),
        }
    }

    fn detach(
        &mut self,
        observer: std::sync::Weak<Mutex<Box<dyn tf_core::Observer<tf_core::VideoEvent> + Send>>>,
    ) {
        match self {
            #[cfg(feature = "youtube")]
            AnyVideo::Youtube(yt) => yt.lock().unwrap().detach(observer),
            #[cfg(test)]
            AnyVideo::Test(test) => test.lock().unwrap().detach(observer),
        }
    }
}

#[cfg(feature = "youtube")]
impl From<Arc<Mutex<ExpandedVideo<tf_yt::YTVideo>>>> for AnyVideo {
    fn from(v: Arc<Mutex<ExpandedVideo<tf_yt::YTVideo>>>) -> Self {
        AnyVideo::Youtube(v)
    }
}

#[cfg(test)]
impl From<Arc<Mutex<ExpandedVideo<tf_test::TestVideo>>>> for AnyVideo {
    fn from(v: Arc<Mutex<ExpandedVideo<tf_test::TestVideo>>>) -> Self {
        AnyVideo::Test(v)
    }
}
