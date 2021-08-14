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

use crate::structure::*;
use crate::subscription::YTSubscription;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct YTVideo {
    pub(crate) url: String,
    pub(crate) title: String,
    pub(crate) uploaded: chrono::NaiveDateTime,
    pub(crate) subscription: YTSubscription,
}

impl tf_core::Video for YTVideo {
    type Subscription = YTSubscription;

    fn url(&self) -> String {
        self.url.clone()
    }

    fn title(&self) -> String {
        self.title.clone()
    }

    fn subscription(&self) -> Self::Subscription {
        self.subscription.clone()
    }

    fn uploaded(&self) -> chrono::NaiveDateTime {
        self.uploaded
    }
}

impl From<Feed> for Vec<YTVideo> {
    fn from(feed: Feed) -> Self {
        feed.entries.into_iter().map(|e| e.into()).collect()
    }
}

impl From<Entry> for YTVideo {
    fn from(e: Entry) -> Self {
        let subscription = YTSubscription::new_with_name(
            e.author.uri.split('/').last().unwrap_or(""),
            &e.author.name,
        );

        YTVideo {
            url: e.link.href.to_string(),
            title: e.title,
            subscription,
            uploaded: e.published,
        }
    }
}
