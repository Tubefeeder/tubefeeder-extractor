use tf_core::{GeneratorWithClient, NetworkError, ParseError};

use crate::{structure::Rss, PTVideo};

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct PTSubscription {
    id: String,
    base_url: String,
    name: Option<String>,
}

impl PTSubscription {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>>(base_url: S1, id: S2) -> Self {
        Self {
            id: id.as_ref().to_owned(),
            base_url: base_url.as_ref().to_owned(),
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
        let url = format!(
            "{}/feeds/videos.xml?videoChannelId={}",
            self.base_url, self.id
        );
        let response = client.get(url.clone()).send().await;

        if response.is_err() {
            log::error!("Error getting {:?}", &url);
            errors.add(NetworkError(url).into());
            return vec![].into_iter();
        }

        let body_res = response.unwrap().text().await;

        if body_res.is_err() {
            log::error!("Error getting {:?}", &url);
            errors.add(NetworkError(url).into());
            return vec![].into_iter();
        }

        let body_parsable = body_res.unwrap().replace("media:", "media/");

        let rss: Result<Rss, quick_xml::de::DeError> = quick_xml::de::from_str(&body_parsable);

        if rss.is_err() {
            log::error!("Error parsing: {}", &rss.err().unwrap());
            errors.add(ParseError(body_parsable).into());
            return vec![].into_iter();
        }

        let items = rss.unwrap().channel.items;

        let items_pt_video: Vec<PTVideo> = items
            .into_iter()
            .map(|i| PTVideo::from_item_and_sub(i, self.clone()))
            .collect();

        items_pt_video.into_iter()
    }
}
