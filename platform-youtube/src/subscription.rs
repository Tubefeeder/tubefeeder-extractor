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

use crate::{structure::Feed, video::YTVideo};

use async_trait::async_trait;
use rusty_pipe::extractors::YTChannelExtractor;
use tf_core::{Error, ErrorStore, GeneratorWithClient, NetworkError, ParseError, Subscription};

fn feed_url() -> String {
    #[cfg(not(test))]
    let url = std::env::var("YOUTUBE_BASE_URL").unwrap_or("https://www.youtube.com".to_owned())
        + "/feeds/videos.xml?channel_id=";
    #[cfg(test)]
    let url = format!("{}/{}/", mockito::server_url(), "youtube");

    url
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

    /// Try to interpret the given string as a id first, if this fails try
    /// to interpret it as a name.
    pub async fn from_id_or_name(id_or_name: &str) -> Result<Self, Error> {
        let extractor = YTChannelExtractor::new::<crate::Downloader>(id_or_name, None).await;
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
        let error_store = ErrorStore::new();
        let mut videos = self.generate_with_client(&error_store, client).await;
        if let Some(video) = videos.next() {
            return video.subscription.name();
        } else {
            return None;
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

        let url = format!("{}{}", feed_url(), self.id());
        let result = client.get(&url).send().await;
        if let Err(_e) = result {
            errors.add(NetworkError(url).into());
            return vec![].into_iter();
        }

        if !result.as_ref().unwrap().status().is_success() {
            errors.add(NetworkError(url).into());
            return vec![].into_iter();
        }

        let body = result.unwrap().text().await;
        if let Err(_e) = body {
            errors.add(NetworkError(url).into());
            return vec![].into_iter();
        }

        // Replace all occurrences of `media:` with `media_` as serde does not seem to like `:`.
        let body_parsable = body.unwrap().replace("media:", "media_");

        let parsed = quick_xml::de::from_str::<Feed>(&body_parsable);
        if let Err(_e) = parsed {
            errors.add(tf_core::ParseError(format!("channel {}", self.id())).into());
            return vec![].into_iter();
        }

        log::debug!(
            "Finished Generating YT videos from channel {}",
            self.name().unwrap_or_else(|| self.id())
        );

        Vec::<YTVideo>::from(parsed.unwrap()).into_iter()
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use mockito::{mock, Matcher};
    use tf_core::Generator;

    fn expected_videos() -> Vec<YTVideo> {
        let subscription = YTSubscription::new_with_name("ThisIsAChannelId", "ChannelName");
        let video1 = YTVideo {
            url: "https://www.youtube.com/watch?v=videoid1".to_string(),
            title: "VIDEO 1 !! Click".to_string(),
            subscription: subscription.clone(),
            uploaded: chrono::NaiveDate::from_ymd(2021, 7, 19).and_hms(16, 18, 6),
            thumbnail_url: "https://i4.ytimg.com/vi/videoid1/hqdefault.jpg".to_owned(),
        };
        let video2 = YTVideo {
            url: "https://www.youtube.com/watch?v=videoid2".to_string(),
            title: "VIDEO 2 !! Click".to_string(),
            subscription,
            uploaded: chrono::NaiveDate::from_ymd(2021, 7, 29).and_hms(16, 18, 6),
            thumbnail_url: "https://i4.ytimg.com/vi/videoid2/hqdefault.jpg".to_owned(),
        };

        vec![video1, video2]
    }

    #[tokio::test]
    async fn youtube_generator() {
        let _m = mock("GET", Matcher::Regex(r"^/youtube/".to_string()))
            .with_status(200)
            .with_body(include_str!("../resources/test/youtubefeed.xml"))
            .create();

        let errors = ErrorStore::new();

        let videos = YTSubscription::new("ThisIsAChannelId")
            .generate(&errors)
            .await;

        assert_eq!(videos.collect::<Vec<_>>(), expected_videos());
    }

    #[tokio::test]
    async fn youtube_generator_parse_error() {
        let _m = mock("GET", Matcher::Regex(r"^/youtube/".to_string()))
            .with_status(200)
            .with_body(include_str!("../resources/test/youtubefeed_invalid.xml"))
            .create();

        let errors = ErrorStore::new();

        let videos = YTSubscription::new("ThisIsAChannelId")
            .generate(&errors)
            .await;

        assert_eq!(videos.count(), 0);
        assert_eq!(errors.summary().parse(), 1);
        assert_eq!(errors.summary().network(), 0);
    }

    #[tokio::test]
    async fn youtube_generator_network_error() {
        let _m = mock("GET", Matcher::Regex(r"$a".to_string())).create();

        let errors = ErrorStore::new();

        let videos = YTSubscription::new("ThisIsAChannelId")
            .generate(&errors)
            .await;

        assert_eq!(videos.count(), 0);
        assert_eq!(errors.summary().parse(), 0);
        assert_eq!(errors.summary().network(), 1);
    }
}
