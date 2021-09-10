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

    pub fn remove(&self, subscription: AnySubscription) {
        log::debug!("Removing subscription {}", subscription);
        match subscription.clone() {
            #[cfg(feature = "youtube")]
            AnySubscription::Youtube(sub) => self.yt_subscriptions.lock().unwrap().remove(sub),
            #[cfg(feature = "testPlatform")]
            AnySubscription::Test(sub) => self.test_subscriptions.lock().unwrap().remove(sub),
        }
        self.observers
            .notify(SubscriptionEvent::Remove(subscription))
    }

    pub fn iter(&self) -> impl Iterator<Item = AnySubscription> {
        let mut vec = vec![];
        #[cfg(feature = "youtube")]
        vec.append(
            &mut self
                .yt_subscriptions
                .lock()
                .unwrap()
                .subscriptions()
                .into_iter()
                .map(|s| s.into())
                .collect::<Vec<AnySubscription>>()
                .clone(),
        );
        #[cfg(feature = "testPlatform")]
        vec.append(
            &mut self
                .test_subscriptions
                .lock()
                .unwrap()
                .subscriptions()
                .into_iter()
                .map(|s| s.into())
                .collect::<Vec<AnySubscription>>()
                .clone(),
        );

        vec.into_iter()
    }
}

impl Default for AnySubscriptionList {
    fn default() -> Self {
        AnySubscriptionList::new()
    }
}

#[derive(Clone, Debug)]
pub enum SubscriptionEvent {
    Add(AnySubscription),
    Remove(AnySubscription),
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
