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

use crate::subscription::YTSubscriptionList;
use crate::{YTSubscription, YTVideo};

use tf_core::{
    ErrorStore, ExpandedVideo, Expander, Generator, StoreAccess, SubscriptionList, VideoStore,
};

use std::sync::{Arc, Mutex};

use async_trait::async_trait;

/// The [Pipeline] generating [Video]s `V` from the [Subscription]s `S`.
#[derive(Clone)]
pub struct YTPipeline {
    /// The [SubscriptionList] used in the [Merger].
    subscription_list: Arc<Mutex<SubscriptionList<YTSubscription>>>,
    /// The [VideoStore] used in the [Expander].
    video_store: Arc<Mutex<VideoStore<ExpandedVideo<YTVideo>>>>,

    /// The [Generator] to get the [Video]s from.
    store_access: StoreAccess<ExpandedVideo<YTVideo>, Expander<YTVideo, YTSubscriptionList>>,
}

impl YTPipeline {
    /// Create a new [YTPipeline] with no [Subscription]s.
    pub fn new() -> Self {
        let subscription_list = Arc::new(Mutex::new(SubscriptionList::new()));
        let video_store = Arc::new(Mutex::new(VideoStore::new()));

        let merger = YTSubscriptionList(subscription_list.clone());
        let expander = Expander::new(merger);
        let store_access = StoreAccess::new(video_store.clone(), expander);

        YTPipeline {
            subscription_list,
            video_store,

            store_access,
        }
    }

    /// Get the list of [Subscription]s used in the [YTPipeline].
    ///
    /// Modifying this [SubscriptionList] will also alter the [Subscription]s in the
    /// [YTPipeline].
    pub fn subscription_list(&self) -> Arc<Mutex<SubscriptionList<YTSubscription>>> {
        self.subscription_list.clone()
    }

    /// Upgrade a video from a normal video to a video in the video storage of the pipeline.
    pub fn upgrade_video(
        &self,
        video: &ExpandedVideo<YTVideo>,
    ) -> Arc<Mutex<ExpandedVideo<YTVideo>>> {
        self.video_store.lock().unwrap().get(video)
    }
}

#[async_trait]
impl Generator for YTPipeline {
    type Item = Arc<Mutex<ExpandedVideo<YTVideo>>>;

    type Iterator = Box<dyn Iterator<Item = <Self as Generator>::Item> + std::marker::Send>;

    async fn generate(&self, errors: &ErrorStore) -> Self::Iterator {
        self.store_access.generate(errors).await
    }
}

impl Default for YTPipeline {
    fn default() -> Self {
        YTPipeline::new()
    }
}
