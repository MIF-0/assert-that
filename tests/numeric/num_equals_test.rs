use assert_that::{Actual, Expected};
use assert_that::simple_nums::NumericAssert;

#[test]
pub fn positive_numbers_should_be_equal() {
    NumericAssert::assert_that(Actual::create_for(1)).is_equal().to(Expected::create_for(1));
}

#[test]
pub fn negative_numbers_should_be_equal() {
    NumericAssert::assert_that(Actual::create_for(-1)).is_equal().to(Actual::create_for(-1));
}

#[test]
pub fn float_numbers_should_be_equal() {
    NumericAssert::assert_that(Actual::create_for(2.3)).is_equal().to(Actual::create_for(2.3));
}

#[test]
pub fn big_numbers_should_be_equal() {
    NumericAssert::assert_that(Actual::create_for(u64::MAX.clone())).is_equal().to(Actual::create_for(u64::MAX.clone()));
}

#[test]
#[should_panic]
pub fn numbers_not_equal() {
    NumericAssert::assert_that(Actual::create_for(1)).is_equal().to(Expected::create_for(-1));
}