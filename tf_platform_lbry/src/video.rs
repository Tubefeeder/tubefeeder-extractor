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
use tf_core::{Subscription, Video, DATE_FORMAT};

use crate::LbrySubscription;
use tf_utils::rss::{FromItemAndSub, Item};

#[derive(Clone)]
pub struct LbryVideo {
    pub(crate) url: String,
    pub(crate) title: String,
    pub(crate) uploaded: chrono::NaiveDateTime,
    pub(crate) subscription: LbrySubscription,
    pub(crate) thumbnail_url: String,
}

impl std::hash::Hash for LbryVideo {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.url.hash(state);
        self.title.hash(state);
        self.subscription.hash(state);
    }
}

impl std::cmp::PartialEq for LbryVideo {
    fn eq(&self, other: &Self) -> bool {
        self.url == other.url
            && self.title == other.title
            && self.subscription == other.subscription
    }
}

impl std::cmp::Eq for LbryVideo {}

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

impl std::convert::TryFrom<Vec<String>> for LbryVideo {
    type Error = ();

    fn try_from(strings: Vec<String>) -> Result<Self, Self::Error> {
        let url_opt = strings.get(0);
        let title = strings.get(1);
        let uploaded = strings.get(2);
        let sub_name = strings.get(3);
        let sub_id = strings.get(4);
        let thumbnail_url = strings.get(5);
        match (url_opt, title, uploaded, sub_name, sub_id, thumbnail_url) {
            (Some(url), Some(tit), Some(upl), Some(sub_n), Some(sub_i), Some(thu)) => {
                let upl_date = chrono::NaiveDateTime::parse_from_str(upl, DATE_FORMAT);
                if let Ok(upl) = upl_date {
                    let sub = LbrySubscription::new_with_name(sub_i, sub_n);
                    Ok(LbryVideo::new(url, tit, upl, sub, thu))
                } else {
                    Err(())
                }
            }
            _ => Err(()),
        }
    }
}

impl From<LbryVideo> for Vec<String> {
    fn from(video: LbryVideo) -> Self {
        let mut result = vec![];
        result.push(video.url());
        result.push(video.title());
        result.push(video.uploaded().format(DATE_FORMAT).to_string());
        let sub = video.subscription();
        result.push(sub.name().unwrap_or_else(|| "".to_string()));
        result.push(sub.id());
        result.push(video.thumbnail_url());
        result
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
