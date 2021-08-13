//! The pipeline of videos.
//! This currently contains
//!
//! - [`Pipeline`][pipe::Pipeline]
//! - [`SubscriptionList`][subscription_list::SubscriptionList]

pub(crate) mod merger;
pub mod pipe;
pub(crate) mod store_access;
pub mod subscription_list;
pub(crate) mod video_store;
