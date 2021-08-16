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

fn feed_url() -> String {
    #[cfg(not(test))]
    let url = "https://www.youtube.com/feeds/videos.xml?channel_id=".to_owned();
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

    pub fn new_with_name(id: &str, name: &str) -> Self {
        YTSubscription {
            id: id.to_owned(),
            name: Some(name.to_owned()),
        }
    }

    /// Get the channel id of the [`YTSubscription`].
    pub fn id(&self) -> String {
        self.id.clone()
    }

    /// Get the name id of the [`YTSubscription`].
    pub fn name(&self) -> Option<String> {
        self.name.clone()
    }
}

#[async_trait]
impl tf_core::Subscription for YTSubscription {
    type Video = YTVideo;
    type Iterator = std::vec::IntoIter<Self::Video>;
    async fn generate_with_client(
        &self,
        client: &reqwest::Client,
    ) -> (Self::Iterator, Option<tf_core::Error>) {
        log::debug!(
            "Generating YT videos from channel {}",
            self.name().unwrap_or(self.id())
        );
        let result = client
            .get(format!("{}{}", feed_url(), self.id()))
            .send()
            .await;
        if let Err(e) = result {
            return (vec![].into_iter(), Some(e.into()));
        }

        let body = result.unwrap().text().await;
        if let Err(e) = body {
            return (vec![].into_iter(), Some(e.into()));
        }

        // Replace all occurrences of `media:` with `media_` as serde does not seem to like `:`.
        let body_parsable = body.unwrap().replace("media:", "media_");

        let parsed = quick_xml::de::from_str::<Feed>(&body_parsable);
        if let Err(_e) = parsed {
            return (
                vec![].into_iter(),
                Some(tf_core::ParseError(format!("channel {}", self.id())).into()),
            );
        }

        log::debug!(
            "Finished Generating YT videos from channel {}",
            self.name().unwrap_or(self.id())
        );

        (Vec::<YTVideo>::from(parsed.unwrap()).into_iter(), None)
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use mockito::{mock, Matcher};
    use tf_core::Subscription;

    fn expected_videos() -> Vec<YTVideo> {
        let subscription = YTSubscription::new_with_name("ThisIsAChannelId", "ChannelName");
        let video1 = YTVideo {
            url: "https://www.youtube.com/watch?v=videoid1".to_string(),
            title: "VIDEO 1 !! Click".to_string(),
            subscription: subscription.clone(),
            uploaded: chrono::NaiveDate::from_ymd(2021, 7, 19).and_hms(16, 18, 6),
        };
        let video2 = YTVideo {
            url: "https://www.youtube.com/watch?v=videoid2".to_string(),
            title: "VIDEO 2 !! Click".to_string(),
            subscription,
            uploaded: chrono::NaiveDate::from_ymd(2021, 7, 29).and_hms(16, 18, 6),
        };

        vec![video1, video2]
    }

    #[tokio::test]
    async fn youtube_generator() {
        let _m = mock("GET", Matcher::Regex(r"^/youtube/".to_string()))
            .with_status(200)
            .with_body(include_str!("../resources/test/youtubefeed.xml"))
            .create();

        let videos = YTSubscription::new("ThisIsAChannelId").generate().await.0;

        assert_eq!(videos.collect::<Vec<_>>(), expected_videos());
    }
}
