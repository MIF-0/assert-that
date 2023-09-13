//! ## Usage
//!
//! ``` rust
//! use easy_assert::{actual, Actual, expected, Expected};
//! use easy_assert::num_assertions::NumericAssert;
//!
//!  NumericAssert::<i32>::assert_that(actual(1))
//!  .is_equal()
//!  .to(expected(1));
//!
//!  NumericAssert::<i32>::assert_that(actual(-1))
//!  .is_not_equal()
//!  .to(expected(1));
//!
//!  NumericAssert::<i32>::assert_that(actual(2))
//!  .is_greater_or_equal()
//!  .to(expected(1));
//!
//!  NumericAssert::<i32>::assert_that(actual(2))
//!  .is_greater()
//!  .than(expected(1));
//!
//!  NumericAssert::<i32>::assert_that(actual(1))
//!  .is_less_or_equal()
//!  .to(expected(1));
//!
//!  NumericAssert::<i32>::assert_that(actual(1))
//!  .is_less()
//!  .than(expected(3));
//! ```

use crate::{test_failed, Actual, Expected};
use num_traits::Num;

use crate::assertions::{Equals, Greater, GreaterOrEqual, Less, LessOrEqual, NotEquals};

pub struct NumericAssert<T>
where
    T: Num + PartialOrd,
{
    actual: Actual<T>,
}

impl<T> NumericAssert<T>
where
    T: Num + PartialOrd,
    T: 'static,
{
    pub fn assert_that(actual: Actual<T>) -> NumericAssert<T> {
        NumericAssert { actual }
    }

    pub fn is_equal(self) -> Box<dyn Equals<T>> {
        Box::new(self)
    }

    pub fn is_not_equal(self) -> Box<dyn NotEquals<T>> {
        Box::new(self)
    }

    pub fn is_less(self) -> Box<dyn Less<T>> {
        Box::new(self)
    }

    pub fn is_less_or_equal(self) -> Box<dyn LessOrEqual<T>> {
        Box::new(self)
    }

    pub fn is_greater(self) -> Box<dyn Greater<T>> {
        Box::new(self)
    }

    pub fn is_greater_or_equal(self) -> Box<dyn GreaterOrEqual<T>> {
        Box::new(self)
    }
}

impl<T> Equals<T> for NumericAssert<T>
where
    T: Num + PartialOrd,
{
    fn to(&self, expected: Expected<T>) {
        if self.actual.ne(&expected) {
            let error_message = format!(
                "\n Actual: {} \n not equal to expected \n {} \n",
                self.actual, expected
            );
            test_failed(&error_message);
        }
    }
}

impl<T> NotEquals<T> for NumericAssert<T>
where
    T: Num + PartialOrd,
{
    fn to(&self, expected: Expected<T>) {
        if self.actual.eq(&expected) {
            let error_message = format!(
                "\n Actual: {} \n is equal to expected \n {} \n",
                self.actual, expected
            );
            test_failed(&error_message);
        }
    }
}

impl<T> Less<T> for NumericAssert<T>
where
    T: Num + PartialOrd,
{
    fn than(&self, expected: Expected<T>) {
        if self.actual.value >= expected.value {
            let error_message = format!(
                "\n Actual: {} \n is bigger or equal to expected \n {} \n",
                self.actual, expected
            );
            test_failed(&error_message);
        }
    }
}

impl<T> LessOrEqual<T> for NumericAssert<T>
where
    T: Num + PartialOrd,
{
    fn to(&self, expected: Expected<T>) {
        if self.actual.value > expected.value {
            let error_message = format!(
                "\n Actual: {} \n is bigger to expected \n {} \n",
                self.actual, expected
            );
            test_failed(&error_message);
        }
    }
}

impl<T> Greater<T> for NumericAssert<T>
where
    T: Num + PartialOrd,
{
    fn than(&self, expected: Expected<T>) {
        if self.actual.value <= expected.value {
            let error_message = format!(
                "\n Actual: {} \n is smaller or equal to expected \n {} \n",
                self.actual, expected
            );
            test_failed(&error_message);
        }
    }
}

impl<T> GreaterOrEqual<T> for NumericAssert<T>
where
    T: Num + PartialOrd,
{
    fn to(&self, expected: Expected<T>) {
        if self.actual.value < expected.value {
            let error_message = format!(
                "\n Actual: {} \n is smaller to expected \n {} \n",
                self.actual, expected
            );
            test_failed(&error_message);
        }
    }
}
