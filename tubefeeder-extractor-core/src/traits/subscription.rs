use std::fmt;

/// A [`Subscription`] to a channel. The [`Subscription`] must be able to generate a [`Generator`]
/// that can fetch the [`Video`][crate::Video]s of the [`Subscription`].
pub trait Subscription {
    type Generator;
    /// Get the [`Generator`] to generate the [`crate::Video`]s of the subscription.
    fn generator(&self) -> Self::Generator;
}

impl<T> fmt::Debug for dyn Subscription<Generator = T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO
        write!(f, "")
    }
}

impl<T> PartialEq<dyn Subscription<Generator = T>> for dyn Subscription<Generator = T> {
    fn eq(&self, _other: &dyn Subscription<Generator = T>) -> bool {
        // TODO
        true
    }
}
