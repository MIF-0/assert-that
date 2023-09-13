//! ## Usage
//!
//! ``` rust
//! use easy_assert::bool_assertions::BooleanAssert;
//!
//! #[test]
//! pub fn true_is_true() {
//!     BooleanAssert::assert_that(true).is_true();
//! }
//!
//! #[test]
//! pub fn false_is_false() {
//!     BooleanAssert::assert_that(false).is_false();
//! }
//! ```

use crate::assertions::BooleanCheck;
use crate::test_failed;

pub struct BooleanAssert {
    actual: bool,
}

impl BooleanAssert {
    pub fn assert_that(actual: bool) -> BooleanAssert {
        BooleanAssert { actual }
    }

    pub fn is_true(&self) {
        BooleanCheck::is_true(self);
    }

    pub fn is_false(&self) {
        BooleanCheck::is_false(self);
    }
}

impl BooleanCheck for BooleanAssert {
    fn is_true(&self) {
        if !self.actual {
            let error_message = format!("Actual is: {}, but should be TRUE", self.actual);
            test_failed(&error_message);
        }
    }

    fn is_false(&self) {
        if self.actual {
            let error_message = format!("Actual is: {}, but should be FALSE", self.actual);
            test_failed(&error_message);
        }
    }
}
