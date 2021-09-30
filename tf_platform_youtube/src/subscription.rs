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
use futures::StreamExt;
use invidious::Invidious;
use tf_core::{Error, ErrorStore, GeneratorWithClient, NetworkError, ParseError, Subscription};

const INVIDIOUS_URL: &str = "https://y.com.cm/";

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

    /// Try to interpret the given string as a id first, if this fails try
    /// to interpret it as a name.
    pub async fn from_id_or_name(id_or_name: &str) -> Result<Self, Error> {
        let invidious = Invidious::new(INVIDIOUS_URL, reqwest::Client::new());
        let extractor = invidious.channel_client(id_or_name).await;
        if extractor.is_ok() {
            Ok(Self::new(id_or_name))
        } else {
            Self::from_name(id_or_name).await
        }
    }

    /// Try to create a new [`YTSubscription`] from the given name.
    ///
    /// Will return `None` if no such channel exists.
    pub async fn from_name(name: &str) -> Result<Self, Error> {
        let url = format!("https://www.youtube.com/c/{}/featured", name);
        let content: Result<String, Error> = async {
            let response = reqwest::get(&url).await;

            if response.is_err() {
                return Err(NetworkError(url).into());
            }

            let parsed = response.unwrap().text().await;

            if parsed.is_err() {
                return Err(NetworkError(url).into());
            }

            Ok(parsed.unwrap())
        }
        .await;

        if let Err(e) = content {
            Err(e)
        } else {
            let regex = regex::Regex::new(r#""externalId":"([0-9a-zA-Z_\-]*)"#).unwrap();

            if let Some(id) = regex.captures(&content.unwrap()) {
                Ok(Self::new_with_name(&id[1].to_string(), name))
            } else {
                Err(ParseError(name.to_string()).into())
            }
        }
    }

    /// Get the channel id of the [`YTSubscription`].
    pub fn id(&self) -> String {
        self.id.clone()
    }

    /// Try to get the channel name from the channel id.
    pub async fn update_name(&self, client: &reqwest::Client) -> Option<String> {
        let invidious = Invidious::new(INVIDIOUS_URL, client.clone());
        let client_res = invidious.channel_client(&self.id).await;
        if let Ok(client) = client_res {
            Some(client.channel().author)
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

        let invidious = Invidious::new(INVIDIOUS_URL, client.clone());

        let client_res = invidious.channel_client(&self.id).await.ok();

        if client_res.is_none() {
            // TODO: Url?
            errors.add(NetworkError("Youtube".to_string()).into());
            return vec![].into_iter();
        }

        let client = client_res.unwrap();

        let videos_stream = Box::pin(client.videos()).take(15);

        let name = client.channel().author;

        videos_stream
            .map(|v| YTVideo::from_extractor(errors, v, self.with_name(&name)))
            .collect::<Vec<YTVideo>>()
            .await
            .into_iter()
    }
}
