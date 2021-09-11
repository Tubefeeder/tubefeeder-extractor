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

use tf_core::{ErrorStore, Generator};
use tf_join::{AnySubscription, AnyVideo, Joiner};

#[cfg(test)]
use tf_test::TestSubscription;
#[cfg(feature = "youtube")]
use tf_yt::YTSubscription;

#[cfg(feature = "youtube")]
const YT_SUBSCRIPTION_IDS: &'static [&'static str] = &["UCj1VqrHhDte54oLgPG4xpuQ"];
#[cfg(test)]
const TEST_SUBSCRIPTION_NAMES: &'static [&'static str] = &["Test1", "Test2"];

#[tokio::main(flavor = "current_thread")]
pub async fn main() {
    env_logger::init();
    log::info!("Logging enabled");
    let join = Joiner::new();
    let errors = ErrorStore::new();

    let subscription_list = join.subscription_list();

    #[cfg(feature = "youtube")]
    YT_SUBSCRIPTION_IDS
        .iter()
        .map(|id| YTSubscription::new(id).into())
        .for_each(|sub: AnySubscription| subscription_list.add(sub));

    #[cfg(test)]
    TEST_SUBSCRIPTION_NAMES
        .iter()
        .map(|name| TestSubscription::new(name).into())
        .for_each(|sub: AnySubscription| subscription_list.add(sub));

    println!("VIDEOS: ");
    for video in join.generate(&errors).await.take(100) {
        let source = match &video {
            #[cfg(feature = "youtube")]
            AnyVideo::Youtube(_v) => "YouT",
            #[cfg(test)]
            AnyVideo::Test(_v) => "Test",
        };

        println!("{}: {} - {}", source, video.title(), video.subscription())
    }
}
