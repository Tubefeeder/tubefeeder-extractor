extern crate tubefeeder_extractor_youtube as tf_yt;

use tf_core::{Generator, Pipeline, Video};
use tf_yt::subscription::YTSubscription;
use tf_yt::video::YTVideo;

const SUBSCRIPTION_IDS: &'static [&'static str] =
    &["UCld68syR8Wi-GY_n4CaoJGA", "UCVls1GmFKf6WlTraIb_IaJg"];

#[tokio::main(flavor = "current_thread")]
pub async fn main() {
    let pipeline = Pipeline::<YTSubscription, YTVideo>::new();
    let subscriptions = pipeline.subscription_list();

    SUBSCRIPTION_IDS
        .iter()
        .map(|id| YTSubscription::new(id))
        .for_each(|sub| subscriptions.lock().unwrap().add(sub));

    println!("VIDEOS: ");
    for video in pipeline.generate().await.0 {
        println!("{}", video.lock().unwrap().title());
    }
}
