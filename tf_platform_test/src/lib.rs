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

//! A test implementation of a [Tubefeeder-extractor](https://github.com/Tubefeeder/tubefeeder-extractor) subscription and video.

use std::convert::TryFrom;

use tf_core::{ErrorStore, GeneratorWithClient, Subscription, Video};

use async_trait::async_trait;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TestVideo {
    title: String,
    uploaded: chrono::NaiveDateTime,
    subscription: TestSubscription,
}

impl TryFrom<Vec<String>> for TestVideo {
    type Error = ();

    fn try_from(strings: Vec<String>) -> Result<Self, Self::Error> {
        let title = strings.get(0);
        let sub_id = strings.get(1);
        match (title, sub_id) {
            (Some(t), Some(s)) => Ok(TestVideo::new(t, TestSubscription::new(s))),
            _ => Err(()),
        }
    }
}

impl From<TestVideo> for Vec<String> {
    fn from(video: TestVideo) -> Self {
        vec![video.title, video.subscription.name]
    }
}

#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct TestSubscription {
    name: String,
}

impl Video for TestVideo {
    type Subscription = TestSubscription;

    fn url(&self) -> String {
        format!("https://test.test/{}", self.title)
    }

    fn title(&self) -> String {
        self.title.clone()
    }

    fn uploaded(&self) -> chrono::NaiveDateTime {
        self.uploaded
    }

    fn subscription(&self) -> TestSubscription {
        self.subscription.clone()
    }

    fn thumbnail_url(&self) -> String {
        "".to_string()
    }
}

impl TestVideo {
    pub fn new<S: AsRef<str>>(title: S, subscription: TestSubscription) -> Self {
        Self {
            title: title.as_ref().to_string(),
            subscription,
            uploaded: chrono::NaiveDate::from_ymd_opt(2021, 1, 1)
                .unwrap()
                .and_hms_opt(20, 10, 0)
                .unwrap(),
        }
    }
}

impl std::fmt::Display for TestSubscription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[async_trait]
impl GeneratorWithClient for TestSubscription {
    type Item = TestVideo;
    type Iterator = std::vec::IntoIter<TestVideo>;
    async fn generate_with_client(&self, _e: &ErrorStore, _c: &reqwest::Client) -> Self::Iterator {
        let video1 = TestVideo {
            title: "This is the test video 1".to_owned(),
            uploaded: chrono::NaiveDate::from_ymd_opt(2021, 8, 17)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
            subscription: self.clone(),
        };

        let video2 = TestVideo {
            title: "This is the test video 2".to_owned(),
            uploaded: chrono::NaiveDate::from_ymd_opt(2021, 5, 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
            subscription: self.clone(),
        };

        vec![video1, video2].into_iter()
    }
}

impl Subscription for TestSubscription {
    type Video = TestVideo;

    fn name(&self) -> Option<String> {
        Some(self.name.clone())
    }
}

impl TestSubscription {
    pub fn new(name: &str) -> Self {
        TestSubscription {
            name: name.to_owned(),
        }
    }
}

impl TryFrom<Vec<String>> for TestSubscription {
    type Error = ();

    fn try_from(strings: Vec<String>) -> Result<Self, Self::Error> {
        if let Some(value) = strings.get(0) {
            Ok(TestSubscription::new(value))
        } else {
            Err(())
        }
    }
}

impl From<TestSubscription> for Vec<String> {
    fn from(sub: TestSubscription) -> Self {
        vec![sub.name]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use tf_core::Generator;

    #[tokio::test]
    async fn subscription() {
        let subscription = TestSubscription {
            name: "TestName".to_owned(),
        };

        let errors = ErrorStore::new();
        let iterator = subscription.generate(&errors).await;

        let titles: Vec<String> = iterator.map(|v| v.title()).collect();

        assert_eq!(
            vec![
                "This is the test video 1".to_owned(),
                "This is the test video 2".to_owned()
            ],
            titles
        );
    }
}
