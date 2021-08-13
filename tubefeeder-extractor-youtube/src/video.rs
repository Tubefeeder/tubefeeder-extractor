use crate::structure::*;
use crate::subscription::YTSubscription;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct YTVideo {
    pub(crate) url: String,
    pub(crate) title: String,
    pub(crate) uploaded: chrono::NaiveDateTime,
    pub(crate) subscription: YTSubscription,
}

impl tf_core::Video for YTVideo {
    type Subscription = YTSubscription;

    fn url(&self) -> String {
        self.url.clone()
    }

    fn title(&self) -> String {
        self.title.clone()
    }

    fn subscription(&self) -> Self::Subscription {
        self.subscription.clone()
    }

    fn uploaded(&self) -> chrono::NaiveDateTime {
        self.uploaded.clone()
    }
}

impl From<Feed> for Vec<YTVideo> {
    fn from(feed: Feed) -> Self {
        feed.entries.into_iter().map(|e| e.into()).collect()
    }
}

impl From<Entry> for YTVideo {
    fn from(e: Entry) -> Self {
        let subscription = YTSubscription::new(e.author.uri.split("/").last().unwrap_or(""));

        YTVideo {
            url: e.link.href.to_string(),
            title: e.title,
            subscription,
            uploaded: e.published,
        }
    }
}
