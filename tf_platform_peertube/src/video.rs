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

use crate::PTSubscription;
use tf_core::{Subscription, Video, DATE_FORMAT};
use tf_utils::rss::{FromItemAndSub, Item};

#[derive(Clone)]
pub struct PTVideo {
    pub(crate) url: String,
    pub(crate) title: String,
    pub(crate) uploaded: chrono::NaiveDateTime,
    pub(crate) subscription: PTSubscription,
    pub(crate) thumbnail_url: String,
}

impl std::hash::Hash for PTVideo {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.url.hash(state);
        self.title.hash(state);
        self.subscription.hash(state);
    }
}

impl std::cmp::PartialEq for PTVideo {
    fn eq(&self, other: &Self) -> bool {
        self.url == other.url
            && self.title == other.title
            && self.subscription == other.subscription
    }
}

impl std::cmp::Eq for PTVideo {}

impl PTVideo {
    pub fn new<T: AsRef<str>>(
        url: T,
        title: T,
        uploaded: chrono::NaiveDateTime,
        subscription: PTSubscription,
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

impl std::convert::TryFrom<Vec<String>> for PTVideo {
    type Error = ();

    fn try_from(strings: Vec<String>) -> Result<Self, Self::Error> {
        let url_opt = strings.get(0);
        let title = strings.get(1);
        let uploaded = strings.get(2);
        let sub_name = strings.get(3);
        let sub_id = strings.get(4);
        let sub_base_url = strings.get(5);
        let thumbnail_url = strings.get(6);
        match (
            url_opt,
            title,
            uploaded,
            sub_name,
            sub_id,
            sub_base_url,
            thumbnail_url,
        ) {
            (Some(url), Some(tit), Some(upl), Some(sub_n), Some(sub_i), Some(sub_u), Some(thu)) => {
                let upl_date = chrono::NaiveDateTime::parse_from_str(upl, DATE_FORMAT);
                if let Ok(upl) = upl_date {
                    let sub = PTSubscription::new_with_name(sub_u, sub_i, sub_n);
                    Ok(PTVideo::new(url, tit, upl, sub, thu))
                } else {
                    Err(())
                }
            }
            _ => Err(()),
        }
    }
}

impl From<PTVideo> for Vec<String> {
    fn from(video: PTVideo) -> Self {
        let mut result = vec![];
        result.push(video.url());
        result.push(video.title());
        result.push(video.uploaded().format(DATE_FORMAT).to_string());
        let sub = video.subscription();
        result.push(sub.name().unwrap_or_else(|| "".to_string()));
        result.push(sub.id());
        result.push(sub.base_url());
        result.push(video.thumbnail_url());
        result
    }
}

#[async_trait]
impl tf_core::Video for PTVideo {
    type Subscription = PTSubscription;

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

impl FromItemAndSub<PTSubscription> for PTVideo {
    fn from_item_and_sub(i: Item, sub: PTSubscription) -> Self {
        Self {
            title: i.media_title,
            url: i.link,
            uploaded: i.pub_date,
            subscription: sub,
            thumbnail_url: i
                .media_thumbnail
                .into_iter()
                .next()
                .map(|m| m.url)
                .unwrap_or_default(),
        }
    }
}
