use std::sync::{Arc, Mutex};

use crate::{Generator, Video, VideoStore};

use async_trait::async_trait;

pub(crate) struct StoreAccess<V, G> {
    store: Arc<Mutex<VideoStore<V>>>,
    generator: G,
}

impl<V, G> StoreAccess<V, G>
where
    V: 'static + Video + std::hash::Hash + std::cmp::Eq + std::marker::Sync + std::marker::Send,
    G: Generator<Item = V> + std::marker::Send + std::marker::Sync + 'static,
    <G as Generator>::Iterator: 'static,
{
    pub(crate) fn new(generator: G) -> Self {
        StoreAccess {
            store: Arc::new(Mutex::new(VideoStore::new())),
            generator,
        }
    }
}

#[async_trait]
impl<V, G> Generator for StoreAccess<V, G>
where
    V: 'static + Video + std::hash::Hash + std::cmp::Eq + std::marker::Sync + std::marker::Send,
    G: Generator<Item = V> + std::marker::Send + std::marker::Sync + 'static,
    <G as Generator>::Iterator: 'static,
{
    type Item = Arc<Mutex<V>>;
    // Better when https://github.com/rust-lang/rust/issues/63063 is stable.
    // type Iterator = impl Iterator<Item = <Self as Generator>::Item>;
    type Iterator = Box<dyn Iterator<Item = <Self as Generator>::Item>>;

    async fn generate(&self) -> (Self::Iterator, Option<crate::Error>) {
        let store = self.store.clone();
        let (gen_iter, gen_err) = self.generator.generate().await;
        let map = gen_iter.map(move |v| store.lock().unwrap().get(v));
        (Box::new(map), gen_err)
    }
}

#[cfg(test)]
mod test {
    use chrono::{NaiveDate, NaiveDateTime};

    use super::*;
    use crate::traits::generator::MockGenerator;
    use crate::traits::video::MockVideo;

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
        generator.expect_generate().returning(|| {
            (
                vec![
                    make_video(NaiveDate::from_ymd(2021, 8, 12).and_hms(0, 0, 0)),
                    make_video(NaiveDate::from_ymd(2021, 8, 11).and_hms(0, 0, 0)),
                ]
                .into_iter(),
                None,
            )
        });

        let store = StoreAccess::new(generator);

        let result = store.generate().await;

        assert!(result.1.is_none());

        let mut iter = result.0;
        let arc1 = iter.next();
        let arc2 = iter.next();

        assert!(arc1.is_some());
        assert!(arc2.is_some());
        assert!(!Arc::ptr_eq(&arc1.unwrap(), &arc2.unwrap()))
    }

    #[tokio::test]
    async fn store_access_one_generate_duplicates() {
        let mut generator = MockGenerator::new();
        generator.expect_generate().returning(|| {
            (
                vec![
                    make_video(NaiveDate::from_ymd(2021, 8, 12).and_hms(0, 0, 0)),
                    make_video(NaiveDate::from_ymd(2021, 8, 12).and_hms(0, 0, 0)),
                ]
                .into_iter(),
                None,
            )
        });

        let store = StoreAccess::new(generator);

        let result = store.generate().await;

        assert!(result.1.is_none());

        let mut iter = result.0;
        let arc1 = iter.next();
        let arc2 = iter.next();

        assert!(arc1.is_some());
        assert!(arc2.is_some());
        assert!(Arc::ptr_eq(&arc1.unwrap(), &arc2.unwrap()));
    }

    #[tokio::test]
    async fn store_access_two_generate_duplicates() {
        let mut generator = MockGenerator::new();
        generator.expect_generate().times(1).returning(|| {
            (
                vec![
                    make_video(NaiveDate::from_ymd(2021, 8, 12).and_hms(0, 0, 0)),
                    make_video(NaiveDate::from_ymd(2021, 8, 11).and_hms(0, 0, 0)),
                ]
                .into_iter(),
                None,
            )
        });
        generator.expect_generate().times(1).returning(|| {
            (
                vec![
                    make_video(NaiveDate::from_ymd(2021, 8, 12).and_hms(0, 0, 0)),
                    make_video(NaiveDate::from_ymd(2021, 8, 11).and_hms(0, 0, 0)),
                ]
                .into_iter(),
                None,
            )
        });

        let store = StoreAccess::new(generator);

        let result_1 = store.generate().await;

        let mut iter_1 = result_1.0;
        let arc1_1 = iter_1.next();
        let arc1_2 = iter_1.next();

        let result_2 = store.generate().await;

        let mut iter_2 = result_2.0;
        let arc2_1 = iter_2.next();
        let arc2_2 = iter_2.next();

        assert!(Arc::ptr_eq(&arc1_1.unwrap(), &arc2_1.unwrap()));
        assert!(Arc::ptr_eq(&arc1_2.unwrap(), &arc2_2.unwrap()));
    }
}
