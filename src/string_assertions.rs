//! ## Usage
//!
//! ``` rust
//! use easy_assert::{actual, expected};
//! use easy_assert::string_assertions::StringAssert;
//!
//!  StringAssert::assert_that(actual(String::from("a")))
//!  .is_equal()
//!  .to(expected(String::from("a")));
//!
//!  StringAssert::assert_that(actual(String::from("a")))
//!  .is_not_equal()
//!  .to(expected(String::from("b")));
//!
//!  StringAssert::assert_that(actual(String::from("abcde")))
//!  .contains(expected(String::from("bcd")));
//!
//!  StringAssert::assert_that(actual(String::from("abcde")))
//!  .length()
//!  .is(expected(5));
//! ```

use crate::assertions::{Contains, Equals, Length, NotEquals};
use crate::{test_failed, Actual, Expected};

pub struct StringAssert {
    actual: Actual<String>,
}

impl StringAssert {
    pub fn assert_that(actual: Actual<String>) -> StringAssert {
        StringAssert { actual }
    }

    pub fn is_equal(self) -> Box<dyn Equals<String>> {
        Box::new(self)
    }

    pub fn length(self) -> Box<dyn Length> {
        Box::new(self)
    }

    pub fn is_not_equal(self) -> Box<dyn NotEquals<String>> {
        Box::new(self)
    }

    pub fn contains(self, expected: Expected<String>) {
        Contains::<String>::contains(&self, expected);
    }
}

impl Equals<String> for StringAssert {
    fn to(&self, expected: Expected<String>) {
        if self.actual.ne(&expected) {
            let error_message = format!(
                "\n Actual: {} \n not equal to \n  Expected: {} \n",
                self.actual, expected
            );
            test_failed(&error_message);
        }
    }
}

impl NotEquals<String> for StringAssert {
    fn to(&self, expected: Expected<String>) {
        if self.actual.eq(&expected) {
            let error_message = format!(
                "\n Actual: {} \n not equal to \n  Expected: {} \n",
                self.actual, expected
            );
            test_failed(&error_message);
        }
    }
}

impl Length for StringAssert {
    fn is(&self, expected: Expected<usize>) {
        if self.actual.value.len() != expected.value {
            let error_message = format!(
                "\n Actual: {} \n length not equal to expected \n {} \n",
                self.actual, expected
            );
            test_failed(&error_message);
        }
    }
}

impl Contains<String> for StringAssert {
    fn contains(&self, expected: Expected<String>) {
        if !self.actual.value.contains(&expected.value) {
            let error_message = format!(
                "\n Actual: {} \n does not contains \n  Expected: {} \n",
                self.actual, expected
            );
            test_failed(&error_message);
        }
    }
}
