use assert_that::{Actual, Expected};
use assert_that::simple_nums::NumericAssert;

#[test]
pub fn positive_numbers_less_success_case() {
    NumericAssert::assert_that(Actual::create_for(1)).is_less_or_equal().to(Expected::create_for(2));
}

#[test]
pub fn negative_number_less_than_positive() {
    NumericAssert::assert_that(Actual::create_for(-3.0)).is_less_or_equal().to(Expected::create_for(2.0));
}

#[test]
pub fn negative_numbers_success_case() {
    NumericAssert::assert_that(Actual::create_for(-3.000002)).is_less_or_equal().to(Expected::create_for(-3.000001));
}

#[test]
pub fn should_pass_when_equal() {
    NumericAssert::assert_that(Actual::create_for(-3.0)).is_less_or_equal().to(Expected::create_for(-3.0));
}

#[test]
#[should_panic]
pub fn should_panic_when_actual_greater() {
    NumericAssert::assert_that(Actual::create_for(1)).is_less_or_equal().to(Expected::create_for(-1));
}