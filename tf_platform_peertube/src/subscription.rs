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

use tf_core::{ErrorStore, GeneratorWithClient, Video};
use tf_utils::rss::{RssExtractor, RssExtractorWrapper, WithName};

use crate::PTVideo;

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct PTSubscription {
    id: String,
    base_url: String,
    name: Option<String>,
}

impl PTSubscription {
    /// Create a new peertube subscription. The base url should be the url peertube is accessible at.
    /// The id should be in the format name@url (you will get that when copying the video channel id).
    pub fn new<S1: AsRef<str>, S2: AsRef<str>>(base_url: S1, id: S2) -> Self {
        // Format url to always have http(s) in the beginning and no ending /
        let mut url = base_url.as_ref().to_owned();
        if !url.starts_with("http") {
            url = format!("https://{}", url);
        }
        if url.ends_with('/') {
            url.pop();
        }
        Self {
            id: id.as_ref().to_owned(),
            base_url: url,
            name: None,
        }
    }

    pub fn new_with_name<S1: AsRef<str>, S2: AsRef<str>, S3: AsRef<str>>(
        base_url: S1,
        id: S2,
        name: S3,
    ) -> Self {
        Self {
            id: id.as_ref().to_owned(),
            base_url: base_url.as_ref().to_owned(),
            name: Some(name.as_ref().to_owned()),
        }
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn base_url(&self) -> String {
        self.base_url.clone()
    }

    /// Try to get the channel name from the channel.
    pub async fn update_name(&self, client: &reqwest::Client) -> Option<String> {
        let errors = ErrorStore::new();
        let video_res = self.generate_with_client(&errors, client).await.next();
        if let Some(video) = video_res {
            video.subscription().name
        } else {
            None
        }
    }
}

impl std::convert::TryFrom<Vec<String>> for PTSubscription {
    type Error = ();

    fn try_from(strings: Vec<String>) -> Result<Self, Self::Error> {
        if let (Some(id), Some(base_url)) = (strings.get(0), strings.get(1)) {
            Ok(PTSubscription::new(base_url, id))
        } else {
            Err(())
        }
    }
}

impl From<PTSubscription> for Vec<String> {
    fn from(sub: PTSubscription) -> Self {
        vec![sub.id, sub.base_url]
    }
}

impl WithName for PTSubscription {
    fn with_name<S: AsRef<str>>(&self, name: S) -> Self {
        Self {
            id: self.id.clone(),
            base_url: self.base_url.clone(),
            name: Some(name.as_ref().to_owned()),
        }
    }
}

impl RssExtractor for PTSubscription {
    fn feed_url(&self) -> String {
        format!(
            "{}/feeds/videos.xml?videoChannelName={}",
            self.base_url, self.id
        )
    }
}

impl std::fmt::Display for PTSubscription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name.as_ref().unwrap_or(&self.id))
    }
}

impl tf_core::Subscription for PTSubscription {
    type Video = PTVideo;

    fn name(&self) -> Option<String> {
        self.name.clone()
    }
}

#[async_trait::async_trait]
impl GeneratorWithClient for PTSubscription {
    type Item = PTVideo;

    type Iterator = std::vec::IntoIter<PTVideo>;

    async fn generate_with_client(
        &self,
        errors: &tf_core::ErrorStore,
        client: &reqwest::Client,
    ) -> Self::Iterator {
        RssExtractorWrapper::<Self>::from(self)
            .generate_with_client(errors, client)
            .await
    }
}
