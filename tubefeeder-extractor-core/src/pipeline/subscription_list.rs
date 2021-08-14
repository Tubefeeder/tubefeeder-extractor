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
