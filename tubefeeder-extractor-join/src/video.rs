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

use std::sync::{Arc, Mutex};

use tf_core::{ExpandedVideo, Video};

use crate::AnySubscription;

#[derive(Clone)]
pub enum AnyVideo {
    #[cfg(feature = "youtube")]
    Youtube(Arc<Mutex<ExpandedVideo<tf_yt::YTVideo>>>),
    #[cfg(feature = "testPlatform")]
    Test(Arc<Mutex<ExpandedVideo<tf_test::TestVideo>>>),
}

impl AnyVideo {
    pub fn url(&self) -> String {
        match self {
            #[cfg(feature = "youtube")]
            AnyVideo::Youtube(yt) => yt.lock().unwrap().url(),
            #[cfg(feature = "testPlatform")]
            AnyVideo::Test(test) => test.lock().unwrap().url(),
        }
    }

    pub fn title(&self) -> String {
        match self {
            #[cfg(feature = "youtube")]
            AnyVideo::Youtube(yt) => yt.lock().unwrap().title(),
            #[cfg(feature = "testPlatform")]
            AnyVideo::Test(test) => test.lock().unwrap().title(),
        }
    }

    pub fn uploaded(&self) -> chrono::NaiveDateTime {
        match self {
            #[cfg(feature = "youtube")]
            AnyVideo::Youtube(yt) => yt.lock().unwrap().uploaded(),
            #[cfg(feature = "testPlatform")]
            AnyVideo::Test(test) => test.lock().unwrap().uploaded(),
        }
    }

    pub fn subscription(&self) -> AnySubscription {
        match self {
            #[cfg(feature = "youtube")]
            AnyVideo::Youtube(yt) => yt.lock().unwrap().subscription().into(),
            #[cfg(feature = "testPlatform")]
            AnyVideo::Test(test) => test.lock().unwrap().subscription().into(),
        }
    }
}

#[cfg(feature = "youtube")]
impl From<Arc<Mutex<ExpandedVideo<tf_yt::YTVideo>>>> for AnyVideo {
    fn from(v: Arc<Mutex<ExpandedVideo<tf_yt::YTVideo>>>) -> Self {
        AnyVideo::Youtube(v)
    }
}

#[cfg(feature = "testPlatform")]
impl From<Arc<Mutex<ExpandedVideo<tf_test::TestVideo>>>> for AnyVideo {
    fn from(v: Arc<Mutex<ExpandedVideo<tf_test::TestVideo>>>) -> Self {
        AnyVideo::Test(v)
    }
}
