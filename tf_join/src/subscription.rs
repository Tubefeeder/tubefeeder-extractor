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

use std::{convert::TryFrom, str::FromStr};

use tf_core::Subscription;

/// A [Subscription][tf_core::Subscription] to any [Platform].
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum AnySubscription {
    #[cfg(feature = "youtube")]
    Youtube(tf_yt::YTSubscription),
    #[cfg(feature = "peertube")]
    Peertube(tf_pt::PTSubscription),
    // -- Add new value here.
    #[cfg(test)]
    Test(tf_test::TestSubscription),
}

impl AnySubscription {
    /// Gives the [Platform] of the [AnySubscription].
    pub fn platform(&self) -> Platform {
        match &self {
            #[cfg(feature = "youtube")]
            AnySubscription::Youtube(_) => Platform::Youtube,
            #[cfg(feature = "peertube")]
            AnySubscription::Peertube(_) => Platform::Peertube,
            // -- Add new case here.
            #[cfg(test)]
            AnySubscription::Test(_) => Platform::Test,
        }
    }
}

impl Subscription for AnySubscription {
    type Video = crate::AnyVideo;
    fn name(&self) -> Option<String> {
        match &self {
            #[cfg(feature = "youtube")]
            AnySubscription::Youtube(s) => s.name(),
            #[cfg(feature = "peertube")]
            AnySubscription::Peertube(s) => s.name(),
            // -- Add new case here.
            #[cfg(test)]
            AnySubscription::Test(s) => s.name(),
        }
    }
}

impl std::fmt::Display for AnySubscription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            #[cfg(feature = "youtube")]
            AnySubscription::Youtube(s) => write!(f, "{}", s),
            #[cfg(feature = "peertube")]
            AnySubscription::Peertube(s) => write!(f, "{}", s),
            // -- Add new case here.
            #[cfg(test)]
            AnySubscription::Test(s) => write!(f, "{}", s),
        }
    }
}

impl TryFrom<Vec<String>> for AnySubscription {
    // TODO: Error handling
    type Error = ();

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        let platform = value.get(0).map(|p| Platform::from_str(p.as_str()));
        match platform {
            #[cfg(feature = "youtube")]
            Some(Ok(Platform::Youtube)) => {
                let id = value.get(1);
                match id {
                    Some(id) => Ok(tf_yt::YTSubscription::new(id).into()),
                    _ => Err(()),
                }
            }
            #[cfg(feature = "peertube")]
            Some(Ok(Platform::Peertube)) => {
                let id = value.get(1);
                let base_url = value.get(2);
                match (id, base_url) {
                    (Some(id), Some(base_url)) => {
                        Ok(tf_pt::PTSubscription::new(base_url, id).into())
                    }
                    _ => Err(()),
                }
            }
            // -- Add new case here.
            #[cfg(test)]
            Some(Ok(Platform::Test)) => {
                let id = value.get(1);
                match id {
                    Some(id) => Ok(tf_test::TestSubscription::new(id).into()),
                    _ => Err(()),
                }
            }
            _ => Err(()),
        }
    }
}

impl From<AnySubscription> for Vec<String> {
    fn from(sub: AnySubscription) -> Self {
        let mut result = vec![sub.platform().into()];
        match sub {
            #[cfg(feature = "youtube")]
            AnySubscription::Youtube(s) => result.push(s.id()),
            #[cfg(feature = "peertube")]
            AnySubscription::Peertube(s) => {
                result.push(s.id());
                result.push(s.base_url());
            }
            // -- Add new case here
            #[cfg(test)]
            AnySubscription::Test(s) => result.push(s.name().unwrap()),
        }

        result
    }
}

#[cfg(feature = "youtube")]
impl From<tf_yt::YTSubscription> for AnySubscription {
    fn from(s: tf_yt::YTSubscription) -> Self {
        AnySubscription::Youtube(s)
    }
}

#[cfg(feature = "peertube")]
impl From<tf_pt::PTSubscription> for AnySubscription {
    fn from(s: tf_pt::PTSubscription) -> Self {
        AnySubscription::Peertube(s)
    }
}

// -- Add new conversion here

#[cfg(test)]
impl From<tf_test::TestSubscription> for AnySubscription {
    fn from(s: tf_test::TestSubscription) -> Self {
        AnySubscription::Test(s)
    }
}

/// The [Platform] where the [AnySubscription] is from.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Platform {
    #[cfg(feature = "youtube")]
    Youtube,
    #[cfg(feature = "peertube")]
    Peertube,
    // -- Add new value here.
    #[cfg(test)]
    Test,
}

impl FromStr for Platform {
    // TODO: Error handling
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            #[cfg(feature = "youtube")]
            "youtube" => Ok(Platform::Youtube),
            #[cfg(feature = "peertube")]
            "peertube" => Ok(Platform::Peertube),
            // -- Add new case here.
            #[cfg(test)]
            "test" => Ok(Platform::Test),
            _ => Err(()),
        }
    }
}

impl From<Platform> for String {
    fn from(p: Platform) -> Self {
        match p {
            #[cfg(feature = "youtube")]
            Platform::Youtube => "youtube".to_owned(),
            #[cfg(feature = "peertube")]
            Platform::Peertube => "peertube".to_owned(),
            #[cfg(test)]
            Platform::Test => "test".to_owned(),
        }
    }
}

#[cfg(test)]
mod test {
    use std::convert::TryInto;

    use super::*;
    use tf_test::TestSubscription;
    #[cfg(feature = "youtube")]
    use tf_yt::YTSubscription;

    #[test]
    #[cfg(feature = "youtube")]
    fn anysubscription_conversion_youtube() {
        let row = vec!["youtube".to_string(), "abcdef".to_string()];
        let subscription_res: Result<AnySubscription, ()> = row.try_into();
        assert!(subscription_res.is_ok());

        let subscription = subscription_res.unwrap();
        assert_eq!(subscription.platform(), Platform::Youtube);
        assert_eq!(subscription.to_string(), "abcdef");
    }

    #[test]
    #[cfg(feature = "youtube")]
    fn anysubscription_conversion_youtube_back() {
        let row = vec!["youtube".to_string(), "abcdef".to_string()];
        let subscription: AnySubscription = YTSubscription::new("abcdef").into();

        assert_eq!(Vec::<String>::from(subscription), row);
    }

    #[test]
    fn anysubscription_conversion_test() {
        let row = vec!["test".to_string(), "abcdef".to_string()];
        let subscription_res: Result<AnySubscription, ()> = row.try_into();
        assert!(subscription_res.is_ok());

        let subscription = subscription_res.unwrap();
        assert_eq!(subscription.platform(), Platform::Test);
        assert_eq!(subscription.to_string(), "abcdef");
    }

    #[test]
    fn anysubscription_conversion_test_back() {
        let row = vec!["test".to_string(), "abcdef".to_string()];
        let subscription: AnySubscription = TestSubscription::new("abcdef").into();

        assert_eq!(Vec::<String>::from(subscription), row);
    }

    #[test]
    fn anysubscription_conversion_fail_no_platform() {
        let row = vec!["thiswillfail".to_string(), "abcdef".to_string()];
        let subscription_res: Result<AnySubscription, ()> = row.try_into();
        assert!(subscription_res.is_err());
    }

    #[test]
    #[cfg(feature = "youtube")]
    fn anysubscription_conversion_fail_to_short() {
        let row = vec!["youtube".to_string()];
        let subscription_res: Result<AnySubscription, ()> = row.try_into();
        assert!(subscription_res.is_err());
    }
}
