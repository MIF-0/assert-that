use assert_that::{actual, expected};
use assert_that::simple_num_assertions::NumericAssert;

#[test]
pub fn positive_numbers_greater_success_case() {
    NumericAssert::assert_that(actual(2)).is_greater().than(expected(1));
}

#[test]
pub fn positive_numbers_greater_than_negative() {
    NumericAssert::assert_that(actual(2.0)).is_greater().than(expected(-1.0));
}

#[test]
pub fn negative_numbers_success_case() {
    NumericAssert::assert_that(actual(-3.000001)).is_greater().than(expected(-3.00002));
}

#[test]
#[should_panic]
pub fn should_panic_when_numbers_are_equal() {
    NumericAssert::assert_that(actual(-3.0)).is_greater().than(expected(-3.0));
}

#[test]
#[should_panic]
pub fn should_panic_when_actual_smaller() {
    NumericAssert::assert_that(actual(1)).is_greater().than(expected(5));
}