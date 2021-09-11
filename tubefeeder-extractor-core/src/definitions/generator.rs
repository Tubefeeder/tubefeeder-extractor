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

#[cfg(test)]
mock! {
    pub(crate) Generator { }

    #[async_trait]
    impl Generator for Generator {
        type Item = MockVideo;
        type Iterator = std::vec::IntoIter<MockVideo>;
        async fn generate(&self, errors: &ErrorStore) -> <Self as Generator>::Iterator;
    }
}
