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

use crate::Generator;
use crate::{Subscription, SubscriptionList, Video};

use std::sync::Arc;
use std::sync::Mutex;

use async_trait::async_trait;

#[derive(Clone)]
pub(crate) struct Merger<S, V> {
    subscription_list: Arc<Mutex<SubscriptionList<S>>>,
    _phantom: std::marker::PhantomData<V>,
}

impl<S, V> Merger<S, V>
where
    S: Subscription<Video = V>,
    V: Video<Subscription = S>,
{
    pub fn new(subscriptions: Arc<Mutex<SubscriptionList<S>>>) -> Self {
        Merger {
            subscription_list: subscriptions,
            _phantom: std::marker::PhantomData,
        }
    }
}

#[async_trait]
impl<S, V> Generator for Merger<S, V>
where
    S: 'static + Subscription<Video = V>,
    V: Video<Subscription = S>,
    <S as Subscription>::Iterator: 'static + std::marker::Send,
{
    type Item = V;

    type Iterator = std::vec::IntoIter<V>;

    async fn generate(&self) -> (Self::Iterator, Option<crate::Error>) {
        // TODO: Error Handling
        let subscriptions = self.subscription_list.lock().unwrap().subscriptions();
        log::debug!("Starting getting subscriptions");
        let client = reqwest::Client::builder()
            .tcp_keepalive(Some(std::time::Duration::from_secs(10)))
            .build()
            .unwrap();
        let results = futures::future::join_all(
            subscriptions
                .iter()
                .map(|s| s.generate_with_client(&client)),
        )
        .await;
        log::debug!("Finished getting subscriptions");

        // TODO: More efficient (e.g. with Heap)
        let mut videos = results
            .into_iter()
            .map(|res| res.0.collect::<Vec<_>>())
            .collect::<Vec<_>>()
            .concat();
        videos.sort_unstable_by_key(|v| v.uploaded());
        videos.reverse();
        (videos.into_iter(), None)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use crate::mock::MockSubscription;
    use crate::mock::MockVideo;

    use chrono::NaiveDate;
    use chrono::NaiveDateTime;

    #[tokio::test]
    async fn merger_no_subscription() {
        let subscriptions: Arc<Mutex<SubscriptionList<MockSubscription>>> =
            Arc::new(Mutex::new(SubscriptionList::new()));
        let merger: Merger<MockSubscription, MockVideo> = Merger::new(subscriptions);

        let mut result = merger.generate().await;

        assert!(result.0.next().is_none());
        assert!(result.1.is_none());
    }

    fn make_subscription(dates: Vec<NaiveDateTime>) -> MockSubscription {
        let dates_clone = dates.clone();

        let mut subscription1 = MockSubscription::new();
        subscription1.expect_generate().returning(move || {
            (
                dates_clone
                    .clone()
                    .into_iter()
                    .map(|d| make_video(d))
                    .collect::<Vec<_>>()
                    .into_iter(),
                None,
            )
        });
        subscription1
            .expect_clone()
            .returning(move || make_subscription(dates.clone()));

        subscription1
    }

    fn make_video(datetime: NaiveDateTime) -> MockVideo {
        let datetime_clone = datetime.clone();
        let mut video = MockVideo::new();
        video.expect_uploaded().returning(move || datetime_clone);
        video.expect_clone().returning(move || make_video(datetime));
        video
    }

    #[tokio::test]
    async fn merger_one_subscription() {
        let subscriptions: Arc<Mutex<SubscriptionList<MockSubscription>>> =
            Arc::new(Mutex::new(SubscriptionList::new()));
        let merger: Merger<MockSubscription, MockVideo> = Merger::new(subscriptions.clone());

        let date_video1 = NaiveDate::from_ymd(2021, 8, 12).and_hms(0, 0, 0);
        let date_video2 = NaiveDate::from_ymd(2021, 8, 11).and_hms(0, 0, 0);

        subscriptions
            .lock()
            .unwrap()
            .add(make_subscription(vec![date_video1, date_video2]));

        let mut result = merger.generate().await;

        assert_eq!(result.0.next().unwrap().uploaded(), date_video1);
        assert_eq!(result.0.next().unwrap().uploaded(), date_video2);
        assert!(result.0.next().is_none());
        assert!(result.1.is_none());
    }

    #[tokio::test]
    async fn merger_two_subscription() {
        let subscriptions: Arc<Mutex<SubscriptionList<MockSubscription>>> =
            Arc::new(Mutex::new(SubscriptionList::new()));
        let merger: Merger<MockSubscription, MockVideo> = Merger::new(subscriptions.clone());

        let date_video1 = NaiveDate::from_ymd(2021, 8, 12).and_hms(0, 0, 0);
        let date_video2 = NaiveDate::from_ymd(2021, 8, 11).and_hms(0, 0, 0);

        let date_video3 = NaiveDate::from_ymd(2021, 8, 10).and_hms(0, 0, 0);
        let date_video4 = NaiveDate::from_ymd(2021, 8, 9).and_hms(0, 0, 0);

        subscriptions
            .lock()
            .unwrap()
            .add(make_subscription(vec![date_video1, date_video3]));

        subscriptions
            .lock()
            .unwrap()
            .add(make_subscription(vec![date_video2, date_video4]));

        let mut result = merger.generate().await;

        assert_eq!(result.0.next().unwrap().uploaded(), date_video1);
        assert_eq!(result.0.next().unwrap().uploaded(), date_video2);
        assert_eq!(result.0.next().unwrap().uploaded(), date_video3);
        assert_eq!(result.0.next().unwrap().uploaded(), date_video4);
        assert!(result.0.next().is_none());
        assert!(result.1.is_none());
    }
}
