use crate::subscription::Subscription;
use crate::video::Video;

use async_trait::async_trait;
use rusty_pipe::extractors::YTChannelExtractor;

pub struct Generator {
    subscription: Subscription,
}

impl Generator {
    pub fn new(subscription: &Subscription) -> Self {
        Generator {
            subscription: subscription.clone(),
        }
    }
}

#[async_trait]
impl tf_core::Generator for Generator {
    type Item = Video;

    async fn generate(&self) -> Result<Vec<Self::Item>, tf_core::Error> {
        log::trace!(
            "Generating videos from the youtube channel {}",
            self.subscription.id()
        );
        let channel_extractor =
            YTChannelExtractor::new::<crate::Downloader>(&self.subscription.id(), None)
                .await
                .map_err(|e| tf_core::ParseError(format!("{}", e)))?;
        let videos_extractor = channel_extractor
            .videos()
            .map_err(|e| tf_core::ParseError(format!("{}", e)))?;

        let mut videos = vec![];
        for extractor in videos_extractor {
            let video = Video::from_extractor(self.subscription.clone(), extractor)?;
            log::trace!("Found {:?}", video);
            videos.push(video);
        }

        Ok(videos)
    }
}

#[cfg(test)]
mod test {
    use tf_core::Generator;
    use tf_core::Subscription;

    /// TODO: Move the test to rusty_pipes.
    #[tokio::test]
    async fn test() {
        let _ = env_logger::init();
        let subscription = crate::subscription::Subscription::new("UCSMOQeBJ2RAnuFungnQOxLg");
        let generator = subscription.generator();
        println!("{:?}", generator.generate().await);
        // panic!()
    }
}
