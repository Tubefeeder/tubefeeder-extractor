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

//! This contains [Filter] to filter out [Filter::Item]s and [FilterGroup] to group
//! [Filter]s of the same kind.

use std::sync::{Mutex, Weak};

use tf_observer::{Observable, Observer, ObserverList};

/// A [Filter] that matches on [Filter::Item].
pub trait Filter {
    /// The item matched on.
    type Item;

    /// Whether the filter matches on the give on [Filter::Item].
    fn matches(&self, item: &Self::Item) -> bool;
}

/// A group of [Filter]s of type `T`.
///
/// This implements [Observable] with the event [FilterEvent].
pub struct FilterGroup<T> {
    // The [Observer]s
    observers: ObserverList<FilterEvent<T>>,

    /// The saved [Filter]s.
    filters: Vec<T>,
}

impl<T> FilterGroup<T> {
    /// Create a new [FilterGroup].
    ///
    /// This will not have any [Filter]s or [Observer]s.
    pub fn new() -> Self {
        Self {
            filters: vec![],
            observers: ObserverList::new(),
        }
    }

    /// Iterate over all the stored [Filter]s.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.filters.iter()
    }
}

impl<T: Clone> FilterGroup<T> {
    /// Add a [Filter] to the [FilterGroup].
    ///
    /// This will notify the [Observer]s with [FilterEvent::Add].
    pub fn add(&mut self, filter: T) {
        self.filters.push(filter.clone());
        self.observers.notify(FilterEvent::Add(filter));
    }
}

impl<T: PartialEq + Clone> FilterGroup<T> {
    /// Remove a [Filter] to the [FilterGroup].
    ///
    /// This will notify the [Observer]s with [FilterEvent::Remove].
    pub fn remove(&mut self, filter: &T) {
        self.filters.retain(|t| t != filter);
        self.observers.notify(FilterEvent::Remove(filter.clone()));
    }
}

impl<I, T: Filter<Item = I>> Filter for FilterGroup<T> {
    type Item = I;

    fn matches(&self, item: &Self::Item) -> bool {
        self.filters.iter().any(move |f| f.matches(item))
    }
}

/// The event called from [FilterGroup].
#[derive(Clone)]
pub enum FilterEvent<T> {
    /// A [Filter] was added to the [FilterGroup]. See [FilterGroup::add].
    Add(T),
    /// A [Filter] was removed from the [FilterGroup]. See [FilterGroup::remove].
    Remove(T),
}

impl<T: Clone> Observable<FilterEvent<T>> for FilterGroup<T> {
    fn attach(
        &mut self,
        observer: Weak<Mutex<Box<(dyn Observer<FilterEvent<T>> + Send + 'static)>>>,
    ) {
        self.observers.attach(observer);
    }
    fn detach(
        &mut self,
        observer: Weak<Mutex<Box<(dyn Observer<FilterEvent<T>> + Send + 'static)>>>,
    ) {
        self.observers.detach(observer);
    }
}
