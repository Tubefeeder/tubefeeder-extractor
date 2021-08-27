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

use std::path::Path;

use crate::Subscription;

use async_trait::async_trait;
use gdk_pixbuf::{Colorspace, Pixbuf};

#[cfg(test)]
use {crate::mock::MockSubscription, mockall::predicate::*, mockall::*};

/// A [`Video`] that can come from any website.
#[async_trait]
pub trait Video:
    Clone + std::hash::Hash + std::cmp::Eq + std::marker::Send + std::marker::Sync
{
    type Subscription: Subscription;

    fn url(&self) -> String;
    fn title(&self) -> String;
    fn uploaded(&self) -> chrono::NaiveDateTime;
    fn subscription(&self) -> Self::Subscription;

    async fn thumbnail_with_client<P: AsRef<Path> + Send>(
        &self,
        _client: &reqwest::Client,
        filename: P,
        width: i32,
        height: i32,
    ) {
        self.default_thumbnail(filename, width, height);
    }

    fn default_thumbnail<P: AsRef<Path>>(&self, filename: P, width: i32, height: i32) {
        let pixbuf =
            Pixbuf::new(Colorspace::Rgb, true, 8, width, height).expect("Could not create empty");
        pixbuf.fill(0);
        let _ = pixbuf.savev(filename, "png", &[]);
    }

    async fn thumbnail<P: AsRef<Path> + Send>(&self, filename: P, width: i32, height: i32) {
        self.thumbnail_with_client(&reqwest::Client::new(), filename, width, height)
            .await
    }
}

#[cfg(test)]
mock! {
    pub(crate) Video {}

    impl Clone for Video {
        fn clone(&self) -> Self;
    }

    impl Video for Video {
        type Subscription = MockSubscription;

        fn url(&self) -> String;
        fn title(&self) -> String;
        fn uploaded(&self) -> chrono::NaiveDateTime;
        fn subscription(&self) -> MockSubscription;
    }
}

#[cfg(test)]
impl std::hash::Hash for MockVideo {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.uploaded().hash(state);
    }
}

#[cfg(test)]
impl PartialEq for MockVideo {
    fn eq(&self, other: &Self) -> bool {
        self.uploaded().eq(&other.uploaded())
    }
}
#[cfg(test)]
impl Eq for MockVideo {}
