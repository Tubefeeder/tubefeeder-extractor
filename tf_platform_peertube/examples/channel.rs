extern crate tf_platform_peertube as tf_pt;

use std::error::Error;
use tf_pt::{PTSubscription, PTVideo};
use tf_core::{ErrorStore, GeneratorWithClient, Video};

const BASE_URL: &str = "https://peertube.linuxrocks.online";
const ID: &str = "3300";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let sub = PTSubscription::new(BASE_URL, ID);

    let client = reqwest::Client::new();
    let error_store = ErrorStore::new();

    let videos: Vec<PTVideo> = sub.generate_with_client(&error_store, &client).await.collect();

    for v in videos {
        println!("Video {}", v.title());
    }

    Ok(())
}