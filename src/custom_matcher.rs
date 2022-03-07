use crate::{Actual, Expected};
use crate::assertions::Matches;

pub struct CustomAssert<T> {
    actual: Actual<T>,
}

impl<T> CustomAssert<T>
    where T: 'static {
    pub fn assert_that(actual: Actual<T>) -> CustomAssert<T> {
        CustomAssert { actual }
    }

    pub fn matches_by(self, matcher: fn(&T, &T) -> bool) -> Box<dyn Matches<T>> {
        Box::new(CustomMatcher {
            actual: self.actual,
            matcher,
        })
    }
}

struct CustomMatcher<T> {
    actual: Actual<T>,
    matcher: fn(&T, &T) -> bool,
}

impl<T> Matches<T> for CustomMatcher<T> {
    fn to(&self, expected: Expected<T>) {
        let success = (self.matcher)(&self.actual.value, &expected.value);
        if !success {
            panic!("\n Actual: {} \n not matches with \n {} \n", self.actual, expected);
        }
    }
}