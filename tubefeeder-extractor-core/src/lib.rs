// Usefull for StoreAccess
// #![feature(type_alias_impl_trait)]

pub mod error;
pub mod observer;
pub mod pipeline;
pub mod traits;

pub use error::{Error, NetworkError, ParseError};
pub use observer::{Observable, Observer, ObserverList};
pub use pipeline::subscription_list::SubscriptionList;
pub use traits::subscription::Subscription;
pub use traits::video::Video;

use pipeline::video_store::VideoStore;
use traits::generator::Generator;
