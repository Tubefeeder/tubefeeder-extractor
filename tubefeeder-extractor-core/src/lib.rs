pub mod error;
pub mod observer;
pub mod traits;

pub use error::{Error, NetworkError, ParseError};
pub use observer::{Observable, Observer, ObserverList};
pub use traits::generator::Generator;
pub use traits::subscription::Subscription;

use async_trait::async_trait;

pub struct AnyVideo {}

/// A [`Video`] that can come from any website.
#[async_trait]
pub trait Video {
    type Subscription;
    type Rating;
    type UploadTime;
    type Thumbnail;

    async fn url(&self) -> String;
    async fn title(&self) -> String;
    async fn subscription(&self) -> Self::Subscription;
    async fn uploaded(&self) -> Self::UploadTime;
    async fn rating(&self) -> Self::Rating;
    async fn thumbnail(&self) -> Self::Thumbnail;
}
