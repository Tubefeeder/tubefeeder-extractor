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

// Usefull for StoreAccess in the future.
// #![feature(type_alias_impl_trait)]

mod definitions;
mod error;
mod observer;
mod pipeline;

pub use definitions::expanded_video::ExpandedVideo;
pub use definitions::expanded_video::VideoEvent;
pub use definitions::generator::Generator;
pub use definitions::subscription::Subscription;
pub use definitions::video::Video;
pub use error::{Error, ErrorEvent, ErrorStore, NetworkError, ParseError};
pub use observer::{Observable, Observer, ObserverList};
pub use pipeline::expander::Expander;
pub use pipeline::pipe::Pipeline;
pub use pipeline::subscription_list::SubscriptionList;

use pipeline::merger::Merger;
use pipeline::store_access::StoreAccess;
use pipeline::video_store::VideoStore;

#[cfg(test)]
mod mock {
    pub(crate) use crate::definitions::generator::MockGenerator;
    pub(crate) use crate::definitions::subscription::MockSubscription;
    pub(crate) use crate::definitions::video::MockVideo;
}

pub mod prelude {
    pub use crate::{Generator, Subscription, Video};
}
