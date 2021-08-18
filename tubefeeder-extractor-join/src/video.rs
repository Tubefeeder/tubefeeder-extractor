use std::sync::{Arc, Mutex};

use tf_core::{ExpandedVideo, Video};

use crate::AnySubscription;

pub enum AnyVideo {
    Youtube(Arc<Mutex<ExpandedVideo<tf_yt::YTVideo>>>),
    Test(Arc<Mutex<ExpandedVideo<tf_test::TestVideo>>>),
}

impl AnyVideo {
    pub fn url(&self) -> String {
        match self {
            AnyVideo::Youtube(yt) => yt.lock().unwrap().url(),
            AnyVideo::Test(test) => test.lock().unwrap().url(),
        }
    }

    pub fn title(&self) -> String {
        match self {
            AnyVideo::Youtube(yt) => yt.lock().unwrap().title(),
            AnyVideo::Test(test) => test.lock().unwrap().title(),
        }
    }

    pub fn uploaded(&self) -> chrono::NaiveDateTime {
        match self {
            AnyVideo::Youtube(yt) => yt.lock().unwrap().uploaded(),
            AnyVideo::Test(test) => test.lock().unwrap().uploaded(),
        }
    }

    pub fn subscription(&self) -> AnySubscription {
        match self {
            AnyVideo::Youtube(yt) => yt.lock().unwrap().subscription().into(),
            AnyVideo::Test(test) => test.lock().unwrap().subscription().into(),
        }
    }
}

impl From<Arc<Mutex<ExpandedVideo<tf_yt::YTVideo>>>> for AnyVideo {
    fn from(v: Arc<Mutex<ExpandedVideo<tf_yt::YTVideo>>>) -> Self {
        AnyVideo::Youtube(v)
    }
}

impl From<Arc<Mutex<ExpandedVideo<tf_test::TestVideo>>>> for AnyVideo {
    fn from(v: Arc<Mutex<ExpandedVideo<tf_test::TestVideo>>>) -> Self {
        AnyVideo::Test(v)
    }
}
