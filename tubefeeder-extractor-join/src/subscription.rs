#[derive(Clone, PartialEq, Eq, Hash)]
pub enum AnySubscription {
    Youtube(tf_yt::YTSubscription),
    Test(tf_test::TestSubscription),
}

impl From<tf_yt::YTSubscription> for AnySubscription {
    fn from(s: tf_yt::YTSubscription) -> Self {
        AnySubscription::Youtube(s)
    }
}

impl From<tf_test::TestSubscription> for AnySubscription {
    fn from(s: tf_test::TestSubscription) -> Self {
        AnySubscription::Test(s)
    }
}
