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

use std::convert::TryFrom;
use std::str::FromStr;

use regex::Regex;

use crate::{AnyVideo, Platform};

use tf_core::{Subscription, Video};
use tf_filter::Filter;

/// A [Filter] for filtering [AnyVideo]s.
#[derive(Debug, Clone)]
pub struct AnyVideoFilter {
    /// Filter the [Platform].
    ///
    /// If this is `None`, the [Platform] will be ignored.
    platform: Option<Platform>,

    /// Filter the [AnyVideo::title].
    ///
    /// If this is `None`, the [AnyVideo::title] will be ignored.
    title: Option<Regex>,

    /// Filter the [AnySubscription::name][crate::AnySubscription::name].
    ///
    /// If this is `None`, the [AnySubscription::name][crate::AnySubscription::name] will be ignored.
    subscription: Option<Regex>,
}

impl PartialEq for AnyVideoFilter {
    fn eq(&self, other: &Self) -> bool {
        self.platform == other.platform
            && self.title.as_ref().map(|r| r.to_string())
                == other.title.as_ref().map(|r| r.to_string())
            && self.subscription.as_ref().map(|r| r.to_string())
                == other.subscription.as_ref().map(|r| r.to_string())
    }
}

impl Eq for AnyVideoFilter {}

impl std::hash::Hash for AnyVideoFilter {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.platform.hash(state);
        self.title.as_ref().map(|r| r.to_string()).hash(state);
        self.subscription
            .as_ref()
            .map(|r| r.to_string())
            .hash(state);
    }
}

impl AnyVideoFilter {
    /// Create a new [AnyVideoFilter] matching a [AnyVideo].
    ///
    /// The [Platform], [AnyVideo::title] and [AnySubscription::name][crate::AnySubscription::name] will be matched.
    /// If the respecting field is `None`, this field will be ignored.
    pub fn new(
        platform: Option<Platform>,
        title: Option<Regex>,
        subscription: Option<Regex>,
    ) -> Self {
        AnyVideoFilter {
            platform,
            title,
            subscription,
        }
    }

    /// Give the title-regex as a String if the title-regex is set.
    pub fn title_str(&self) -> Option<String> {
        self.title.clone().map(|r| r.to_string())
    }

    /// Give the subscription-regex as a String if the subscription-regex is set.
    pub fn subscription_str(&self) -> Option<String> {
        self.subscription.clone().map(|r| r.to_string())
    }
}

impl Filter for AnyVideoFilter {
    type Item = AnyVideo;

    fn matches(&self, video: &<Self as Filter>::Item) -> bool {
        if let Some(platform) = &self.platform {
            if &video.platform() != platform {
                return false;
            }
        }

        if let Some(title) = &self.title {
            if !title.is_match(&video.title()) {
                return false;
            }
        }

        if let Some(subscription) = &self.subscription {
            if !subscription.is_match(&video.subscription().name().unwrap_or("".to_string())) {
                return false;
            }
        }

        return true;
    }
}

impl TryFrom<&[&str]> for AnyVideoFilter {
    // TODO: Error handling
    type Error = ();

    fn try_from(value: &[&str]) -> Result<Self, Self::Error> {
        let platform_opt = value.get(0);
        let title_opt = value.get(1);
        let subscription_opt = value.get(2);

        if platform_opt.is_none() || title_opt.is_none() || subscription_opt.is_none() {
            return Err(());
        }

        let platform = map_empty_to_none(platform_opt.unwrap()).map(|p| Platform::from_str(&p));
        let title = map_empty_to_none(title_opt.unwrap()).map(|s| Regex::new(&s));
        let subscription = map_empty_to_none(subscription_opt.unwrap()).map(|s| Regex::new(&s));

        if let Some(Err(_e)) = platform {
            return Err(());
        }

        if let Some(Err(_e)) = title {
            return Err(());
        }

        if let Some(Err(_e)) = subscription {
            return Err(());
        }

        Ok(AnyVideoFilter::new(
            platform.map(|r: Result<Platform, _>| r.unwrap()),
            title.map(|r| r.unwrap()),
            subscription.map(|r| r.unwrap()),
        ))
    }
}

impl From<AnyVideoFilter> for Vec<String> {
    fn from(filter: AnyVideoFilter) -> Self {
        let mut result = vec![];
        result.push(filter.platform.map(|p| p.into()).unwrap_or("".to_string()));
        result.push(
            filter
                .title
                .map(|r| r.to_string())
                .unwrap_or("".to_string()),
        );
        result.push(
            filter
                .subscription
                .map(|r| r.to_string())
                .unwrap_or("".to_string()),
        );
        return result;
    }
}

