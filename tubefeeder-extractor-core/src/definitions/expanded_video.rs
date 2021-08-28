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

use std::{hash::Hash, path::Path};

use async_trait::async_trait;

use crate::{Observable, ObserverList, Video};

#[derive(Clone)]
pub struct ExpandedVideo<V> {
    observers: ObserverList<VideoEvent>,
    video: V,
    playing: bool,
}

impl<V> PartialEq for ExpandedVideo<V>
where
    V: Video,
{
    fn eq(&self, other: &Self) -> bool {
        self.video.eq(&other.video)
    }
}

impl<V> Eq for ExpandedVideo<V> where V: Video {}

impl<V> Hash for ExpandedVideo<V>
where
    V: Video,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.video.hash(state);
    }
}

#[async_trait]
impl<V: Video> Video for ExpandedVideo<V> {
    type Subscription = V::Subscription;

    fn url(&self) -> String {
        self.video.url()
    }

    fn title(&self) -> String {
        self.video.title()
    }

    fn uploaded(&self) -> chrono::NaiveDateTime {
        self.video.uploaded()
    }

    fn subscription(&self) -> Self::Subscription {
        self.video.subscription()
    }

    async fn thumbnail_with_client<P: AsRef<Path> + Send>(
        &self,
        client: &reqwest::Client,
        filename: P,
        width: i32,
        height: i32,
    ) {
        self.video
            .thumbnail_with_client(client, filename, width, height)
            .await
    }
}

impl<V> From<V> for ExpandedVideo<V>
where
    V: Video,
{
    fn from(video: V) -> Self {
        ExpandedVideo {
            video,
            playing: false,
            observers: ObserverList::new(),
        }
    }
}

impl<V: Video> ExpandedVideo<V> {
    pub fn play(&mut self) {
        self.playing = true;
        self.observers.notify(VideoEvent::Play);
    }

    pub fn stop(&mut self) {
        self.playing = false;
        self.observers.notify(VideoEvent::Stop);
    }

    pub fn playing(&self) -> bool {
        self.playing
    }
}

#[derive(Clone)]
pub enum VideoEvent {
    Play,
    Stop,
}

impl<V: Video> Observable<VideoEvent> for ExpandedVideo<V> {
    fn attach(
        &mut self,
        observer: std::sync::Weak<std::sync::Mutex<Box<dyn crate::Observer<VideoEvent> + Send>>>,
    ) {
        self.observers.attach(observer)
    }

    fn detach(
        &mut self,
        observer: std::sync::Weak<std::sync::Mutex<Box<dyn crate::Observer<VideoEvent> + Send>>>,
    ) {
        self.observers.detach(observer)
    }
}
