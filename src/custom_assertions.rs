//! ## Usage
//!
//! ``` rust
//! use easy_assert::{actual_with, expected_with};
//! use easy_assert::custom_assertions::CustomAssert;
//!
//! struct TestStruct {
//!     a: i32,
//!     b: String,
//!     c: bool,
//! }
//!
//! fn custom_match(val1: &TestStruct, val2: &TestStruct) -> bool {
//!     val1.a == val2.a && val1.b.eq(&val2.b) && val1.c == val2.c
//! }
//!
//! fn custom_description(value: &TestStruct) -> String {
//!     format!(
//!         "TestStruct:\n a = {}\n, b = {}\n, c = {}",
//!         value.a, value.b, value.c
//!     )
//! }
//!
//! #[test]
//! fn my_test() {
//!     let val1 = TestStruct {
//!         a: 1,
//!         b: String::from("a"),
//!         c: false,
//!     };
//!     let val2 = TestStruct {
//!         a: 1,
//!         b: String::from("a"),
//!         c: false,
//!     };
//!
//!     CustomAssert::assert_that(actual_with(val1, custom_description))
//!         .matches_by(custom_match)
//!         .to(expected_with(val2, custom_description));
//! }
//! ```

use crate::assertions::Matches;
use crate::{Actual, Expected};

pub struct CustomAssert<T> {
    actual: Actual<T>,
}

impl<T> CustomAssert<T>
where
    T: 'static,
{
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
            panic!(
                "\n Actual: {} \n not matches with \n {} \n",
                self.actual, expected
            );
        }
    }
}
