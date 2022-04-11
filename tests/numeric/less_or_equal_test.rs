use assert_that::num_assertions::NumericAssert;
use assert_that::{actual, expected};

#[test]
pub fn positive_numbers_less_success_case() {
    NumericAssert::assert_that(actual(1))
        .is_less_or_equal()
        .to(expected(2));
}

#[test]
pub fn negative_number_less_than_positive() {
    NumericAssert::assert_that(actual(-3.0))
        .is_less_or_equal()
        .to(expected(2.0));
}

#[test]
pub fn negative_numbers_success_case() {
    NumericAssert::assert_that(actual(-3.000002))
        .is_less_or_equal()
        .to(expected(-3.000001));
}

#[test]
pub fn should_pass_when_equal() {
    NumericAssert::assert_that(actual(-3.0))
        .is_less_or_equal()
        .to(expected(-3.0));
}

#[test]
#[should_panic]
pub fn should_panic_when_actual_greater() {
    NumericAssert::assert_that(actual(1))
        .is_less_or_equal()
        .to(expected(-1));
}
