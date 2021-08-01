use crate::subscription::Subscription;

use async_trait::async_trait;
use rusty_pipe::extractors::YTStreamInfoItemExtractor;

#[derive(Clone, Debug)]
pub struct Video {
    url: String,
    title: String,
    subscription: Subscription,
    uploaded: String,
}

impl Video {
    pub(crate) fn from_extractor(
        subscription: Subscription,
        extractor: YTStreamInfoItemExtractor,
    ) -> Result<Self, tf_core::Error> {
        let url = extractor
            .url()
            .map_err(|e| tf_core::Error::from(tf_core::ParseError(format!("{}", e))))?;
        let title = extractor
            .name()
            .map_err(|e| tf_core::Error::from(tf_core::ParseError(format!("{}", e))))?;
        let uploaded = extractor
            .textual_upload_date()
            .map_err(|e| tf_core::Error::from(tf_core::ParseError(format!("{}", e))))?;

        Ok(Video {
            url,
            title,
            subscription,
            uploaded,
        })
    }
}

#[async_trait]
impl tf_core::Video for Video {
    type Subscription = Subscription;
    type Rating = ();
    type Thumbnail = ();
    type UploadTime = String;

    async fn url(&self) -> String {
        self.url.clone()
    }
    async fn title(&self) -> String {
        self.title.clone()
    }
    async fn subscription(&self) -> Self::Subscription {
        self.subscription.clone()
    }
    async fn uploaded(&self) -> Self::UploadTime {
        self.uploaded.clone()
    }
    async fn rating(&self) -> Self::Rating {}
    async fn thumbnail(&self) -> Self::Thumbnail {}
}
