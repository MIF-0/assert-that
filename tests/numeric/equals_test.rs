use assert_that::num_assertions::NumericAssert;
use assert_that::{actual, expected};

#[test]
pub fn positive_numbers_should_be_equal() {
    NumericAssert::assert_that(actual(1))
        .is_equal()
        .to(expected(1));
}

#[test]
pub fn negative_numbers_should_be_equal() {
    NumericAssert::assert_that(actual(-1))
        .is_equal()
        .to(actual(-1));
}

#[test]
pub fn float_numbers_should_be_equal() {
    NumericAssert::assert_that(actual(2.3))
        .is_equal()
        .to(actual(2.3));
}

#[test]
pub fn big_numbers_should_be_equal() {
    NumericAssert::assert_that(actual(u64::MAX.clone()))
        .is_equal()
        .to(actual(u64::MAX.clone()));
}

#[test]
#[should_panic]
pub fn numbers_not_equal_but_should() {
    NumericAssert::assert_that(actual(1))
        .is_equal()
        .to(expected(-1));
}

#[test]
pub fn numbers_not_equal_and_should_not() {
    NumericAssert::assert_that(actual(1))
        .is_not_equal()
        .to(expected(-1));
}

#[test]
#[should_panic]
pub fn numbers_equal_but_should_not() {
    NumericAssert::assert_that(actual(-1))
        .is_not_equal()
        .to(expected(-1));
}
