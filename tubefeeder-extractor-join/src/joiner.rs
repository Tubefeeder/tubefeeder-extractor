use tf_core::{Generator, Pipeline};

use async_trait::async_trait;

use crate::{AnySubscription, AnyVideo};

pub struct Joiner {
    yt_pipeline: Pipeline<tf_yt::YTSubscription, tf_yt::YTVideo>,
    test_pipeline: Pipeline<tf_test::TestSubscription, tf_test::TestVideo>,
}

impl Joiner {
    pub fn new() -> Self {
        Joiner {
            yt_pipeline: Pipeline::new(),
            test_pipeline: Pipeline::new(),
        }
    }

    pub fn subscribe(&self, subscription: AnySubscription) {
        match subscription {
            AnySubscription::Youtube(s) => {
                self.yt_pipeline.subscription_list().lock().unwrap().add(s)
            }
            AnySubscription::Test(s) => self
                .test_pipeline
                .subscription_list()
                .lock()
                .unwrap()
                .add(s),
        }
    }
}

#[async_trait]
impl Generator for Joiner {
    type Item = AnyVideo;

    type Iterator = std::vec::IntoIter<AnyVideo>;

    async fn generate(&self) -> (Self::Iterator, Option<tf_core::Error>) {
        // TODO: Error handling
        let ((yt_iter, yt_err), (test_iter, _test_err)) =
            tokio::join!(self.yt_pipeline.generate(), self.test_pipeline.generate());

        let mut yt_any: Vec<AnyVideo> = yt_iter.map(|v| v.into()).collect();
        let mut test_any: Vec<AnyVideo> = test_iter.map(|v| v.into()).collect();

        yt_any.append(&mut test_any);

        // TODO: More efficient
        yt_any.sort_by_cached_key(|v| v.uploaded());
        yt_any.reverse();

        (yt_any.into_iter(), yt_err)
    }
}
