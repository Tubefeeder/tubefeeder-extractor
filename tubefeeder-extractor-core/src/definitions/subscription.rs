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

use async_trait::async_trait;

use crate::{ErrorStore, Video};

#[cfg(test)]
use {crate::mock::MockVideo, mockall::predicate::*, mockall::*};

/// A [Subscription] to a channel. The [Subscription] must be able to generate
/// [Video]s asyncronously.
#[async_trait]
pub trait Subscription:
    Clone
    + std::marker::Send
    + std::marker::Sync
    + std::fmt::Display
    + PartialEq
    + Eq
    + PartialOrd
    + Ord
{
    /// The type of video that will be generated.
    type Video: crate::Video;

    /// The type of iterator that will be returned.
    type Iterator: Iterator<Item = Self::Video>;

    /// Generate the [Video]s asyncronously with the default [reqwest::Client::new].
    /// All [Error][crate::Error]s should be put into the given [ErrorStore].
    async fn generate(&self, errors: &ErrorStore) -> Self::Iterator {
        self.generate_with_client(errors, &reqwest::Client::new())
            .await
    }

    /// Generate the [Video]s asyncronously with the given [reqwest::Client].
    /// All [Error][crate::Error]s should be put into the given [ErrorStore].
    async fn generate_with_client(
        &self,
        errors: &ErrorStore,
        client: &reqwest::Client,
    ) -> Self::Iterator;
}

#[async_trait]
impl<S, V> super::generator::Generator for S
where
    S: Subscription<Video = V> + std::marker::Sync + std::marker::Send,
    V: Video<Subscription = S>,
{
    type Item = V;

    type Iterator = <S as Subscription>::Iterator;

    async fn generate(&self, errors: &ErrorStore) -> <S as Subscription>::Iterator {
        self.generate(errors).await
    }
}

#[cfg(test)]
mock! {
    pub(crate) Subscription {}

    impl Clone for Subscription {
        fn clone(&self) -> Self;
    }

    impl std::fmt::Display for Subscription {
        fn fmt<'a>(&self, f: &mut std::fmt::Formatter<'a>) -> Result<(), std::fmt::Error> {
            write!(f, "Test")
        }
    }

    impl std::cmp::PartialEq for Subscription {
        fn eq(&self, _other: &MockSubscription) -> bool {
            false
        }
    }

    impl Eq for Subscription {}

    impl std::cmp::PartialOrd for Subscription {
        fn partial_cmp(&self, _other: &MockSubscription) -> Option<std::cmp::Ordering> {
            None
        }
    }

    impl std::cmp::Ord for Subscription {
        fn cmp(&self, _other: &MockSubscription) -> std::cmp::Ordering {
            Ordering::Eq
        }
    }

    #[async_trait]
    impl Subscription for Subscription {
        type Video = MockVideo;
        type Iterator = std::vec::IntoIter<MockVideo>;
        async fn generate_with_client(&self, errors: &ErrorStore, client: &reqwest::Client) -> <Self as Subscription>::Iterator;
        async fn generate(&self, errors: &ErrorStore) -> <Self as Subscription>::Iterator;
    }
}
