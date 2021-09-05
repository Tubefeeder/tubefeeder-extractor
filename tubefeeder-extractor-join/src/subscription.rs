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

use std::convert::{TryFrom, TryInto};

#[derive(Clone, PartialEq, Eq, Hash, Debug, Ord, PartialOrd)]
pub enum AnySubscription {
    #[cfg(feature = "youtube")]
    Youtube(tf_yt::YTSubscription),
    #[cfg(feature = "testPlatform")]
    Test(tf_test::TestSubscription),
}

impl AnySubscription {
    pub fn platform(&self) -> Platform {
        match &self {
            #[cfg(feature = "youtube")]
            AnySubscription::Youtube(_) => Platform::Youtube,
            #[cfg(feature = "testPlatform")]
            AnySubscription::Test(_) => Platform::Test,
        }
    }
}

impl std::fmt::Display for AnySubscription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            #[cfg(feature = "youtube")]
            AnySubscription::Youtube(s) => write!(f, "{}", s),
            #[cfg(feature = "testPlatform")]
            AnySubscription::Test(s) => write!(f, "{}", s),
        }
    }
}

impl TryFrom<&[&str]> for AnySubscription {
    // TODO: Error handling
    type Error = ();

    fn try_from(value: &[&str]) -> Result<Self, Self::Error> {
        let platform = value.get(0).map(|&p| p.try_into());
        match platform {
            #[cfg(feature = "youtube")]
            Some(Ok(Platform::Youtube)) => {
                let id = value.get(1);
                match id {
                    Some(id) => Ok(tf_yt::YTSubscription::new(id).into()),
                    _ => Err(()),
                }
            }
            #[cfg(feature = "testPlatform")]
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
            #[cfg(feature = "testPlatform")]
            AnySubscription::Test(s) => result.push(s.name()),
        }

        return result;
    }
}

#[cfg(feature = "youtube")]
impl From<tf_yt::YTSubscription> for AnySubscription {
    fn from(s: tf_yt::YTSubscription) -> Self {
        AnySubscription::Youtube(s)
    }
}

#[cfg(feature = "testPlatform")]
impl From<tf_test::TestSubscription> for AnySubscription {
    fn from(s: tf_test::TestSubscription) -> Self {
        AnySubscription::Test(s)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Platform {
    #[cfg(feature = "youtube")]
    Youtube,
    #[cfg(feature = "testPlatform")]
    Test,
}

impl TryFrom<&str> for Platform {
    // TODO: Error handling
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            #[cfg(feature = "youtube")]
            "youtube" => Ok(Platform::Youtube),
            #[cfg(feature = "testPlatform")]
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
            #[cfg(feature = "testPlatform")]
            Platform::Test => "testPlatform".to_owned(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use tf_yt::YTSubscription;

    #[test]
    #[cfg(feature = "youtube")]
    fn anysubscription_conversion_youtube() {
        let row = vec!["youtube", "abcdef"];
        let subscription_res: Result<AnySubscription, ()> = row.as_slice().try_into();
        assert!(subscription_res.is_ok());

        let subscription = subscription_res.unwrap();
        assert_eq!(subscription.platform(), Platform::Youtube);
        assert_eq!(subscription.to_string(), "abcdef");
    }

    #[test]
    #[cfg(feature = "youtube")]
    fn anysubscription_conversion_youtube_back() {
        let row = vec!["youtube", "abcdef"];
        let subscription: AnySubscription = YTSubscription::new("abcdef").into();

        assert_eq!(Vec::<String>::from(subscription), row);
    }

    #[test]
    #[cfg(feature = "testPlatform")]
    fn anysubscription_conversion_test() {
        let row = vec!["test", "abcdef"];
        let subscription_res: Result<AnySubscription, ()> = row.as_slice().try_into();
        assert!(subscription_res.is_ok());

        let subscription = subscription_res.unwrap();
        assert_eq!(subscription.platform(), Platform::Test);
        assert_eq!(subscription.to_string(), "abcdef");
    }

    #[test]
    #[cfg(feature = "testPlatform")]
    fn anysubscription_conversion_test_back() {
        let row = vec!["test", "abcdef"];
        let subscription: AnySubscription = TestSubscription::new("abcdef").into();

        assert_eq!(Vec::<String>::from(subscription), row);
    }

    #[test]
    fn anysubscription_conversion_fail_no_platform() {
        let row = vec!["thiswillfail", "abcdef"];
        let subscription_res: Result<AnySubscription, ()> = row.as_slice().try_into();
        assert!(subscription_res.is_err());
    }

    #[test]
    #[cfg(feature = "youtube")]
    fn anysubscription_conversion_fail_to_short() {
        let row = vec!["youtube"];
        let subscription_res: Result<AnySubscription, ()> = row.as_slice().try_into();
        assert!(subscription_res.is_err());
    }
}
