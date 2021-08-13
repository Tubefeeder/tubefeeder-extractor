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
