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

extern crate tubefeeder_extractor_join as tf_join;

use tf_core::{Generator, Video};
use tf_join::AnySubscription;
use tf_join::AnyVideo;
use tf_join::Joiner;
use tf_test::TestSubscription;
use tf_yt::YTSubscription;

const YT_SUBSCRIPTION_IDS: &'static [&'static str] = &["UCj1VqrHhDte54oLgPG4xpuQ"];
const TEST_SUBSCRIPTION_NAMES: &'static [&'static str] = &["Test1", "Test2"];

#[tokio::main(flavor = "current_thread")]
pub async fn main() {
    env_logger::init();
    log::info!("Logging enabled");
    let join = Joiner::new();

    YT_SUBSCRIPTION_IDS
        .iter()
        .map(|id| YTSubscription::new(id).into())
        .for_each(|sub: AnySubscription| join.subscribe(sub.into()));

    TEST_SUBSCRIPTION_NAMES
        .iter()
        .map(|name| TestSubscription::new(name).into())
        .for_each(|sub: AnySubscription| join.subscribe(sub.into()));

    println!("VIDEOS: ");
    for video in join.generate().await.0.take(100) {
        match video {
            AnyVideo::Youtube(v) => {
                let yt_v = v.lock().unwrap();
                let sub = yt_v.subscription();
                println!(
                    "YouT: {} - {}",
                    sub.name().unwrap_or(sub.id()),
                    yt_v.title()
                );
            }
            AnyVideo::Test(v) => {
                let test_v = v.lock().unwrap();
                let sub = test_v.subscription();
                println!("Test: {} - {}", sub.name(), test_v.title());
            }
        }
    }
}
