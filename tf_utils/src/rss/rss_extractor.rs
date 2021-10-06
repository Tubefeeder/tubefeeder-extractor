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

use tf_core::{GeneratorWithClient, NetworkError, ParseError, Subscription, Video};

use crate::rss::Item;

use super::Rss;

pub struct RssExtractorWrapper<S>(S);

impl<S> From<S> for RssExtractorWrapper<S> {
    fn from(sub: S) -> Self {
        RssExtractorWrapper(sub)
    }
}

impl<S> From<&S> for RssExtractorWrapper<S>
where
    S: Clone,
{
    fn from(sub: &S) -> Self {
        RssExtractorWrapper(sub.clone())
    }
}

pub trait FromItemAndSub<S> {
    fn from_item_and_sub(item: Item, sub: S) -> Self;
}

pub trait WithName {
    fn with_name<S: AsRef<str>>(&self, name: S) -> Self;
}

pub trait RssExtractor {
    fn feed_url(&self) -> String;
}

#[async_trait::async_trait]
impl<S, V> GeneratorWithClient for RssExtractorWrapper<S>
where
    S: Subscription<Video = V> + WithName + RssExtractor,
    V: Video<Subscription = S> + FromItemAndSub<S>,
{
    type Item = V;

    type Iterator = std::vec::IntoIter<Self::Item>;

    async fn generate_with_client(
        &self,
        errors: &tf_core::ErrorStore,
        client: &reqwest::Client,
    ) -> Self::Iterator {
        let rss_res = parse_rss_from_url(&self.0.feed_url(), client).await;

        if rss_res.is_err() {
            errors.add(rss_res.err().unwrap());
            return vec![].into_iter();
        }

        let rss = rss_res.unwrap();

        let name = rss.channel.itunes_author;
        let items = rss.channel.items;

        let items_pt_video: Vec<V> = items
            .into_iter()
            .map(|i| V::from_item_and_sub(i, self.0.with_name(&name)))
            .collect();

        items_pt_video.into_iter()
    }
}

async fn parse_rss_from_url(url: &str, client: &reqwest::Client) -> Result<Rss, tf_core::Error> {
    let response = client.get(url.clone()).send().await;

    if response.is_err() {
        log::error!("Error getting {:?}", url);
        return Err(NetworkError(url.to_string()).into());
    }

    let body_res = response.unwrap().text().await;

    if body_res.is_err() {
        log::error!("Error getting {:?}", url);
        return Err(NetworkError(url.to_string()).into());
    }

    let body_parsable = body_res
        .unwrap()
        .replace("media:", "media/")
        .replace("itunes:", "itunes/");

    let rss_res: Result<Rss, quick_xml::de::DeError> = quick_xml::de::from_str(&body_parsable);

    if rss_res.is_err() {
        log::error!("Error parsing: {}", &rss_res.err().unwrap());
        return Err(ParseError(body_parsable).into());
    }

    Ok(rss_res.unwrap())
}
