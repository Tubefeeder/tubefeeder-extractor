//! The traits [`Observable`] and [`Observer`] construct the typical observer-pattern.
//!
//! The messages parsed in between [`Observable`] and [`Observer`] is the generic variable `T`.
//! Because keeping track of all [`Observer`]s when implementing [`Observable`] can be hard,
//! this module also contains a [`ObserverList`] where [`Observer`]s can be
//! [`attached`][ObserverList::attach], [`detached`][ObserverList::detach] and
//! [`notified`][ObserverList::notify].

use std::sync::Mutex;
use std::sync::Weak;

#[cfg(test)]
use mockall::predicate::*;
#[cfg(test)]
use mockall::*;

/// A [`Observer`] observing a [`Observable<T>`].
/// `T` is the message being sent from the [`Observable<T>`] to the [`Observer<T>`].
#[cfg_attr(test, automock)]
pub trait Observer<T> {
    /// The [`Observable<T>`] sending a message to the [`Observer<T>`].
    fn notify(&mut self, message: T);
}

/// A [`Observable`] that can be observed by [`Observer<T>`]
/// `T` is the message being sent from the [`Observable<T>`] to the [`Observer<T>`].
/// This should be implemented using the [`ObserverList<T>`].
pub trait Observable<T> {
    /// Attach a [`Observer<T>`] to the [`Observable`].
    /// Should be implemented using [`ObserverList::attach`].
    fn attach(&mut self, observer: Weak<Mutex<Box<dyn Observer<T>>>>);
    /// Detach a [`Observer<T>`] to the [`Observable`].
    /// Should be implemented using [`ObserverList::detach`].
    fn detach(&mut self, observer: Weak<Mutex<Box<dyn Observer<T>>>>);
}

/// A list of [`Observer<T>`] using the message `T`.
pub struct ObserverList<T> {
    /// The [`Observer<T>`] list.
    observers: Vec<Weak<Mutex<Box<dyn Observer<T>>>>>,
}

impl<T> ObserverList<T> {
    /// Create a new [`ObserverList<T>`] with no [`Observer`]s.
    pub fn new() -> Self {
        ObserverList { observers: vec![] }
    }

    /// Attach a [`Observer<T>`] to the [`ObserverList`].
    pub fn attach(&mut self, observer: Weak<Mutex<Box<dyn Observer<T>>>>) {
        self.observers.push(observer);
    }

    /// Detach a [`Observer<T>`] to the [`ObserverList`].
    /// This will also detach all dropped [`Observer`]s.
    pub fn detach(&mut self, observer: Weak<Mutex<Box<dyn Observer<T>>>>) {
        self.observers
            .retain(|o| o.upgrade().is_some() && !o.ptr_eq(&observer));
    }
}

impl<T: Clone> ObserverList<T> {
    /// Notify all [`Observer<T>`] in the list with the given message.
    /// Only a clone of the message and not the real object will be sent.
    pub fn notify(&self, message: T) {
        self.observers.iter().for_each(|o| {
            if let Some(mutex) = o.upgrade() {
                if let Ok(mut observer) = mutex.lock() {
                    observer.notify(message.clone());
                }
            }
        })
    }
}

impl<T> Default for ObserverList<T> {
    fn default() -> Self {
        ObserverList::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn observer_list_attach_detach() {
        let mut observer_list = ObserverList::new();

        let observer1 = MockObserver::new();

        let observer1_ref = Arc::new(Mutex::new(Box::new(observer1) as Box<dyn Observer<u64>>));

        observer_list.attach(Arc::downgrade(&observer1_ref));

        assert_eq!(
            1,
            observer_list.observers.len(),
            "The observable does not have the correct amount of observers"
        );

        observer_list.detach(Arc::downgrade(&observer1_ref));

        assert_eq!(
            0,
            observer_list.observers.len(),
            "The observable does not have the correct amount of observers"
        );
    }

    #[test]
    fn observer_list_notify() {
        let mut observer_list = ObserverList::new();

        let mut observer1 = MockObserver::new();
        observer1
            .expect_notify()
            .with(predicate::eq(10u64))
            .times(1)
            .returning(|_| ());

        let observer1_ref = Arc::new(Mutex::new(Box::new(observer1) as Box<dyn Observer<u64>>));

        observer_list.attach(Arc::downgrade(&observer1_ref));
        observer_list.notify(10);
    }

    #[test]
    fn observable_multi_observer() {
        let mut observer_list = ObserverList::new();

        let mut observer1 = MockObserver::new();
        observer1
            .expect_notify()
            .with(predicate::eq(10u64))
            .times(1)
            .returning(|_| ());
        observer1
            .expect_notify()
            .with(predicate::eq(20u64))
            .times(1)
            .returning(|_| ());

        let mut observer2 = MockObserver::new();
        observer2
            .expect_notify()
            .with(predicate::eq(20u64))
            .times(1)
            .returning(|_| ());

        let observer1_ref = Arc::new(Mutex::new(Box::new(observer1) as Box<dyn Observer<u64>>));
        let observer2_ref = Arc::new(Mutex::new(Box::new(observer2) as Box<dyn Observer<u64>>));

        observer_list.attach(Arc::downgrade(&observer1_ref));
        observer_list.notify(10);

        observer_list.attach(Arc::downgrade(&observer2_ref));
        observer_list.notify(20);
    }

    #[test]
    fn observer_list_test_drop_inactive() {
        let mut observer_list = ObserverList::new();

        let observer1 = MockObserver::new();

        let observer1_ref = Arc::new(Mutex::new(Box::new(observer1) as Box<dyn Observer<u64>>));

        observer_list.attach(Arc::downgrade(&observer1_ref));

        assert_eq!(
            1,
            observer_list.observers.len(),
            "The observable does not have the correct amount of observers"
        );

        observer_list.detach(Weak::new());

        assert_eq!(
            1,
            observer_list.observers.len(),
            "The observable does not have the correct amount of observers"
        );

        drop(observer1_ref);
        observer_list.detach(Weak::new());

        assert_eq!(
            0,
            observer_list.observers.len(),
            "The observable does not have the correct amount of observers"
        );
    }
}
