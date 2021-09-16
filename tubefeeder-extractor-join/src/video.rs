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
    path::Path,
    str::FromStr,
    sync::{Arc, Mutex},
};

use async_trait::async_trait;

use tf_core::{ExpandedVideo, Subscription, Video};
use tf_observer::{Observable, Observer};

use crate::{AnySubscription, Platform};

const DATE_FORMAT: &'static str = "%Y-%m-%dT%H:%M:%S";

/// A [Video] coming from any [Platform].
#[derive(Clone)]
pub enum AnyVideo {
    #[cfg(feature = "youtube")]
    Youtube(Arc<Mutex<ExpandedVideo<tf_yt::YTVideo>>>),
    #[cfg(test)]
    Test(Arc<Mutex<ExpandedVideo<tf_test::TestVideo>>>),
}

impl std::hash::Hash for AnyVideo {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            #[cfg(feature = "youtube")]
            AnyVideo::Youtube(v) => v.lock().unwrap().hash(state),
            #[cfg(test)]
            AnyVideo::Test(v) => v.lock().unwrap().hash(state),
        }
    }
}

impl std::cmp::PartialEq for AnyVideo {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            #[cfg(feature = "youtube")]
            (AnyVideo::Youtube(v1), AnyVideo::Youtube(v2)) => {
                if Arc::ptr_eq(v1, v2) {
                    true
                } else {
                    v1.lock().unwrap().eq(&v2.lock().unwrap())
                }
            }
            #[cfg(test)]
            (AnyVideo::Test(v1), AnyVideo::Test(v2)) => {
                if Arc::ptr_eq(v1, v2) {
                    true
                } else {
                    v1.lock().unwrap().eq(&v2.lock().unwrap())
                }
            }
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
        match self {
            #[cfg(feature = "youtube")]
            AnyVideo::Youtube(yt) => yt.lock().unwrap().url(),
            #[cfg(test)]
            AnyVideo::Test(test) => test.lock().unwrap().url(),
        }
    }

    fn title(&self) -> String {
        match self {
            #[cfg(feature = "youtube")]
            AnyVideo::Youtube(yt) => yt.lock().unwrap().title(),
            #[cfg(test)]
            AnyVideo::Test(test) => test.lock().unwrap().title(),
        }
    }

    fn uploaded(&self) -> chrono::NaiveDateTime {
        match self {
            #[cfg(feature = "youtube")]
            AnyVideo::Youtube(yt) => yt.lock().unwrap().uploaded(),
            #[cfg(test)]
            AnyVideo::Test(test) => test.lock().unwrap().uploaded(),
        }
    }

    fn subscription(&self) -> AnySubscription {
        match self {
            #[cfg(feature = "youtube")]
            AnyVideo::Youtube(yt) => yt.lock().unwrap().subscription().into(),
            #[cfg(test)]
            AnyVideo::Test(test) => test.lock().unwrap().subscription().into(),
        }
    }

    async fn thumbnail_with_client<P: AsRef<Path> + Send>(
        &self,
        client: &reqwest::Client,
        filename: P,
        width: i32,
        height: i32,
    ) {
        match self {
            #[cfg(feature = "youtube")]
            AnyVideo::Youtube(yt) => {
                let v = yt.lock().unwrap().clone();
                v.thumbnail_with_client(client, filename, width, height)
                    .await
            }
            #[cfg(test)]
            AnyVideo::Test(test) => {
                let v = test.lock().unwrap().clone();
                v.thumbnail_with_client(client, filename, width, height)
                    .await
            }
        }
    }
}

impl AnyVideo {
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
        observer: std::sync::Weak<Mutex<Box<dyn Observer<tf_core::VideoEvent> + Send>>>,
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
        observer: std::sync::Weak<Mutex<Box<dyn Observer<tf_core::VideoEvent> + Send>>>,
    ) {
        match self {
            #[cfg(feature = "youtube")]
            AnyVideo::Youtube(yt) => yt.lock().unwrap().detach(observer),
            #[cfg(test)]
            AnyVideo::Test(test) => test.lock().unwrap().detach(observer),
        }
    }
}

impl TryFrom<&[&str]> for AnyVideo {
    // TODO: Error handling
    type Error = ();

    fn try_from(value: &[&str]) -> Result<Self, Self::Error> {
        let platform = value.get(0).map(|&p| Platform::from_str(p));
        match platform {
            #[cfg(feature = "youtube")]
            Some(Ok(Platform::Youtube)) => {
                let url_opt = value.get(1);
                let title = value.get(2);
                let uploaded = value.get(3);
                let sub_name = value.get(4);
                let sub_id = value.get(5);
                let thumbnail_url = value.get(6);
                match (url_opt, title, uploaded, sub_name, sub_id, thumbnail_url) {
                    (Some(url), Some(tit), Some(upl), Some(sub_n), Some(sub_i), Some(thu)) => {
                        let upl_date = chrono::NaiveDateTime::parse_from_str(upl, DATE_FORMAT);
                        if let Ok(upl) = upl_date {
                            let sub = tf_yt::YTSubscription::new_with_name(sub_i, sub_n);
                            Ok(Arc::new(Mutex::new(ExpandedVideo::from(tf_yt::YTVideo::new(
                                url, tit, upl, sub, thu,
                            ))))
                            .into())
                        } else {
                            Err(())
                        }
                    }
                    _ => Err(()),
                }
            }
            #[cfg(test)]
            Some(Ok(Platform::Test)) => {
                let title = value.get(1);
                let sub_id = value.get(2);
                match (title, sub_id) {
                    (Some(t), Some(s)) => Ok(Arc::new(Mutex::new(ExpandedVideo::from(
                        tf_test::TestVideo::new(t, tf_test::TestSubscription::new(s)),
                    )))
                    .into()),
                    _ => Err(()),
                }
            }
            _ => Err(()),
        }
    }
}

impl From<AnyVideo> for Vec<String> {
    fn from(video: AnyVideo) -> Self {
        let mut result = vec![video.platform().into()];
        match video {
            #[cfg(feature = "youtube")]
            AnyVideo::Youtube(v_arc) => {
                let v = v_arc.lock().unwrap();
                result.push(v.url());
                result.push(v.title());
                result.push(v.uploaded().format(DATE_FORMAT).to_string());
                let sub = v.subscription();
                result.push(sub.name().unwrap_or("".to_string()));
                result.push(sub.id());
                result.push(v.internal().thumbnail_url());
            }
            #[cfg(test)]
            AnyVideo::Test(v_arc) => {
                let v = v_arc.lock().unwrap();
                result.push(v.title());
                result.push(v.subscription().name().unwrap_or("".to_string()));
            }
        }

        return result;
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

#[cfg(test)]
mod test {
    use super::*;
    use std::convert::TryInto;
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
        let row = vec!["test", "Video", "Sub"];
        let video_res: Result<AnyVideo, ()> = row.as_slice().try_into();

        assert!(video_res.is_ok());

        let video = video_res.unwrap();

        assert_eq!(video.title(), "Video");
        assert_eq!(video.subscription().name(), Some("Sub".to_string()));
    }

    #[test]
    fn anyvideo_conversion_test_back() {
        let row = vec!["test", "Video", "Sub"];
        let video: AnyVideo = Arc::new(Mutex::new(ExpandedVideo::from(TestVideo::new(
            "Video".to_string(),
            TestSubscription::new("Sub"),
        ))))
        .into();

        assert_eq!(Vec::<String>::from(video), row);
    }
}
