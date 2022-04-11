use easy_assert::num_assertions::NumericAssert;
use easy_assert::{actual, expected};

#[test]
pub fn positive_numbers_less_success_case() {
    NumericAssert::assert_that(actual(1))
        .is_less()
        .than(expected(2));
}

#[test]
pub fn negative_number_less_than_positive() {
    NumericAssert::assert_that(actual(-3.0))
        .is_less()
        .than(expected(2.0));
}

#[test]
pub fn negative_numbers_success_case() {
    NumericAssert::assert_that(actual(-3.000002))
        .is_less()
        .than(expected(-3.000001));
}

#[test]
#[should_panic]
pub fn should_panic_when_numbers_are_equal() {
    NumericAssert::assert_that(actual(-3.0))
        .is_less()
        .than(expected(-3.0));
}

#[test]
#[should_panic]
pub fn should_panic_when_actual_greater() {
    NumericAssert::assert_that(actual(1))
        .is_less()
        .than(expected(-1));
}
