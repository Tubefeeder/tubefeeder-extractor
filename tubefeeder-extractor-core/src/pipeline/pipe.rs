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
    ErrorStore, ExpandedVideo, Expander, Generator, Merger, StoreAccess, Subscription,
    SubscriptionList, Video, VideoStore,
};

use std::sync::{Arc, Mutex};

use async_trait::async_trait;

#[derive(Clone)]
pub struct Pipeline<S, V> {
    subscription_list: Arc<Mutex<SubscriptionList<S>>>,
    _video_store: Arc<Mutex<VideoStore<ExpandedVideo<V>>>>,

    store_access: StoreAccess<ExpandedVideo<V>, Expander<V, Merger<S, V>>>,
}

impl<S, V> Pipeline<S, V>
where
    S: 'static + Subscription<Video = V>,
    V: 'static + Video<Subscription = S>,
    <S as Subscription>::Iterator: std::marker::Send,
{
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

    pub fn subscription_list(&self) -> Arc<Mutex<SubscriptionList<S>>> {
        self.subscription_list.clone()
    }
}

#[async_trait]
impl<S, V> Generator for Pipeline<S, V>
where
    S: 'static + Subscription<Video = V>,
    V: 'static + Video<Subscription = S>,
    <S as Subscription>::Iterator: std::marker::Send,
{
    type Item = Arc<Mutex<ExpandedVideo<V>>>;

    type Iterator = Box<dyn Iterator<Item = <Self as Generator>::Item> + std::marker::Send>;

    async fn generate(&self, errors: Arc<Mutex<ErrorStore>>) -> Self::Iterator {
        self.store_access.generate(errors).await
    }
}

impl<S, V> Default for Pipeline<S, V>
where
    S: 'static + Subscription<Video = V>,
    V: 'static + Video<Subscription = S>,
    <S as Subscription>::Iterator: std::marker::Send,
{
    fn default() -> Self {
        Pipeline::new()
    }
}
