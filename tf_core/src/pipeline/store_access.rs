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

use crate::{ErrorStore, Generator, Video, VideoStore};

use async_trait::async_trait;

/// A pipeline elemnt putting generated [Video]s `V` from the [Generator] `G` into
/// the [VideoStore] and generating `Arc<Mutex<V>>`.
#[derive(Clone)]
pub struct StoreAccess<V, G> {
    /// The [VideoStore] beeing accessed
    store: Arc<Mutex<VideoStore<V>>>,
    /// The internal [Generator] generating the [Video]s put into the [VideoStore].
    generator: G,
}

impl<V, G> StoreAccess<V, G>
where
    V: 'static + Video,
    G: Generator<Item = V>,
    <G as Generator>::Iterator: 'static,
{
    /// Create a new [StoreAccess] using the given [VideoStore] and [Generator].
    pub fn new(store: Arc<Mutex<VideoStore<V>>>, generator: G) -> Self {
        StoreAccess { store, generator }
    }
}

#[async_trait]
impl<V, G> Generator for StoreAccess<V, G>
where
    V: 'static + Video + std::hash::Hash + std::cmp::Eq + std::marker::Sync + std::marker::Send,
    G: Generator<Item = V> + std::marker::Send + std::marker::Sync + 'static,
    <G as Generator>::Iterator: 'static + std::marker::Send,
{
    type Item = Arc<Mutex<V>>;
    // Better when https://github.com/rust-lang/rust/issues/63063 is stable.
    // type Iterator = impl Iterator<Item = <Self as Generator>::Item>;
    type Iterator = Box<dyn Iterator<Item = <Self as Generator>::Item> + std::marker::Send>;

    async fn generate(&self, errors: &ErrorStore) -> Self::Iterator {
        let store = self.store.clone();
        let gen_iter = self.generator.generate(errors).await;
        let map = gen_iter.map(move |v| store.lock().unwrap().get(&v));
        Box::new(map) as <Self as Generator>::Iterator
    }
}

#[cfg(test)]
mod test {
    use chrono::{NaiveDate, NaiveDateTime};

    use super::*;
    use crate::mock::MockGenerator;
    use crate::mock::MockVideo;

    fn make_video(datetime: NaiveDateTime) -> MockVideo {
        let datetime_clone = datetime.clone();
        let mut video = MockVideo::new();
        video.expect_uploaded().returning(move || datetime_clone);
        video.expect_clone().returning(move || make_video(datetime));
        video
    }

    #[tokio::test]
    async fn store_access_no_duplicates() {
        let mut generator = MockGenerator::new();
        generator.expect_generate().returning(|_| {
            vec![
                make_video(
                    NaiveDate::from_ymd_opt(2021, 8, 12)
                        .unwrap()
                        .and_hms_opt(0, 0, 0)
                        .unwrap(),
                ),
                make_video(
                    NaiveDate::from_ymd_opt(2021, 8, 11)
                        .unwrap()
                        .and_hms_opt(0, 0, 0)
                        .unwrap(),
                ),
            ]
            .into_iter()
        });

        let store = StoreAccess::new(Arc::new(Mutex::new(VideoStore::new())), generator);

        let errors = ErrorStore::new();
        let mut result = store.generate(&errors).await;

        let arc1 = result.next();
        let arc2 = result.next();

        assert!(arc1.is_some());
        assert!(arc2.is_some());
        assert!(!Arc::ptr_eq(&arc1.unwrap(), &arc2.unwrap()))
    }

    #[tokio::test]
    async fn store_access_one_generate_duplicates() {
        let mut generator = MockGenerator::new();
        generator.expect_generate().returning(|_| {
            vec![
                make_video(
                    NaiveDate::from_ymd_opt(2021, 8, 12)
                        .unwrap()
                        .and_hms_opt(0, 0, 0)
                        .unwrap(),
                ),
                make_video(
                    NaiveDate::from_ymd_opt(2021, 8, 12)
                        .unwrap()
                        .and_hms_opt(0, 0, 0)
                        .unwrap(),
                ),
            ]
            .into_iter()
        });

        let store = StoreAccess::new(Arc::new(Mutex::new(VideoStore::new())), generator);

        let errors = ErrorStore::new();
        let mut result = store.generate(&errors).await;

        let arc1 = result.next();
        let arc2 = result.next();

        assert!(arc1.is_some());
        assert!(arc2.is_some());
        assert!(Arc::ptr_eq(&arc1.unwrap(), &arc2.unwrap()));
    }

    #[tokio::test]
    async fn store_access_two_generate_duplicates() {
        let mut generator = MockGenerator::new();
        generator.expect_generate().times(1).returning(|_| {
            vec![
                make_video(
                    NaiveDate::from_ymd_opt(2021, 8, 12)
                        .unwrap()
                        .and_hms_opt(0, 0, 0)
                        .unwrap(),
                ),
                make_video(
                    NaiveDate::from_ymd_opt(2021, 8, 11)
                        .unwrap()
                        .and_hms_opt(0, 0, 0)
                        .unwrap(),
                ),
            ]
            .into_iter()
        });
        generator.expect_generate().times(1).returning(|_| {
            vec![
                make_video(
                    NaiveDate::from_ymd_opt(2021, 8, 12)
                        .unwrap()
                        .and_hms_opt(0, 0, 0)
                        .unwrap(),
                ),
                make_video(
                    NaiveDate::from_ymd_opt(2021, 8, 11)
                        .unwrap()
                        .and_hms_opt(0, 0, 0)
                        .unwrap(),
                ),
            ]
            .into_iter()
        });

        let store = StoreAccess::new(Arc::new(Mutex::new(VideoStore::new())), generator);

        let errors = ErrorStore::new();
        let mut result_1 = store.generate(&errors).await;

        let arc1_1 = result_1.next();
        let arc1_2 = result_1.next();

        let mut result_2 = store.generate(&errors).await;

        let arc2_1 = result_2.next();
        let arc2_2 = result_2.next();

        assert!(Arc::ptr_eq(&arc1_1.unwrap(), &arc2_1.unwrap()));
        assert!(Arc::ptr_eq(&arc1_2.unwrap(), &arc2_2.unwrap()));
    }
}
