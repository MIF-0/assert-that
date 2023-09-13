//! ## Usage
//!
//! ``` rust
//!
//!use easy_assert::{expected, expected_vec};
//!use easy_assert::option_assertions::OptionAssert;
//!
//!#[test]
//!pub fn none_is_none() {
//!    OptionAssert::<&str>::assert_that(None).is_none();
//!}
//!
//!#[test]
//!#[should_panic]
//!pub fn some_is_not_none() {
//!    OptionAssert::assert_that(Some("a")).is_none();
//!}
//!
//!#[test]
//!#[should_panic]
//!pub fn does_not_contains_value() {
//!    OptionAssert::assert_that(Some(1))
//!        .contains()
//!        .value()
//!        .matches_by(|a,b| a==b)
//!        .to(expected(2));
//!}
//!
//!
//!#[test]
//!pub fn contains_vec() {
//!    OptionAssert::assert_that(Some(vec!['a','b','c']))
//!        .contains()
//!        .list()
//!        .with_element_matcher(|a,b| a==b)
//!        .is_equal_to(expected_vec(vec!['a','b','c']))
//!        .in_order();
//!}
//!
//!#[test]
//!#[should_panic]
//!pub fn does_not_contains_vec() {
//!    OptionAssert::assert_that(Some(vec!['a','b','c']))
//!        .contains()
//!        .list()
//!        .with_element_matcher(|a,b| a==b)
//!        .is_equal_to(expected_vec(vec!['a','b','d']))
//!        .in_order();
//!}
//!
//!#[test]
//!#[should_panic]
//!pub fn contains_for_none_will_raise_error() {
//!       OptionAssert::<i32>::assert_that(None)
//!        .contains()
//!        .value()
//!        .matches_by(|a,b| a==b)
//!        .to(expected(2));
//!}
//!
//! ```

use crate::assertions::OptionCheck;
use crate::custom_assertions::CustomAssert;
use crate::list_assertions::ListAssert;
use crate::{actual, actual_vec, test_failed};
use core::fmt::Display;

pub struct OptionAssert<T> {
    actual: Option<T>,
}

impl<T: 'static> OptionAssert<T> {
    pub fn assert_that(actual: Option<T>) -> OptionAssert<T> {
        OptionAssert { actual }
    }

    pub fn is_none(&self) {
        OptionCheck::is_none(self)
    }

    pub fn is_some(&self) {
        OptionCheck::is_some(self)
    }

    pub fn contains(self) -> ElementMatcher<T> {
        ElementMatcher {
            actual: self.actual.expect("\n Actual: is None, but should be Some"),
        }
    }
}

impl<T: 'static> OptionCheck for OptionAssert<T> {
    fn is_none(&self) {
        if self.actual.is_some() {
            test_failed("Actual is Some, but should be None");
        }
    }

    fn is_some(&self) {
        if self.actual.is_none() {
            test_failed("Actual: is None, but should be Some");
        }
    }
}

pub struct ElementMatcher<T> {
    actual: T,
}

impl<T> ElementMatcher<Vec<T>>
where
    T: Display + 'static,
{
    pub fn list(self) -> ListAssert<T> {
        ListAssert::assert_that(actual_vec(self.actual))
    }
}

impl<T> ElementMatcher<T>
where
    T: Display + 'static,
{
    pub fn value(self) -> CustomAssert<T> {
        CustomAssert::assert_that(actual(self.actual))
    }
}
