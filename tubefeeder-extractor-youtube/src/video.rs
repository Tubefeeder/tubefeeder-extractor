use crate::subscription::YTSubscription;

use tf_core::Video;

use async_trait::async_trait;
use chrono::NaiveDateTime;
use rusty_pipe::extractors::{YTStreamExtractor, YTStreamInfoItemExtractor};

#[derive(Clone, Debug)]
pub struct YTVideo {
    url: String,
    title: String,
    subscription: YTSubscription,
    uploaded: NaiveDateTime,
}

impl YTVideo {
    pub(crate) async fn from_extractor(
        subscription: YTSubscription,
        extractor: YTStreamInfoItemExtractor,
    ) -> Result<Self, tf_core::Error> {
        let url = extractor
            .url()
            .map_err(|e| tf_core::Error::from(tf_core::ParseError(format!("{}", e))))?;
        let title = extractor
            .name()
            .map_err(|e| tf_core::Error::from(tf_core::ParseError(format!("{}", e))))?;

        let uploaded = if let Ok(id) = extractor.video_id() {
            if let Ok(stream_extractor) = YTStreamExtractor::new(&id, crate::Downloader {}).await {
                stream_extractor
                    .upload_date()
                    .map(|d| d.and_hms(0, 0, 0))
                    .map_err(|e| tf_core::Error::from(tf_core::ParseError(format!("{}", e))))
                    .unwrap_or(NaiveDateTime::from_timestamp(0, 0))
            } else {
                NaiveDateTime::from_timestamp(0, 0)
            }
        } else {
            NaiveDateTime::from_timestamp(0, 0)
        };

        Ok(YTVideo {
            url,
            title,
            subscription,
            uploaded,
        })
    }
}

#[async_trait]
impl Video for YTVideo {
    type Subscription = YTSubscription;
    type Rating = ();
    type Thumbnail = ();

    async fn url(&self) -> String {
        self.url.clone()
    }
    async fn title(&self) -> String {
        self.title.clone()
    }
    async fn subscription(&self) -> Self::Subscription {
        self.subscription.clone()
    }
    async fn uploaded(&self) -> chrono::NaiveDateTime {
        self.uploaded.clone()
    }
    async fn rating(&self) -> Self::Rating {}
    async fn thumbnail(&self) -> Self::Thumbnail {}
}
