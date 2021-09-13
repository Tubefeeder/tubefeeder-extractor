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

use crate::{
    ErrorStore, ExpandedVideo, Expander, Generator, GeneratorWithClient, Merger, StoreAccess,
    Subscription, SubscriptionList, Video, VideoStore,
};

use std::sync::{Arc, Mutex};

use async_trait::async_trait;

/// The [Pipeline] generating [Video]s `V` from the [Subscription]s `S`.
#[derive(Clone)]
pub struct Pipeline<S, V> {
    /// The [SubscriptionList] used in the [Merger].
    subscription_list: Arc<Mutex<SubscriptionList<S>>>,
    /// The [VideoStore] used in the [Expander].
    _video_store: Arc<Mutex<VideoStore<ExpandedVideo<V>>>>,

    /// The [Generator] to get the [Video]s from.
    store_access: StoreAccess<ExpandedVideo<V>, Expander<V, Merger<S, V>>>,
}

impl<S, V> Pipeline<S, V>
where
    S: 'static + Subscription<Video = V> + GeneratorWithClient<Item = V> + Generator<Item = V>,
    V: 'static + Video<Subscription = S>,
    <S as GeneratorWithClient>::Iterator: std::marker::Send,
{
    /// Create a new [Pipeline] with no [Subscription]s.
    pub fn new() -> Self {
        let subscription_list = Arc::new(Mutex::new(SubscriptionList::new()));
        let _video_store = Arc::new(Mutex::new(VideoStore::new()));

        let merger = Merger::new(subscription_list.clone());
        let expander = Expander::new(merger);
        let store_access = StoreAccess::new(_video_store.clone(), expander);

        Pipeline {
            subscription_list,
            _video_store,

            store_access,
        }
    }

    /// Get the list of [Subscription]s used in the [Pipeline].
    ///
    /// Modifying this [SubscriptionList] will also alter the [Subscription]s in the
    /// [Pipeline].
    pub fn subscription_list(&self) -> Arc<Mutex<SubscriptionList<S>>> {
        self.subscription_list.clone()
    }
}

#[async_trait]
impl<S, V> Generator for Pipeline<S, V>
where
    S: 'static + Subscription<Video = V> + GeneratorWithClient<Item = V> + Generator<Item = V>,
    V: 'static + Video<Subscription = S>,
    <S as GeneratorWithClient>::Iterator: std::marker::Send,
{
    type Item = Arc<Mutex<ExpandedVideo<V>>>;

    type Iterator = Box<dyn Iterator<Item = <Self as Generator>::Item> + std::marker::Send>;

    async fn generate(&self, errors: &ErrorStore) -> Self::Iterator {
        self.store_access.generate(errors).await
    }
}

impl<S, V> Default for Pipeline<S, V>
where
    S: 'static + Subscription<Video = V> + GeneratorWithClient<Item = V> + Generator<Item = V>,
    V: 'static + Video<Subscription = S>,
    <S as GeneratorWithClient>::Iterator: std::marker::Send,
{
    fn default() -> Self {
        Pipeline::new()
    }
}
