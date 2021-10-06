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

use std::hash::Hash;

use async_trait::async_trait;

use crate::Video;
use tf_observer::{Observable, Observer, ObserverList};

/// A [Video] with a expanded feature set.
///
/// A [ExpandedVideo] implements [Observable] and will notify using [VideoEvent].
/// You can set the playing-status of the [ExpandedVideo] using [ExpandedVideo::play]
/// and [ExpandedVideo::stop].
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

    fn thumbnail_url(&self) -> String {
        self.video.thumbnail_url()
    }

    async fn thumbnail_with_client(&self, client: &reqwest::Client) -> image::DynamicImage {
        self.video.thumbnail_with_client(client).await
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
    /// Mark the [ExpandedVideo] as playing and notify the observers
    /// using [VideoEvent::Play].
    pub fn play(&mut self) {
        self.playing = true;
        self.observers.notify(VideoEvent::Play);
    }

    /// Mark the [ExpandedVideo] as stopped and notify the observers
    /// using [VideoEvent::Stop].
    pub fn stop(&mut self) {
        self.playing = false;
        self.observers.notify(VideoEvent::Stop);
    }

    /// Get if the video is currently playing.
    pub fn playing(&self) -> bool {
        self.playing
    }

    /// Get a clone of the internal video.
    pub fn internal(&self) -> V {
        self.video.clone()
    }
}

/// A event thrown by [ExpandedVideo].
#[derive(Clone)]
pub enum VideoEvent {
    /// The [ExpandedVideo] was played, see [ExpandedVideo::play].
    Play,
    /// The [ExpandedVideo] was stopped, see [ExpandedVideo::stop].
    Stop,
}

impl<V: Video> Observable<VideoEvent> for ExpandedVideo<V> {
    fn attach(
        &mut self,
        observer: std::sync::Weak<std::sync::Mutex<Box<dyn Observer<VideoEvent> + Send>>>,
    ) {
        self.observers.attach(observer)
    }

    fn detach(
        &mut self,
        observer: std::sync::Weak<std::sync::Mutex<Box<dyn Observer<VideoEvent> + Send>>>,
    ) {
        self.observers.detach(observer)
    }
}
