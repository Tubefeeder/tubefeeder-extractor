//! The pipeline of videos.
//! This currently contains
//!
//! - [`Merger`][merger::Merger]
//! - [`Pipeline`][pipeline::Pipeline]
//! - [`StoreAccess`][store_access::StoreAccess]
//! - [`SubscriptionList`][subscription_list::SubscriptionList]
//! - [`VideoStore`][video_store::VideoStore]

pub(crate) mod merger;
pub mod pipe;
pub(crate) mod store_access;
pub mod subscription_list;
pub(crate) mod video_store;
