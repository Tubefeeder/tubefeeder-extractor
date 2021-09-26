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

use crate::{ErrorStore, ExpandedVideo, Generator, Video};

use std::marker::PhantomData;

use async_trait::async_trait;

/// A [Pipeline][crate::Pipeline]-component expanding [Video]s `V` into
/// [ExpandedVideo]s.
#[derive(Clone)]
pub(crate) struct Expander<V, G> {
    /// The internal [Generator].
    generator: G,

    /// Phantom data.
    phantom: PhantomData<V>,
}

impl<V, G> Expander<V, G>
where
    G: Generator<Item = V> + std::marker::Sync + std::marker::Send,
    <G as Generator>::Iterator: 'static,
    V: Video,
{
    /// Create a new [Expander] wrapping the given [Generator].
    pub fn new(generator: G) -> Self {
        Expander {
            generator,
            phantom: PhantomData,
        }
    }
}

#[async_trait]
impl<V, G> Generator for Expander<V, G>
where
    G: Generator<Item = V> + std::marker::Sync + std::marker::Send,
    <G as Generator>::Iterator: 'static + std::marker::Send,
    V: Video,
{
    type Item = ExpandedVideo<V>;

    type Iterator = Box<dyn Iterator<Item = <Self as Generator>::Item> + std::marker::Send>;

    async fn generate(&self, errors: &ErrorStore) -> Self::Iterator {
        let iterator = self.generator.generate(errors).await;
        let mapped_iterator: Box<dyn Iterator<Item = ExpandedVideo<V>> + std::marker::Send> =
            Box::new(iterator.map(|v| v.into()))
                as Box<dyn Iterator<Item = <Self as Generator>::Item> + std::marker::Send>;
        mapped_iterator
    }
}
