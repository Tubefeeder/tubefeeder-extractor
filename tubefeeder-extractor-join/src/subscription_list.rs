use std::sync::{Arc, Mutex};

use tf_core::{Observable, ObserverList, SubscriptionList};
#[cfg(feature = "testPlatform")]
use tf_test::TestSubscription;

#[cfg(feature = "youtube")]
use tf_yt::YTSubscription;

use crate::AnySubscription;

#[derive(Clone)]
pub struct AnySubscriptionList {
    observers: ObserverList<SubscriptionEvent>,

    #[cfg(feature = "youtube")]
    yt_subscriptions: Arc<Mutex<SubscriptionList<YTSubscription>>>,
    #[cfg(feature = "testPlatform")]
    test_subscriptions: Arc<Mutex<SubscriptionList<TestSubscription>>>,
}

impl AnySubscriptionList {
    pub(crate) fn new() -> Self {
        AnySubscriptionList {
            observers: ObserverList::default(),

            #[cfg(feature = "youtube")]
            yt_subscriptions: Arc::new(Mutex::new(SubscriptionList::default())),
            #[cfg(feature = "testPlatform")]
            test_subscriptions: Arc::new(Mutex::new(SubscriptionList::default())),
        }
    }

    #[cfg(feature = "youtube")]
    pub(crate) fn yt_subscriptions(&mut self, sub: Arc<Mutex<SubscriptionList<YTSubscription>>>) {
        self.yt_subscriptions = sub;
    }

    #[cfg(feature = "testPlatform")]
    pub(crate) fn test_subscriptions(
        &mut self,
        sub: Arc<Mutex<SubscriptionList<TestSubscription>>>,
    ) {
        self.test_subscriptions = sub;
    }

    pub fn add(&self, subscription: AnySubscription) {
        match subscription.clone() {
            #[cfg(feature = "youtube")]
            AnySubscription::Youtube(sub) => self.yt_subscriptions.lock().unwrap().add(sub),
            #[cfg(feature = "testPlatform")]
            AnySubscription::Test(sub) => self.test_subscriptions.lock().unwrap().add(sub),
        }
        self.observers.notify(SubscriptionEvent::Add(subscription))
    }
}

impl Default for AnySubscriptionList {
    fn default() -> Self {
        AnySubscriptionList::new()
    }
}

#[derive(Clone)]
pub enum SubscriptionEvent {
    Add(AnySubscription),
    _Remove(AnySubscription),
}

impl Observable<SubscriptionEvent> for AnySubscriptionList {
    fn attach(
        &mut self,
        observer: std::sync::Weak<Mutex<Box<dyn tf_core::Observer<SubscriptionEvent> + Send>>>,
    ) {
        self.observers.attach(observer)
    }

    fn detach(
        &mut self,
        observer: std::sync::Weak<Mutex<Box<dyn tf_core::Observer<SubscriptionEvent> + Send>>>,
    ) {
        self.observers.detach(observer)
    }
}
