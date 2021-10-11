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
    convert::TryFrom,
    str::FromStr,
    sync::{Arc, Mutex},
};

use async_trait::async_trait;

use tf_core::{ExpandedVideo, Video};
use tf_observer::{Observable, Observer};

use crate::{AnySubscription, Platform};

macro_rules! match_video {
    ($video: ident, $func_name: ident) => {
        match_video!($video, $func_name())
    };
    ($video: ident, $($func_name: ident ($($arg: ident),*)).*) => {
        match $video {
            #[cfg(feature = "youtube")]
            AnyVideo::Youtube(v) => v.lock().unwrap().$($func_name($($arg)*))*,
            #[cfg(feature = "peertube")]
            AnyVideo::Peertube(v) => v.lock().unwrap().$($func_name($($arg)*))*,
            #[cfg(feature = "lbry")]
            AnyVideo::Lbry(v) => v.lock().unwrap().$($func_name($($arg)*))*,
            // -- Add new value here.
            #[cfg(test)]
            AnyVideo::Test(v) => v.lock().unwrap().$($func_name($($arg)*))*,
        }
    };
}

fn arc_eq<S: Eq>(a1: &Arc<Mutex<S>>, a2: &Arc<Mutex<S>>) -> bool {
    if Arc::ptr_eq(a1, a2) {
        true
    } else {
        a1.lock().unwrap().eq(&a2.lock().unwrap())
    }
}

/// A [Video] coming from any [Platform].
#[derive(Clone)]
pub enum AnyVideo {
    #[cfg(feature = "youtube")]
    Youtube(Arc<Mutex<ExpandedVideo<tf_yt::YTVideo>>>),
    #[cfg(feature = "peertube")]
    Peertube(Arc<Mutex<ExpandedVideo<tf_pt::PTVideo>>>),
    #[cfg(feature = "lbry")]
    Lbry(Arc<Mutex<ExpandedVideo<tf_lbry::LbryVideo>>>),
    // -- Add new value here.
    #[cfg(test)]
    Test(Arc<Mutex<ExpandedVideo<tf_test::TestVideo>>>),
}

impl std::hash::Hash for AnyVideo {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match_video!(self, hash(state))
    }
}

impl std::cmp::PartialEq for AnyVideo {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            #[cfg(feature = "youtube")]
            (AnyVideo::Youtube(v1), AnyVideo::Youtube(v2)) => arc_eq(v1, v2),
            #[cfg(feature = "peertube")]
            (AnyVideo::Peertube(v1), AnyVideo::Peertube(v2)) => arc_eq(v1, v2),
            #[cfg(feature = "lbry")]
            (AnyVideo::Lbry(v1), AnyVideo::Lbry(v2)) => arc_eq(v1, v2),
            // -- Add new value here.
            #[cfg(test)]
            (AnyVideo::Test(v1), AnyVideo::Test(v2)) => arc_eq(v1, v2),
            #[allow(unreachable_patterns)]
            _ => false,
        }
    }
}

impl std::cmp::Eq for AnyVideo {}

#[async_trait]
impl Video for AnyVideo {
    type Subscription = AnySubscription;

    fn url(&self) -> String {
        match_video!(self, url)
    }

    fn title(&self) -> String {
        match_video!(self, title)
    }

    fn uploaded(&self) -> chrono::NaiveDateTime {
        match_video!(self, uploaded)
    }

    fn subscription(&self) -> AnySubscription {
        match self {
            #[cfg(feature = "youtube")]
            AnyVideo::Youtube(yt) => yt.lock().unwrap().subscription().into(),
            #[cfg(feature = "peertube")]
            AnyVideo::Peertube(pt) => pt.lock().unwrap().subscription().into(),
            #[cfg(feature = "lbry")]
            AnyVideo::Lbry(lbry) => lbry.lock().unwrap().subscription().into(),
            // -- Add new value here.
            #[cfg(test)]
            AnyVideo::Test(test) => test.lock().unwrap().subscription().into(),
        }
    }

    fn thumbnail_url(&self) -> String {
        match_video!(self, thumbnail_url)
    }

    async fn thumbnail_with_client(&self, client: &reqwest::Client) -> image::DynamicImage {
        match self {
            #[cfg(feature = "youtube")]
            AnyVideo::Youtube(yt) => {
                let v = yt.lock().unwrap().clone();
                v.thumbnail_with_client(client).await
            }
            #[cfg(feature = "peertube")]
            AnyVideo::Peertube(pt) => {
                let v = pt.lock().unwrap().clone();
                v.thumbnail_with_client(client).await
            }
            #[cfg(feature = "lbry")]
            AnyVideo::Lbry(lbry) => {
                let v = lbry.lock().unwrap().clone();
                v.thumbnail_with_client(client).await
            }
            // -- Add new value here
            #[cfg(test)]
            AnyVideo::Test(test) => {
                let v = test.lock().unwrap().clone();
                v.thumbnail_with_client(client).await
            }
        }
    }
}

impl AnyVideo {
    /// Set the playing status of the [AnyVideo] to playing.
    pub fn play(&self) {
        match_video!(self, play)
    }

    /// Set the playing status of the [AnyVideo] to stopped.
    pub fn stop(&self) {
        match_video!(self, stop)
    }

    /// Gets the playing status of the [AnyVideo].
    pub fn playing(&self) -> bool {
        match_video!(self, playing)
    }

    /// Get the [Platform] where the [AnyVideo] was uploaded.
    pub fn platform(&self) -> Platform {
        match self {
            #[cfg(feature = "youtube")]
            AnyVideo::Youtube(_v) => Platform::Youtube,
            #[cfg(feature = "peertube")]
            AnyVideo::Peertube(_v) => Platform::Peertube,
            #[cfg(feature = "lbry")]
            AnyVideo::Lbry(_v) => Platform::Lbry,
            // -- Add new value here
            #[cfg(test)]
            AnyVideo::Test(_v) => Platform::Test,
        }
    }
}

