use std::{collections::hash_map::DefaultHasher, hash::Hasher};

pub trait Filter {
    type Item;

    fn matches(&self, item: &Self::Item) -> bool;

    /// Get a identifier similar to a hash for the filter.
    fn id(&self) -> u64;
}

pub struct FilterGroup<T> {
    filters: Vec<Box<dyn Filter<Item = T> + Send>>,
}

impl<T> FilterGroup<T> {
    pub fn new() -> Self {
        Self { filters: vec![] }
    }

    pub fn add<F: 'static + Filter<Item = T> + Send>(&mut self, filter: F) {
        self.filters
            .push(Box::new(filter) as Box<dyn Filter<Item = T> + Send>);
    }

    pub fn remove<F: 'static + Filter<Item = T>>(&mut self, filter: &F) {
        self.filters.retain(|f| f.id() == filter.id())
    }
}

impl<T> Filter for FilterGroup<T> {
    type Item = T;

    fn matches(&self, item: &Self::Item) -> bool {
        self.filters.iter().any(move |f| f.matches(item))
    }

    fn id(&self) -> u64 {
        let mut s = DefaultHasher::new();
        for f in &self.filters {
            s.write_u64(f.id());
        }
        s.finish()
    }
}
