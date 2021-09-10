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
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
};

use tf_core::{ErrorStore, Generator, Pipeline};
use tf_filter::{Filter, FilterGroup};

use async_trait::async_trait;

use crate::{AnySubscription, AnySubscriptionList, AnyVideo, AnyVideoFilter};

#[derive(Clone)]
pub struct Joiner {
    subscription_list: AnySubscriptionList,

    filters: Arc<Mutex<FilterGroup<AnyVideoFilter>>>,
    #[cfg(feature = "youtube")]
    yt_pipeline: Pipeline<tf_yt::YTSubscription, tf_yt::YTVideo>,
    #[cfg(test)]
    test_pipeline: Pipeline<tf_test::TestSubscription, tf_test::TestVideo>,
}

impl Joiner {
    pub fn new() -> Self {
        #[cfg(feature = "youtube")]
        let yt_pipeline = Pipeline::new();
        #[cfg(test)]
        let test_pipeline = Pipeline::new();

        let mut subscriptions = AnySubscriptionList::default();
        #[cfg(feature = "youtube")]
        subscriptions.yt_subscriptions(yt_pipeline.subscription_list());
        #[cfg(test)]
        subscriptions.test_subscriptions(test_pipeline.subscription_list());

        Joiner {
            subscription_list: subscriptions,
            #[cfg(feature = "youtube")]
            yt_pipeline,
            #[cfg(test)]
            test_pipeline,
            filters: Arc::new(Mutex::new(FilterGroup::new())),
        }
    }

    pub fn subscription_list(&self) -> AnySubscriptionList {
        self.subscription_list.clone()
    }

    pub fn filters(&self) -> Arc<Mutex<FilterGroup<AnyVideoFilter>>> {
        self.filters.clone()
    }

    pub fn subscribe(&self, subscription: AnySubscription) {
        match subscription {
            #[cfg(feature = "youtube")]
            AnySubscription::Youtube(s) => {
                self.yt_pipeline.subscription_list().lock().unwrap().add(s)
            }
            #[cfg(test)]
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

    async fn generate(&self, errors: Arc<Mutex<ErrorStore>>) -> Self::Iterator {
        // TODO: Error handling
        // TODO: More efficient
        let mut generators: Vec<
            Pin<
                Box<
                    dyn Future<Output = Box<dyn Iterator<Item = AnyVideo> + std::marker::Send>>
                        + std::marker::Send,
                >,
            >,
        > = vec![];
        #[cfg(feature = "youtube")]
        let errors_clone = errors.clone();
        generators.push(Box::pin(async move {
            let iter = self.yt_pipeline.generate(errors_clone).await;
            let iter_mapped = iter.map(|v| v.into());
            Box::new(iter_mapped) as Box<dyn Iterator<Item = AnyVideo> + std::marker::Send>
        }));
        #[cfg(test)]
        generators.push(Box::pin(async {
            let iter = self.test_pipeline.generate(errors).await;
            let iter_mapped = iter.map(|v| v.into());
            Box::new(iter_mapped) as Box<dyn Iterator<Item = AnyVideo> + std::marker::Send>
        }));

        let results = futures::future::join_all(generators).await;
        let mut videos: Vec<AnyVideo> = results
            .into_iter()
            .map(|i| i.collect())
            .collect::<Vec<Vec<AnyVideo>>>()
            .concat();
        videos.retain(|v| !self.filters.lock().unwrap().matches(v));
        videos.sort_by_cached_key(|v| v.uploaded());
        videos.reverse();
        videos.into_iter()
    }
}

impl Default for Joiner {
    fn default() -> Self {
        Joiner::new()
    }
}
