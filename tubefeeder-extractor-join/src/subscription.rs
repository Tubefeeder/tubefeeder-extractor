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

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum AnySubscription {
    #[cfg(feature = "youtube")]
    Youtube(tf_yt::YTSubscription),
    #[cfg(feature = "testPlatform")]
    Test(tf_test::TestSubscription),
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
