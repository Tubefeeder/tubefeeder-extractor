// Usefull for StoreAccess in the future.
// #![feature(type_alias_impl_trait)]

pub mod error;
pub mod observer;
pub mod pipeline;
pub mod traits;

pub use error::{Error, NetworkError, ParseError};
pub use observer::{Observable, Observer, ObserverList};
pub use pipeline::pipe::Pipeline;
pub use pipeline::subscription_list::SubscriptionList;
pub use traits::generator::Generator;
pub use traits::subscription::Subscription;
pub use traits::video::Video;

use pipeline::merger::Merger;
use pipeline::store_access::StoreAccess;
use pipeline::video_store::VideoStore;

#[cfg(test)]
mod mock {
    pub(crate) use crate::traits::generator::MockGenerator;
    pub(crate) use crate::traits::subscription::MockSubscription;
    pub(crate) use crate::traits::video::MockVideo;
}
