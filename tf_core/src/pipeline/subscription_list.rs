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

use crate::{Subscription, Video};

/// A list of [Subscription]s.
pub struct SubscriptionList<S> {
    subscriptions: Vec<S>,
}

impl<V, S> SubscriptionList<S>
where
    V: Video<Subscription = S>,
    S: Subscription<Video = V>,
{
    /// Generate a new, empty [SubscriptionList].
    pub fn new() -> Self {
        SubscriptionList {
            subscriptions: vec![],
        }
    }

    /// Add a [Subscription] to the [SubscriptionList].
    ///
    /// This does not currently check for duplicates.
    pub fn add(&mut self, subscription: S) {
        self.subscriptions.push(subscription);
    }

    /// Remove a [Subscription] from the [SubscriptionList].
    pub fn remove(&mut self, subscription: S) {
        self.subscriptions.retain(|s| s != &subscription);
    }

    /// Update a [Subscription] from the [SubscriptionList].
    pub fn update(&mut self, subscription: S) {
        self.remove(subscription.clone());
        self.add(subscription)
    }

    /// Get a [Vec] of all [Subscription]s.
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
