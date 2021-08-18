use tf_core::{Subscription, Video};

use async_trait::async_trait;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TestVideo {
    title: String,
    uploaded: chrono::NaiveDateTime,
    subscription: TestSubscription,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TestSubscription {
    name: String,
}

impl Video for TestVideo {
    type Subscription = TestSubscription;

    fn url(&self) -> String {
        format!("https://test.test/{}", self.title)
    }

    fn title(&self) -> String {
        self.title.clone()
    }

    fn uploaded(&self) -> chrono::NaiveDateTime {
        self.uploaded.clone()
    }

    fn subscription(&self) -> TestSubscription {
        self.subscription.clone()
    }
}

#[async_trait]
impl Subscription for TestSubscription {
    type Video = TestVideo;
    type Iterator = std::vec::IntoIter<TestVideo>;
    async fn generate_with_client(
        &self,
        _: &reqwest::Client,
    ) -> (Self::Iterator, Option<tf_core::Error>) {
        let video1 = TestVideo {
            title: "This is the test video 1".to_owned(),
            uploaded: chrono::NaiveDate::from_ymd(2021, 8, 17).and_hms(0, 0, 0),
            subscription: self.clone(),
        };

        let video2 = TestVideo {
            title: "This is the test video 2".to_owned(),
            uploaded: chrono::NaiveDate::from_ymd(2021, 5, 1).and_hms(0, 0, 0),
            subscription: self.clone(),
        };

        (vec![video1, video2].into_iter(), None)
    }
}

impl TestVideo {
    pub fn title(&self) -> String {
        self.title.clone()
    }
}

impl TestSubscription {
    pub fn new(name: &str) -> Self {
        TestSubscription {
            name: name.to_owned(),
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn subscription() {
        let subscription = TestSubscription {
            name: "TestName".to_owned(),
        };

        let (iterator, error) = subscription.generate().await;

        assert!(error.is_none());

        let titles: Vec<String> = iterator.map(|v| v.title()).collect();

        assert_eq!(
            vec![
                "This is the test video 1".to_owned(),
                "This is the test video 2".to_owned()
            ],
            titles
        );
    }
}
