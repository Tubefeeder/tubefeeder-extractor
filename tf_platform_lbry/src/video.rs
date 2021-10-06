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

use async_trait::async_trait;

use crate::LbrySubscription;
use tf_utils::rss::{FromItemAndSub, Item};

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct LbryVideo {
    pub(crate) url: String,
    pub(crate) title: String,
    pub(crate) uploaded: chrono::NaiveDateTime,
    pub(crate) subscription: LbrySubscription,
    pub(crate) thumbnail_url: String,
}

impl LbryVideo {
    pub fn new<T: AsRef<str>>(
        url: T,
        title: T,
        uploaded: chrono::NaiveDateTime,
        subscription: LbrySubscription,
        thumbnail_url: T,
    ) -> Self {
        Self {
            url: url.as_ref().to_owned(),
            title: title.as_ref().to_owned(),
            uploaded,
            subscription,
            thumbnail_url: thumbnail_url.as_ref().to_owned(),
        }
    }
}

#[async_trait]
impl tf_core::Video for LbryVideo {
    type Subscription = LbrySubscription;

    fn url(&self) -> String {
        self.url.clone()
    }

    fn title(&self) -> String {
        self.title.clone()
    }

    fn uploaded(&self) -> chrono::NaiveDateTime {
        self.uploaded.clone()
    }

    fn subscription(&self) -> Self::Subscription {
        self.subscription.clone()
    }

    fn thumbnail_url(&self) -> String {
        self.thumbnail_url.clone()
    }
}

impl FromItemAndSub<LbrySubscription> for LbryVideo {
    fn from_item_and_sub(i: Item, sub: LbrySubscription) -> Self {
        Self {
            title: i.itunes_title,
            url: i.link,
            uploaded: i.pub_date,
            subscription: sub,
            thumbnail_url: i.itunes_image.href,
        }
    }
}
