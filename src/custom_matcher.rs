use crate::{Actual, Expected};
use crate::assertions::Matches;

pub struct CustomAssert<T> {
    actual: Actual<T>,
}

impl <T> CustomAssert<T>
where T: 'static {

    pub fn assert_that(actual: Actual<T>) -> CustomAssert<T> {
        CustomAssert{ actual}
    }

    pub fn matches_by(self, matcher: fn(&T, &T)) -> Box<dyn Matches<T>> {
        Box::new(CustomMatcher {
            actual: self.actual,
            matcher
        })
    }
}

struct CustomMatcher<T> {
    actual: Actual<T>,
    matcher: fn(&T, &T),
}

impl<T> Matches<T> for CustomMatcher<T> {
    fn to(self, expected: Expected<T>) {
        let success = (matcher)(&self.actual.value, &self.expected.value);
    }
}