use easy_assert::num_assertions::NumericAssert;
use easy_assert::{actual, expected};

#[test]
pub fn positive_numbers_greater_success_case() {
    NumericAssert::assert_that(actual(2))
        .is_greater_or_equal()
        .to(expected(1));
}

#[test]
pub fn positive_numbers_greater_than_negative() {
    NumericAssert::assert_that(actual(2.0))
        .is_greater_or_equal()
        .to(expected(-1.0));
}

#[test]
pub fn negative_numbers_success_case() {
    NumericAssert::assert_that(actual(-3.000001))
        .is_greater_or_equal()
        .to(expected(-3.00002));
}

#[test]
pub fn numbers_are_equals() {
    NumericAssert::assert_that(actual(-3.0))
        .is_greater_or_equal()
        .to(expected(-3.0));
}

#[test]
#[should_panic]
pub fn should_panic_when_expected_smaller() {
    NumericAssert::assert_that(actual(1))
        .is_greater_or_equal()
        .to(expected(5));
}
