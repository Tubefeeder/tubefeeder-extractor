use crate::video::YTVideo;

use std::pin::Pin;

use futures::Stream;
use rusty_pipe::extractors::YTChannelExtractor;

use tf_core::Subscription;

#[derive(Clone, Debug)]
pub struct YTSubscription {
    id: String,
}

impl YTSubscription {
    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn new(id: &str) -> Self {
        YTSubscription { id: id.to_owned() }
    }
}

impl Subscription for YTSubscription {
    type Video = YTVideo;
    fn generator(&self) -> Pin<Box<dyn Stream<Item = Result<YTVideo, tf_core::Error>>>> {
        let id = self.id.clone();
        let self_clone = self.clone();
        log::debug!("Constructing Stream");
        let stream = async_stream::stream! {
            let mut channel_extractor = YTChannelExtractor::new::<crate::Downloader>(&id, None).await;
            loop {
                log::debug!("Got new extractor");
                if let Err(e) = channel_extractor {
                    log::error!("Failed parsing channel page with id {}", id);
                    yield Err(tf_core::ParseError(format!("{}", e)).into());
                    break;
                }

                let videos = channel_extractor.as_ref().unwrap().videos();

                if let Err(e) = videos {
                    log::error!("Failed parsing video page of channel with id {}", id);
                    yield Err(tf_core::ParseError(format!("{}", e)).into());
                    break;
                }


                for v in videos.unwrap() {
                    let video = YTVideo::from_extractor(self_clone.clone(), v).await;
                    if let Err(e) = video {
                        log::error!("Failed parsing video of channel with id {}", id);
                        yield Err(e);
                    } else {
                        yield Ok(video.unwrap());
                    }
                }

                let next_page_url = channel_extractor.as_ref().unwrap().next_page_url();

                if let Err(e) = next_page_url {
                    log::error!("Failed parsing next page of channel with id {}", id);
                    yield Err(tf_core::ParseError(format!("{}", e)).into());
                    break;
                }

                if let Ok(None) = next_page_url {
                    log::debug!("Got no new page, exiting stream");
                    break;
                }

                channel_extractor = YTChannelExtractor::new::<crate::Downloader>(&id, Some(next_page_url.unwrap().unwrap())).await;
            }
        };

        Box::pin(stream)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use futures::stream::StreamExt;

    #[tokio::test]
    async fn test() {
        let _ = env_logger::init();
        let subscription = YTSubscription::new("UCSMOQeBJ2RAnuFungnQOxLg");
        let mut stream = subscription.generator();

        while let Some(v) = stream.as_mut().next().await {
            println!("{:?}", v);
        }

        panic!()
    }
}
