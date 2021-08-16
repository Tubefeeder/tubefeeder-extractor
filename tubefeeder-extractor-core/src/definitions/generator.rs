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

#[cfg(test)]
use {crate::mock::MockVideo, mockall::predicate::*, mockall::*};

#[async_trait]
pub trait Generator {
    type Item;
    type Iterator: Iterator<Item = Self::Item>;
    async fn generate(&self) -> (Self::Iterator, Option<crate::Error>);
}

#[cfg(test)]
mock! {
    pub(crate) Generator { }

    #[async_trait]
    impl Generator for Generator {
        type Item = MockVideo;
        type Iterator = std::vec::IntoIter<MockVideo>;
        async fn generate(&self) -> (<Self as Generator>::Iterator, Option<crate::Error>);
    }
}
