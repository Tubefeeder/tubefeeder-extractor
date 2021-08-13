//! The pipeline of videos.
//! This currently contains
//!
//! - [`Merger`][merger::Merger]
//! - [`SubscriptionList`][subscription_list::SubscriptionList]

pub(crate) mod merger;
pub(crate) mod store_access;
pub mod subscription_list;
pub(crate) mod video_store;
