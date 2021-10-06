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

extern crate tf_platform_lbry as tf_lbry;

use std::error::Error;
use tf_core::{ErrorStore, GeneratorWithClient, Video};
use tf_lbry::{LbrySubscription, LbryVideo};

const ID: &str = "@SomeOrdinaryGamers:a";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let sub = LbrySubscription::new(ID);

    let client = reqwest::Client::new();
    let error_store = ErrorStore::new();

    let videos: Vec<LbryVideo> = sub
        .generate_with_client(&error_store, &client)
        .await
        .collect();

    for v in videos {
        println!("Video {}", v.title());
    }

    Ok(())
}
