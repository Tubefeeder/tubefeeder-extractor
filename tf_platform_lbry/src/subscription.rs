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

use crate::LbryVideo;

#[derive(Clone, Hash, Eq, Debug)]
pub struct LbrySubscription {
    id: String,
    name: Option<String>,
}

impl std::cmp::PartialEq for LbrySubscription {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl LbrySubscription {
    /// Create a new lbry subscription.
    /// The id should be in the format @name:number.
    pub fn new<S: AsRef<str>>(id: S) -> Self {
        Self {
            id: id.as_ref().to_owned(),
            name: None,
        }
    }

    pub fn new_with_name<S1: AsRef<str>, S2: AsRef<str>>(id: S1, name: S2) -> Self {
        Self {
            id: id.as_ref().to_owned(),
            name: Some(name.as_ref().to_owned()),
        }
    }

    pub fn id(&self) -> String {
        self.id.clone()
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

impl std::convert::TryFrom<Vec<String>> for LbrySubscription {
    type Error = ();

    fn try_from(strings: Vec<String>) -> Result<Self, Self::Error> {
        if let Some(value) = strings.get(0) {
            Ok(LbrySubscription::new(value))
        } else {
            Err(())
        }
    }
}

impl From<LbrySubscription> for Vec<String> {
    fn from(sub: LbrySubscription) -> Self {
        vec![sub.id]
    }
}

impl WithName for LbrySubscription {
    fn with_name<S: AsRef<str>>(&self, name: S) -> Self {
        Self {
            id: self.id.clone(),
            name: Some(name.as_ref().to_owned()),
        }
    }
}

impl RssExtractor for LbrySubscription {
    fn feed_url(&self) -> String {
        format!("https://odysee.com/$/rss/{}", self.id)
    }
}

impl std::fmt::Display for LbrySubscription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name.as_ref().unwrap_or(&self.id))
    }
}

impl tf_core::Subscription for LbrySubscription {
    type Video = LbryVideo;

    fn name(&self) -> Option<String> {
        self.name.clone()
    }
}

#[async_trait::async_trait]
impl GeneratorWithClient for LbrySubscription {
    type Item = LbryVideo;

    type Iterator = std::vec::IntoIter<LbryVideo>;

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
