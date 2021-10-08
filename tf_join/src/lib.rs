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

//! Joining multiple platforms into one.
//!
//! ### Capabilities
//!
//! - Join multiple platforms together using [Joiner].
//! - Filter out videos using [AnyVideoFilter].
//! - Generalization of [Video][tf_core::Video] and [Subscription][tf_core::Subscription] using
//! [AnyVideo] and [AnySubscription].
//! - Generalization of [SubscriptionList][tf_core::SubscriptionList] using [AnySubscriptionList].
//!
//!
//! ### Features
//!
//! The feature enable and disable specific platforms. All features are activated
//! by default. The possible features currently are:
//!
//! - `youtube`
//! - `peertube`
//! - `lbry`

mod filter;
mod joiner;
mod subscription;
mod subscription_list;
mod video;

pub use crate::filter::AnyVideoFilter;
pub use crate::joiner::Joiner;
pub use crate::subscription::AnySubscription;
pub use crate::subscription::Platform;
pub use crate::subscription_list::AnySubscriptionList;
pub use crate::subscription_list::SubscriptionEvent;
pub use crate::video::AnyVideo;
