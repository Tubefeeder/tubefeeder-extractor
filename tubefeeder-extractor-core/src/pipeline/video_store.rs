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

use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Weak;
use std::sync::{Arc, Mutex};

pub(crate) struct VideoStore<V> {
    videos: HashMap<V, Weak<Mutex<V>>>,
}

impl<V: Hash + Clone + std::cmp::Eq> VideoStore<V> {
    pub(crate) fn new() -> Self {
        VideoStore {
            videos: HashMap::new(),
        }
    }

    pub(crate) fn get(&mut self, video: V) -> Arc<Mutex<V>> {
        if let Some(value) = self.videos.get(&video) {
            if let Some(strong) = value.upgrade() {
                strong
            } else {
                let value = Arc::new(Mutex::new(video.clone()));
                self.videos.insert(video, Arc::downgrade(&value));
                value
            }
        } else {
            let value = Arc::new(Mutex::new(video.clone()));
            self.videos.insert(video, Arc::downgrade(&value));
            value
        }
    }
}

#[cfg(test)]
mod test {
    use crate::mock::MockVideo;
    use chrono::{NaiveDate, NaiveDateTime};

    use super::*;

    fn make_video(datetime: NaiveDateTime) -> MockVideo {
        let datetime_clone = datetime.clone();
        let mut video = MockVideo::new();
        video.expect_uploaded().returning(move || datetime_clone);
        video.expect_clone().returning(move || make_video(datetime));
        video
    }

    #[test]
    fn video_store_empty() {
        let store = VideoStore::<MockVideo>::new();
        assert!(store.videos.is_empty())
    }

    #[test]
    fn video_store_no_duplicates() {
        let mut store = VideoStore::<MockVideo>::new();
        let arc1 = store.get(make_video(
            NaiveDate::from_ymd(2021, 8, 21).and_hms(0, 0, 0),
        ));

        let arc2 = store.get(make_video(
            NaiveDate::from_ymd(2021, 8, 20).and_hms(0, 0, 0),
        ));

        assert_eq!(store.videos.len(), 2);
        assert!(!Arc::ptr_eq(&arc1, &arc2));
    }
    #[test]
    fn video_store_duplicates() {
        let mut store = VideoStore::<MockVideo>::new();

        let date = NaiveDate::from_ymd(2021, 8, 21).and_hms(0, 0, 0);
        let arc1 = store.get(make_video(date.clone()));

        let arc2 = store.get(make_video(date));

        assert_eq!(store.videos.len(), 1);
        assert!(Arc::ptr_eq(&arc1, &arc2));
    }
}
