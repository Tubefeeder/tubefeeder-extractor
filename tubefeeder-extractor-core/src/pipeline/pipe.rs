use crate::{Generator, Merger, StoreAccess, Subscription, SubscriptionList, Video, VideoStore};

use std::sync::{Arc, Mutex};

use async_trait::async_trait;

pub struct Pipeline<S, V> {
    subscription_list: Arc<Mutex<SubscriptionList<S>>>,
    _video_store: Arc<Mutex<VideoStore<V>>>,

    store_access: StoreAccess<V, Merger<S, V>>,
}

impl<S, V> Pipeline<S, V>
where
    S: Subscription<Video = V>,
    V: 'static + Video<Subscription = S>,
    <S as Subscription>::Iterator: std::marker::Send,
{
    pub fn new() -> Self {
        let subscription_list = Arc::new(Mutex::new(SubscriptionList::new()));
        let _video_store = Arc::new(Mutex::new(VideoStore::new()));

        let merger = Merger::new(subscription_list.clone());
        let store_access = StoreAccess::new(_video_store.clone(), merger);

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
    type Item = Arc<Mutex<V>>;

    type Iterator = Box<dyn Iterator<Item = <Self as Generator>::Item>>;

    async fn generate(&self) -> (Self::Iterator, Option<crate::Error>) {
        self.store_access.generate().await
    }
}

impl<S, V> Default for Pipeline<S, V>
where
    S: Subscription<Video = V>,
    V: 'static + Video<Subscription = S>,
    <S as Subscription>::Iterator: std::marker::Send,
{
    fn default() -> Self {
        Pipeline::new()
    }
}
