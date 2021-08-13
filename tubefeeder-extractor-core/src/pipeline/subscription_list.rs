use crate::{Subscription, Video};

pub struct SubscriptionList<S> {
    subscriptions: Vec<S>,
}

impl<V, S> SubscriptionList<S>
where
    V: Video<Subscription = S>,
    S: Subscription<Video = V>,
{
    pub fn new() -> Self {
        SubscriptionList {
            subscriptions: vec![],
        }
    }

    pub fn add(&mut self, subscription: S) {
        self.subscriptions.push(subscription);
    }
    pub fn subscriptions(&self) -> Vec<S> {
        self.subscriptions.clone()
    }
}

impl<S, V> Default for SubscriptionList<S>
where
    V: Video<Subscription = S>,
    S: Subscription<Video = V>,
{
    fn default() -> Self {
        SubscriptionList::new()
    }
}
