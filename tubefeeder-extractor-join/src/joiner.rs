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

use std::{future::Future, pin::Pin};

use tf_core::{Generator, Pipeline};

use async_trait::async_trait;

use crate::{AnySubscription, AnyVideo};

pub struct Joiner {
    #[cfg(feature = "youtube")]
    yt_pipeline: Pipeline<tf_yt::YTSubscription, tf_yt::YTVideo>,
    #[cfg(feature = "testPlatform")]
    test_pipeline: Pipeline<tf_test::TestSubscription, tf_test::TestVideo>,
}

impl Joiner {
    pub fn new() -> Self {
        Joiner {
            #[cfg(feature = "youtube")]
            yt_pipeline: Pipeline::new(),
            #[cfg(feature = "testPlatform")]
            test_pipeline: Pipeline::new(),
        }
    }

    pub fn subscribe(&self, subscription: AnySubscription) {
        match subscription {
            #[cfg(feature = "youtube")]
            AnySubscription::Youtube(s) => {
                self.yt_pipeline.subscription_list().lock().unwrap().add(s)
            }
            #[cfg(feature = "testPlatform")]
            AnySubscription::Test(s) => self
                .test_pipeline
                .subscription_list()
                .lock()
                .unwrap()
                .add(s),
        }
    }
}

#[async_trait]
impl Generator for Joiner {
    type Item = AnyVideo;

    type Iterator = std::vec::IntoIter<AnyVideo>;

    async fn generate(&self) -> (Self::Iterator, Option<tf_core::Error>) {
        // TODO: Error handling
        // TODO: More efficient
        let mut generators: Vec<
            Pin<
                Box<
                    dyn Future<
                            Output = (
                                Box<dyn Iterator<Item = AnyVideo> + std::marker::Send>,
                                Option<tf_core::Error>,
                            ),
                        > + std::marker::Send,
                >,
            >,
        > = vec![];
        #[cfg(feature = "youtube")]
        generators.push(Box::pin(async {
            let (iter, err) = self.yt_pipeline.generate().await;
            let iter_mapped = iter.map(|v| v.into());
            (
                Box::new(iter_mapped) as Box<dyn Iterator<Item = AnyVideo> + std::marker::Send>,
                err,
            )
        }));
        #[cfg(feature = "testPlatform")]
        generators.push(Box::pin(async {
            let (iter, err) = self.test_pipeline.generate().await;
            let iter_mapped = iter.map(|v| v.into());
            (
                Box::new(iter_mapped) as Box<dyn Iterator<Item = AnyVideo> + std::marker::Send>,
                err,
            )
        }));

        let results = futures::future::join_all(generators).await;
        let mut videos: Vec<AnyVideo> = results
            .into_iter()
            .map(|(i, _e)| i.collect())
            .collect::<Vec<Vec<AnyVideo>>>()
            .concat();
        videos.sort_by_cached_key(|v| v.uploaded());
        videos.reverse();
        (videos.into_iter(), None)
    }
}
