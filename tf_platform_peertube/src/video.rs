use std::path::Path;

use async_trait::async_trait;
use gdk_pixbuf::Pixbuf;
use gio::{MemoryInputStream, NONE_CANCELLABLE};

use crate::{structure::Item, PTSubscription};

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct PTVideo {
    pub(crate) url: String,
    pub(crate) title: String,
    pub(crate) uploaded: chrono::NaiveDateTime,
    pub(crate) subscription: PTSubscription,
    pub(crate) thumbnail_url: String,
}

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

    pub fn thumbnail_url(&self) -> String {
        self.thumbnail_url.clone()
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

    async fn thumbnail_with_client<P: AsRef<Path> + Send>(
        &self,
        client: &reqwest::Client,
        filename: P,
        width: i32,
        height: i32,
    ) {
        log::debug!("Getting thumbnail for peertube video {}", self.title);
        let response = client.get(&self.thumbnail_url).send().await;
        log::debug!(
            "Got response for thumbnail for peertube video {}",
            self.title
        );

        if response.is_err() {
            log::debug!(
                "Failed getting thumbnail for peertube video {}, use default",
                self.title
            );
            self.default_thumbnail(filename, width, height);
            return;
        }

        let parsed = response.unwrap().bytes().await;

        if parsed.is_err() {
            log::debug!(
                "Failed getting thumbnail for peertube video {}, use default",
                self.title
            );
            self.default_thumbnail(filename, width, height);
            return;
        }

        let parsed_bytes = parsed.unwrap();

        let glib_bytes = glib::Bytes::from(&parsed_bytes.to_vec());

        let stream = MemoryInputStream::from_bytes(&glib_bytes);

        log::debug!(
            "Finished Getting thumbnail for youtube video {}",
            self.title
        );
        let pixbuf = Pixbuf::from_stream_at_scale(&stream, width, height, true, NONE_CANCELLABLE);
        if let Ok(pixbuf) = pixbuf {
            let _ = pixbuf.savev(filename, "png", &[]);
        } else {
            self.default_thumbnail(filename, width, height);
        }
    }
}

impl PTVideo {
    pub(crate) fn from_item_and_sub(i: Item, sub: PTSubscription) -> Self {
        Self {
            title: i.title,
            url: i.link,
            uploaded: i.pub_date,
            subscription: sub,
            thumbnail_url: i.thumbnail.url,
        }
    }
}
