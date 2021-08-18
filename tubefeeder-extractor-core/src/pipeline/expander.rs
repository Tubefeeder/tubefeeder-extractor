use crate::{ExpandedVideo, Generator, Video};

use std::marker::PhantomData;

use async_trait::async_trait;

#[derive(Clone)]
pub struct Expander<V, G> {
    generator: G,
    video: PhantomData<V>,
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

    async fn generate(&self) -> (Self::Iterator, Option<crate::Error>) {
        let (iterator, error) = self.generator.generate().await;
        let mapped_iterator: Box<dyn Iterator<Item = ExpandedVideo<V>> + std::marker::Send> =
            Box::new(iterator.map(|v| v.into()))
                as Box<dyn Iterator<Item = <Self as Generator>::Item> + std::marker::Send>;
        (mapped_iterator, error)
    }
}

impl<V, G> Expander<V, G>
where
    G: Generator<Item = V> + std::marker::Sync + std::marker::Send,
    <G as Generator>::Iterator: 'static,
    V: Video,
{
    pub fn new(generator: G) -> Self {
        Expander {
            generator,
            video: PhantomData,
        }
    }
}
