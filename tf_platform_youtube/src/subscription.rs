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

use crate::video::YTVideo;

use async_trait::async_trait;
use piped::{ChannelSearchItem, PipedClient};
use tf_core::{ErrorStore, GeneratorWithClient, Subscription};

const PIPED_API_URL: &'static str = "https://pipedapi.kavin.rocks";

fn piped_api_url() -> String {
    match std::env::var("PIPED_API_URL") {
        Ok(url) => url,
        Err(_e) => PIPED_API_URL.to_string(),
    }
}

/// A [`YTSubscription`] to a YouTube-Channel. The Youtube-Channel is referenced by the channel id.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct YTSubscription {
    /// The channel id.
    id: String,
    name: Option<String>,
}

impl YTSubscription {
    /// Create a new [`YTSubscription`] using the given channel id.
    pub fn new(id: &str) -> Self {
        YTSubscription {
            id: id.to_owned(),
            name: None,
        }
    }

    /// Create a new [`YTSubscription`] using the given channel id and name.
    pub fn new_with_name(id: &str, name: &str) -> Self {
        YTSubscription {
            id: id.to_owned(),
            name: Some(name.to_owned()),
        }
    }

    /// Try to get a subscription using youtube search.
    /// This will try to search the given query in youtube filtered only to channels
    /// and return the first result if it exists.
    pub async fn try_from_search<S: AsRef<str>>(query: S) -> Option<Self> {
        log::debug!("Getting channel from query {}", query.as_ref());
        let piped = PipedClient::new(&reqwest::Client::new(), piped_api_url());
        let result = piped.search_channel(query).await;
        if let Ok(channel_search) = result {
            log::debug!(
                "Got back a result with {} items",
                channel_search.items.len()
            );
            channel_search.items.get(0).map(|i| i.into())
        } else {
            log::error!("Got back a error: {}", result.err().unwrap());
            None
        }
    }

    /// Get the channel id of the [`YTSubscription`].
    pub fn id(&self) -> String {
        self.id.clone()
    }

    /// Try to get the channel name from the channel id.
    pub async fn update_name(&self, client: &reqwest::Client) -> Option<String> {
        let piped = PipedClient::new(&client, piped_api_url());
        if let Ok(channel) = piped.channel_from_id(&self.id).await {
            Some(channel.name)
        } else {
            None
        }
    }

    fn with_name(&self, name: &str) -> Self {
        Self {
            id: self.id.clone(),
            name: Some(name.to_string()),
        }
    }
}

impl Subscription for YTSubscription {
    type Video = YTVideo;

    /// Get the name of the [`YTSubscription`].
    fn name(&self) -> Option<String> {
        self.name.clone()
    }
}

impl std::fmt::Display for YTSubscription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name().unwrap_or_else(|| self.id()))
    }
}

impl std::convert::TryFrom<Vec<String>> for YTSubscription {
    type Error = ();

    fn try_from(strings: Vec<String>) -> Result<Self, Self::Error> {
        if let Some(value) = strings.get(0) {
            Ok(YTSubscription::new(value))
        } else {
            Err(())
        }
    }
}

impl From<YTSubscription> for Vec<String> {
    fn from(sub: YTSubscription) -> Self {
        vec![sub.id]
    }
}

#[async_trait]
impl GeneratorWithClient for YTSubscription {
    type Item = YTVideo;
    type Iterator = std::vec::IntoIter<Self::Item>;

    async fn generate_with_client(
        &self,
        errors: &ErrorStore,
        client: &reqwest::Client,
    ) -> Self::Iterator {
        log::debug!(
            "Generating YT videos from channel {}",
            self.name().unwrap_or_else(|| self.id())
        );

        let piped = PipedClient::new(client, piped_api_url());
        let channel_res = piped.channel_from_id(self.id.clone()).await;

        if let Err(e) = &channel_res {
            log::error!("Error generating youtube videos from subscription {}", self);
            errors.add(piped_to_tubefeeder_error(e));
            return vec![].into_iter();
        }

        let channel = channel_res.unwrap();
        let videos = channel.related_streams;
        let name = channel.name;

        Box::new(
            videos
                .into_iter()
                .map(|v| YTVideo::from_related_stream(errors, v, self.with_name(&name))),
        )
        .collect::<Vec<YTVideo>>()
        .into_iter()
    }
}

impl From<&ChannelSearchItem> for YTSubscription {
    fn from(item: &ChannelSearchItem) -> Self {
        Self {
            id: item.url.split('/').last().unwrap_or_default().to_string(),
            name: Some(item.name.to_string()),
        }
    }
}

fn piped_to_tubefeeder_error(error: &piped::Error) -> tf_core::Error {
    match error {
        piped::Error::Network(e) => tf_core::NetworkError(e.to_string()).into(),
        piped::Error::ParseResponse(e) => tf_core::ParseError(e.to_string()).into(),
        piped::Error::Parseurl(e) => tf_core::ParseError(e.to_string()).into(),
    }
}
