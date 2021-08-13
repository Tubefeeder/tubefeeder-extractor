use async_trait::async_trait;

use crate::Video;

#[cfg(test)]
use {crate::mock::MockVideo, mockall::predicate::*, mockall::*};

/// A [`Subscription`] to a channel. The [`Subscription`][Subscription] must be able to generate
/// [`Video`][crate::Video]s asyncronously.
#[async_trait]
pub trait Subscription: Clone + std::marker::Send + std::marker::Sync {
    type Video: crate::Video;
    type Iterator: Iterator<Item = Self::Video>;
    async fn generate(&self) -> (Self::Iterator, Option<crate::Error>);
}

#[async_trait]
impl<S, V> super::generator::Generator for S
where
    S: Subscription<Video = V> + std::marker::Sync + std::marker::Send,
    V: Video<Subscription = S>,
{
    type Item = V;

    type Iterator = <S as Subscription>::Iterator;

    async fn generate(&self) -> (<S as Subscription>::Iterator, Option<crate::Error>) {
        self.generate().await
    }
}

#[cfg(test)]
mock! {
    pub(crate) Subscription {}

    impl Clone for Subscription {
        fn clone(&self) -> Self;
    }


    #[async_trait]
    impl Subscription for Subscription {
        type Video = MockVideo;
        type Iterator = std::vec::IntoIter<MockVideo>;
        async fn generate(&self) -> (<Self as Subscription>::Iterator, Option<crate::Error>);
    }
}
