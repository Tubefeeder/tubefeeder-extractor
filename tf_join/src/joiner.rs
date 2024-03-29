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

use tf_core::{ErrorStore, Generator, Pipeline, Video};
use tf_filter::{Filter, FilterGroup};

use async_trait::async_trait;
use tf_yt::YTPipeline;

use crate::{AnySubscriptionList, AnyVideo, AnyVideoFilter};

/// Join multiple platforms together into one [Generator].
///
/// This will handle the generation and filtering of videos.
#[derive(Clone)]
pub struct Joiner {
    /// The [AnySubscriptionList] used to generate the [AnyVideo]s.
    subscription_list: AnySubscriptionList,

    /// The [FilterGroup] used to filter out [AnyVideo]s.
    filters: Arc<Mutex<FilterGroup<AnyVideoFilter>>>,
    #[cfg(feature = "youtube")]
    yt_pipeline: YTPipeline,
    #[cfg(feature = "peertube")]
    pt_pipeline: Pipeline<tf_pt::PTSubscription, tf_pt::PTVideo>,
    #[cfg(feature = "lbry")]
    lbry_pipeline: Pipeline<tf_lbry::LbrySubscription, tf_lbry::LbryVideo>,
    // -- Add value here.
    #[cfg(test)]
    test_pipeline: Pipeline<tf_test::TestSubscription, tf_test::TestVideo>,
}

impl Joiner {
    /// Create a new [Joiner] with no [AnySubscription][crate::AnySubscription]s and no [Filter][tf_filter::Filter]s.
    pub fn new() -> Self {
        #[cfg(feature = "youtube")]
        let yt_pipeline = YTPipeline::new();
        #[cfg(feature = "peertube")]
        let pt_pipeline = Pipeline::new();
        #[cfg(feature = "lbry")]
        let lbry_pipeline = Pipeline::new();
        // -- Add value here.
        #[cfg(test)]
        let test_pipeline = Pipeline::new();

        let mut subscriptions = AnySubscriptionList::default();
        #[cfg(feature = "youtube")]
        subscriptions.yt_subscriptions(yt_pipeline.subscription_list());
        #[cfg(feature = "peertube")]
        subscriptions.pt_subscriptions(pt_pipeline.subscription_list());
        #[cfg(feature = "lbry")]
        subscriptions.lbry_subscriptions(lbry_pipeline.subscription_list());
        // -- Add function call here.
        #[cfg(test)]
        subscriptions.test_subscriptions(test_pipeline.subscription_list());

        Joiner {
            subscription_list: subscriptions,
            #[cfg(feature = "youtube")]
            yt_pipeline,
            #[cfg(feature = "peertube")]
            pt_pipeline,
            #[cfg(feature = "lbry")]
            lbry_pipeline,
            // -- Add value here.
            #[cfg(test)]
            test_pipeline,
            filters: Arc::new(Mutex::new(FilterGroup::new())),
        }
    }

    /// Get the [AnySubscriptionList] used to generate the [AnyVideo]s.
    ///
    /// When modifying this list, it will also change the [AnySubscription][crate::AnySubscription]s
    /// of this [Joiner].
    pub fn subscription_list(&self) -> AnySubscriptionList {
        self.subscription_list.clone()
    }

    /// Get the [FilterGroup] used to filter out [AnyVideo]s.
    pub fn filters(&self) -> Arc<Mutex<FilterGroup<AnyVideoFilter>>> {
        self.filters.clone()
    }

    /// Upgrades a normal [AnyVideo] into a [AnyVideo] in the video storage of the pipelines.
    pub fn upgrade_video(&self, video: &AnyVideo) -> AnyVideo {
        match video {
            #[cfg(feature = "youtube")]
            AnyVideo::Youtube(v) => self.yt_pipeline.upgrade_video(&v.lock().unwrap()).into(),
            #[cfg(feature = "peertube")]
            AnyVideo::Peertube(v) => self.pt_pipeline.upgrade_video(&v.lock().unwrap()).into(),
            #[cfg(feature = "lbry")]
            AnyVideo::Lbry(v) => self.lbry_pipeline.upgrade_video(&v.lock().unwrap()).into(),
            // -- Add case here.
            #[cfg(test)]
            AnyVideo::Test(v) => self.test_pipeline.upgrade_video(&v.lock().unwrap()).into(),
        }
    }
}

#[async_trait]
impl Generator for Joiner {
    type Item = AnyVideo;

    type Iterator = std::vec::IntoIter<AnyVideo>;

    async fn generate(&self, errors: &ErrorStore) -> Self::Iterator {
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
        generators.push(Box::pin(async move {
            let iter = self.yt_pipeline.generate(errors).await;
            let iter_mapped = iter.map(|v| v.into());
            Box::new(iter_mapped) as Box<dyn Iterator<Item = AnyVideo> + std::marker::Send>
        }));
        #[cfg(feature = "peertube")]
        generators.push(Box::pin(async move {
            let iter = self.pt_pipeline.generate(errors).await;
            let iter_mapped = iter.map(|v| v.into());
            Box::new(iter_mapped) as Box<dyn Iterator<Item = AnyVideo> + std::marker::Send>
        }));
        #[cfg(feature = "lbry")]
        generators.push(Box::pin(async move {
            let iter = self.lbry_pipeline.generate(errors).await;
            let iter_mapped = iter.map(|v| v.into());
            Box::new(iter_mapped) as Box<dyn Iterator<Item = AnyVideo> + std::marker::Send>
        }));
        // -- Add generators.push here.
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
        videos
            .iter()
            .map(|v| v.subscription())
            .for_each(|s| self.subscription_list.update(s));
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