impl Observable<tf_core::VideoEvent> for AnyVideo {
    fn attach(
        &mut self,
        observer: std::sync::Weak<Mutex<Box<dyn Observer<tf_core::VideoEvent> + Send>>>,
    ) {
        match_video!(self, attach(observer));
    }

    fn detach(
        &mut self,
        observer: std::sync::Weak<Mutex<Box<dyn Observer<tf_core::VideoEvent> + Send>>>,
    ) {
        match_video!(self, detach(observer));
    }
}

impl TryFrom<Vec<String>> for AnyVideo {
    // TODO: Error handling
    type Error = ();

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        if value.len() == 0 {
            return Err(());
        }

        let mut value_mut = value.clone();

        let platform = Platform::from_str(value_mut.remove(0).as_str());
        match platform {
            #[cfg(feature = "youtube")]
            Ok(Platform::Youtube) => tf_yt::YTVideo::try_from(value_mut)
                .map(|v| Arc::new(Mutex::new(ExpandedVideo::from(v))).into()),
            #[cfg(feature = "peertube")]
            Ok(Platform::Peertube) => tf_pt::PTVideo::try_from(value_mut)
                .map(|v| Arc::new(Mutex::new(ExpandedVideo::from(v))).into()),
            #[cfg(feature = "lbry")]
            Ok(Platform::Lbry) => tf_lbry::LbryVideo::try_from(value_mut)
                .map(|v| Arc::new(Mutex::new(ExpandedVideo::from(v))).into()),
            // -- Add value here
            #[cfg(test)]
            Ok(Platform::Test) => tf_test::TestVideo::try_from(value_mut)
                .map(|v| Arc::new(Mutex::new(ExpandedVideo::from(v))).into()),
            _ => Err(()),
        }
    }
}

impl From<AnyVideo> for Vec<String> {
    fn from(video: AnyVideo) -> Self {
        let mut result = vec![video.platform().into()];
        let vid_vec: Vec<String> = match video {
            #[cfg(feature = "youtube")]
            AnyVideo::Youtube(v) => v.lock().unwrap().clone().into(),
            #[cfg(feature = "peertube")]
            AnyVideo::Peertube(v) => v.lock().unwrap().clone().into(),
            #[cfg(feature = "lbry")]
            AnyVideo::Lbry(v) => v.lock().unwrap().clone().into(),
            // -- Add new value here.
            #[cfg(test)]
            AnyVideo::Test(v) => v.lock().unwrap().clone().into(),
        };
        result.append(&mut vid_vec.clone());
        result
    }
}

#[cfg(feature = "youtube")]
impl From<Arc<Mutex<ExpandedVideo<tf_yt::YTVideo>>>> for AnyVideo {
    fn from(v: Arc<Mutex<ExpandedVideo<tf_yt::YTVideo>>>) -> Self {
        AnyVideo::Youtube(v)
    }
}

#[cfg(feature = "peertube")]
impl From<Arc<Mutex<ExpandedVideo<tf_pt::PTVideo>>>> for AnyVideo {
    fn from(v: Arc<Mutex<ExpandedVideo<tf_pt::PTVideo>>>) -> Self {
        AnyVideo::Peertube(v)
    }
}

#[cfg(feature = "lbry")]
impl From<Arc<Mutex<ExpandedVideo<tf_lbry::LbryVideo>>>> for AnyVideo {
    fn from(v: Arc<Mutex<ExpandedVideo<tf_lbry::LbryVideo>>>) -> Self {
        AnyVideo::Lbry(v)
    }
}

// -- Add conversion here.

#[cfg(test)]
impl From<Arc<Mutex<ExpandedVideo<tf_test::TestVideo>>>> for AnyVideo {
    fn from(v: Arc<Mutex<ExpandedVideo<tf_test::TestVideo>>>) -> Self {
        AnyVideo::Test(v)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::convert::TryInto;
    use tf_core::Subscription;
    use tf_test::{TestSubscription, TestVideo};

    #[test]
    fn anyvideo_eq() {
        let s1 = TestSubscription::new("Channel1");
        let v1: AnyVideo = Arc::new(Mutex::new(ExpandedVideo::from(TestVideo::new(
            "Video1",
            s1.clone(),
        ))))
        .into();
        let v2: AnyVideo = Arc::new(Mutex::new(ExpandedVideo::from(TestVideo::new(
            "Video1",
            s1.clone(),
        ))))
        .into();
        let v3: AnyVideo = Arc::new(Mutex::new(ExpandedVideo::from(TestVideo::new(
            "Video2",
            s1.clone(),
        ))))
        .into();

        assert!(v1 == v1);
        assert!(v1 == v2);
        assert!(v1 != v3);
    }

    #[test]
    fn anyvideo_conversion_test() {
        let row = vec!["test".to_string(), "Video".to_string(), "Sub".to_string()];
        let video_res: Result<AnyVideo, ()> = row.try_into();

        assert!(video_res.is_ok());

        let video = video_res.unwrap();

        assert_eq!(video.title(), "Video");
        assert_eq!(video.subscription().name(), Some("Sub".to_string()));
    }

    #[test]
    fn anyvideo_conversion_test_back() {
        let row = vec!["test".to_string(), "Video".to_string(), "Sub".to_string()];
        let video: AnyVideo = Arc::new(Mutex::new(ExpandedVideo::from(TestVideo::new(
            "Video".to_string(),
            TestSubscription::new("Sub"),
        ))))
        .into();

        assert_eq!(Vec::<String>::from(video), row);
    }
}
