use crate::error::Error;

use async_trait::async_trait;

/// A [`Generator`] generates [`Generator::Item`] asynchronously.
#[async_trait]
pub trait Generator {
    type Item;
    /// Generate the [`Generator::Item`] asynchronously.
    /// If any errors occur, it will be returned as a [`Error`].
    async fn generate(&self) -> Result<Vec<Self::Item>, Error>;
}
