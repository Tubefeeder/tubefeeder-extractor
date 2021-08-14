/*
 * Copyright 2021 Julian Schmidhuber <github@schmiddi.anonaddy.com>
 *
 * This file is part of Tubefeeder-extractor.
 *
 * Tubefeeder-extractor is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Tubefeeder-extractor is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Tubefeeder-extractor.  If not, see <https://www.gnu.org/licenses/>.
 */

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
