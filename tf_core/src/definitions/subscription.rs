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

use crate::Video;

#[cfg(test)]
use {crate::mock::MockVideo, mockall::predicate::*, mockall::*};

/// A [Subscription] to a channel. The [Subscription] must be able to generate
/// [Video]s asyncronously.
#[async_trait]
pub trait Subscription:
    Clone + std::marker::Send + std::marker::Sync + std::fmt::Display + PartialEq + Eq + std::hash::Hash
{
    type Video: Video;

    fn name(&self) -> Option<String>;
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

    impl Subscription for Subscription {
        type Video = MockVideo;
        fn name(&self) -> Option<String>;
    }

    #[async_trait]
    impl crate::GeneratorWithClient for Subscription {
        type Item = MockVideo;
        type Iterator = std::vec::IntoIter<MockVideo>;
        async fn generate_with_client(&self, errors: &crate::ErrorStore, client: &reqwest::Client) -> <Self as crate::GeneratorWithClient>::Iterator;
    }
}

#[cfg(test)]
impl std::hash::Hash for MockSubscription {
    fn hash<H>(&self, _state: &mut H)
    where
        H: std::hash::Hasher,
    {
    }
}
