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

use tf_core::SubscriptionList;
use tf_observer::{Observable, Observer, ObserverList};
#[cfg(test)]
use tf_test::TestSubscription;
#[cfg(feature = "youtube")]
use tf_yt::YTSubscription;

use crate::AnySubscription;

/// A wrapper around all the available [SubscriptionList] of the platforms.
///
/// This implements [Observable] and emits [SubscriptionEvent] to the [Observer]s.
#[derive(Clone)]
pub struct AnySubscriptionList {
    observers: ObserverList<SubscriptionEvent>,

    #[cfg(feature = "youtube")]
    yt_subscriptions: Arc<Mutex<SubscriptionList<YTSubscription>>>,
    #[cfg(test)]
    test_subscriptions: Arc<Mutex<SubscriptionList<TestSubscription>>>,
}

impl AnySubscriptionList {
    /// Create a new [AnySubscriptionList].
    ///
    /// This [AnySubscriptionList] will have no observers and the wrapped [SubscriptionList]s
    /// will be empty.
    pub(crate) fn new() -> Self {
        AnySubscriptionList {
            observers: ObserverList::default(),

            #[cfg(feature = "youtube")]
            yt_subscriptions: Arc::new(Mutex::new(SubscriptionList::default())),
            #[cfg(test)]
            test_subscriptions: Arc::new(Mutex::new(SubscriptionList::default())),
        }
    }

    /// Set the wrapped [SubscriptionList] for youtube.
    #[cfg(feature = "youtube")]
    pub(crate) fn yt_subscriptions(&mut self, sub: Arc<Mutex<SubscriptionList<YTSubscription>>>) {
        self.yt_subscriptions = sub;
    }

    /// Set the wrapped [SubscriptionList] for tests.
    #[cfg(test)]
    pub(crate) fn test_subscriptions(
        &mut self,
        sub: Arc<Mutex<SubscriptionList<TestSubscription>>>,
    ) {
        self.test_subscriptions = sub;
    }

    /// Add a [AnySubscription] to the [AnySubscriptionList].
    ///
    /// This will notify all [Observer]s with [SubscriptionEvent::Add].
    pub fn add(&self, subscription: AnySubscription) {
        match subscription.clone() {
            #[cfg(feature = "youtube")]
            AnySubscription::Youtube(sub) => self.yt_subscriptions.lock().unwrap().add(sub),
            #[cfg(test)]
            AnySubscription::Test(sub) => self.test_subscriptions.lock().unwrap().add(sub),
        }
        self.observers.notify(SubscriptionEvent::Add(subscription))
    }

    /// Remove a [AnySubscription] to the [AnySubscriptionList].
    ///
    /// This will notify all [Observer]s with [SubscriptionEvent::Remove].
    pub fn remove(&self, subscription: AnySubscription) {
        match subscription.clone() {
            #[cfg(feature = "youtube")]
            AnySubscription::Youtube(sub) => self.yt_subscriptions.lock().unwrap().remove(sub),
            #[cfg(test)]
            AnySubscription::Test(sub) => self.test_subscriptions.lock().unwrap().remove(sub),
        }
        self.observers
            .notify(SubscriptionEvent::Remove(subscription))
    }

    /// Iterate over all stored [AnySubscription]s.
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
                .collect::<Vec<AnySubscription>>(),
        );
        #[cfg(test)]
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

/// The event sent by [AnySubscriptionList].
#[derive(Clone, Debug)]
pub enum SubscriptionEvent {
    /// A [AnySubscription] was added. See [AnySubscriptionList::add].
    Add(AnySubscription),
    /// A [AnySubscription] was removed. See [AnySubscriptionList::remove].
    Remove(AnySubscription),
}

impl Observable<SubscriptionEvent> for AnySubscriptionList {
    fn attach(
        &mut self,
        observer: std::sync::Weak<Mutex<Box<dyn Observer<SubscriptionEvent> + Send>>>,
    ) {
        self.observers.attach(observer)
    }

    fn detach(
        &mut self,
        observer: std::sync::Weak<Mutex<Box<dyn Observer<SubscriptionEvent> + Send>>>,
    ) {
        self.observers.detach(observer)
    }
}
