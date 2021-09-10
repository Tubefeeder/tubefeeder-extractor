use std::sync::{Mutex, Weak};

use tf_core::{Observable, Observer, ObserverList};

pub trait Filter {
    type Item;

    fn matches(&self, item: &Self::Item) -> bool;
}

pub struct FilterGroup<T> {
    observers: ObserverList<FilterEvent<T>>,

    filters: Vec<T>,
}

impl<T> FilterGroup<T> {
    pub fn new() -> Self {
        Self {
            filters: vec![],
            observers: ObserverList::new(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.filters.iter()
    }
}

impl<T: Clone> FilterGroup<T> {
    pub fn add(&mut self, filter: T) {
        self.filters.push(filter.clone());
        self.observers.notify(FilterEvent::Add(filter));
    }
}

impl<T: PartialEq + Clone> FilterGroup<T> {
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

#[derive(Clone)]
pub enum FilterEvent<T> {
    Add(T),
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
