use std::hash::Hash;

use crate::Video;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ExpandedVideo<V> {
    video: V,
    playing: bool,
}

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
}

impl<V> From<V> for ExpandedVideo<V>
where
    V: Video,
{
    fn from(video: V) -> Self {
        ExpandedVideo {
            video,
            playing: false,
        }
    }
}

impl<V: Video> ExpandedVideo<V> {
    pub fn play(&mut self) {
        self.playing = true;
    }

    pub fn stop(&mut self) {
        self.playing = false;
    }
}
