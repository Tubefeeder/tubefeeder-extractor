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

use crate::ErrorStore;

#[cfg(test)]
use {crate::mock::MockVideo, mockall::predicate::*, mockall::*};

/// Generate a [Generator::Iterator] of [Generator::Item] asyncronously.
#[async_trait]
pub trait Generator {
    /// The item being generated.
    type Item;

    /// The outcoming [Iterator].
    type Iterator: Iterator<Item = Self::Item>;

    /// Generate [Self::Item] asyncronously and putting all [Error][crate::Error]s into the given [ErrorStore].
    async fn generate(&self, errors: &ErrorStore) -> Self::Iterator;
}

/// Generate a [Generator::Iterator] of [Generator::Item] given a [reqwest::Client] asyncronously.
#[async_trait]
pub trait GeneratorWithClient {
    /// The item being generated.
    type Item;

    /// The outcoming [Iterator].
    type Iterator: Iterator<Item = Self::Item>;

    /// Generate [Self::Item] asyncronously and putting all [Error][crate::Error]s into the given [ErrorStore].
    async fn generate_with_client(
        &self,
        errors: &ErrorStore,
        client: &reqwest::Client,
    ) -> Self::Iterator;
}

#[async_trait]
impl<T> Generator for T
where
    T: GeneratorWithClient + std::marker::Sync,
{
    type Item = <T as GeneratorWithClient>::Item;

    type Iterator = <T as GeneratorWithClient>::Iterator;

    async fn generate(&self, errors: &ErrorStore) -> Self::Iterator {
        self.generate_with_client(errors, &reqwest::Client::new())
            .await
    }
}

#[cfg(test)]
mock! {
    pub(crate) Generator { }

    impl std::fmt::Display for Generator {
        fn fmt<'a>(&self, _fmt: &mut std::fmt::Formatter<'a>) -> Result<(), std::fmt::Error>;
    }

    impl Clone for Generator {
        fn clone(&self) -> Self;
    }

    impl PartialEq<MockGenerator> for Generator {
        fn eq(&self, other: &MockGenerator) -> bool;
    }

    impl Eq for Generator {
    }

    impl std::convert::TryFrom<Vec<String>> for Generator {
        type Error = ();
        fn try_from(_vec: Vec<String>) -> Result<Self, ()> {
            Err(())
        }
    }

    #[async_trait]
    impl Generator for Generator {
        type Item = MockVideo;
        type Iterator = std::vec::IntoIter<MockVideo>;
        async fn generate(&self, errors: &ErrorStore) -> <Self as Generator>::Iterator;
    }

    impl crate::Subscription for Generator {
        type Video = MockVideo;
        fn name(&self) -> Option<String>;
    }
}

#[cfg(test)]
impl std::hash::Hash for MockGenerator {
    fn hash<H>(&self, _fmt: &mut H)
    where
        H: std::hash::Hasher,
    {
    }
}

#[cfg(test)]
impl std::convert::From<MockGenerator> for Vec<String> {
    fn from(_sub: MockGenerator) -> Self {
        vec![]
    }
}