/// Maps a empty String to `None`, otherwise to `Some` of the given String.
fn map_empty_to_none<S: AsRef<str>>(st: S) -> Option<String> {
    let string = st.as_ref().to_string();
    if string.is_empty() {
        None
    } else {
        Some(string)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::convert::TryInto;
    use std::sync::{Arc, Mutex};
    use tf_core::ExpandedVideo;
    use tf_test::{TestSubscription, TestVideo};

    #[test]
    fn filter_match_title_subscription() {
        let sub = TestSubscription::new("Subscription");
        let video = Arc::new(Mutex::new(ExpandedVideo::from(TestVideo::new(
            "Title", sub,
        ))))
        .into();

        let filter = AnyVideoFilter::new(
            None,
            Some(Regex::new("itl").unwrap()),
            Some(Regex::new("ubscr").unwrap()),
        );

        assert!(filter.matches(&video));
    }

    #[test]
    fn filter_match_title() {
        let sub = TestSubscription::new("Subscription");
        let video = Arc::new(Mutex::new(ExpandedVideo::from(TestVideo::new(
            "Title", sub,
        ))))
        .into();

        let filter = AnyVideoFilter::new(None, Some(Regex::new("itl").unwrap()), None);

        assert!(filter.matches(&video));
    }

    #[test]
    fn filter_match_all() {
        let sub = TestSubscription::new("Subscription");
        let video = Arc::new(Mutex::new(ExpandedVideo::from(TestVideo::new(
            "Title", sub,
        ))))
        .into();

        let filter = AnyVideoFilter::new(
            Some(Platform::Test),
            Some(Regex::new("itl").unwrap()),
            Some(Regex::new("ubscr").unwrap()),
        );

        assert!(filter.matches(&video));
    }

    #[test]
    fn filter_no_match_title() {
        let sub = TestSubscription::new("Subscription");
        let video = Arc::new(Mutex::new(ExpandedVideo::from(TestVideo::new(
            "Title", sub,
        ))))
        .into();

        let filter = AnyVideoFilter::new(
            Some(Platform::Test),
            Some(Regex::new("nomatch").unwrap()),
            Some(Regex::new("ubscr").unwrap()),
        );

        assert!(!filter.matches(&video));
    }

    #[test]
    fn filter_no_match_subscription() {
        let sub = TestSubscription::new("Subscription");
        let video = Arc::new(Mutex::new(ExpandedVideo::from(TestVideo::new(
            "Title", sub,
        ))))
        .into();

        let filter = AnyVideoFilter::new(
            Some(Platform::Test),
            Some(Regex::new("itl").unwrap()),
            Some(Regex::new("nomatch").unwrap()),
        );

        assert!(!filter.matches(&video));
    }

    #[test]
    #[cfg(feature = "youtube")]
    fn filter_no_match_platform() {
        let sub = TestSubscription::new("Subscription");
        let video = Arc::new(Mutex::new(ExpandedVideo::from(TestVideo::new(
            "Title", sub,
        ))))
        .into();

        let filter = AnyVideoFilter::new(
            Some(Platform::Youtube),
            Some(Regex::new("itl").unwrap()),
            Some(Regex::new("ubscr").unwrap()),
        );

        assert!(!filter.matches(&video));
    }

    #[test]
    fn filter_conversion_all() {
        let filter = AnyVideoFilter::new(
            Some(Platform::Test),
            Some(Regex::new("itl").unwrap()),
            Some(Regex::new("ubscr").unwrap()),
        );

        assert_eq!(
            Vec::<String>::from(filter),
            vec!["test".to_string(), "itl".to_string(), "ubscr".to_string()]
        );
    }

    #[test]
    fn filter_conversion_title() {
        let filter = AnyVideoFilter::new(None, Some(Regex::new("itl").unwrap()), None);

        assert_eq!(
            Vec::<String>::from(filter),
            vec!["".to_string(), "itl".to_string(), "".to_string()]
        );
    }

    #[test]
    fn filter_conversion_all_back() {
        let filter = AnyVideoFilter::new(
            Some(Platform::Test),
            Some(Regex::new("itl").unwrap()),
            Some(Regex::new("ubscr").unwrap()),
        );

        assert_eq!(
            Ok(filter),
            vec!["test", "itl", "ubscr"].as_slice().try_into()
        );
    }

    #[test]
    fn filter_conversion_title_back() {
        let filter = AnyVideoFilter::new(None, Some(Regex::new("itl").unwrap()), None);

        assert_eq!(Ok(filter), vec!["", "itl", ""].as_slice().try_into());
    }

    #[test]
    fn filter_conversion_back_fail() {
        assert!(AnyVideoFilter::try_from(vec!["", "itl"].as_slice()).is_err());
    }
}
