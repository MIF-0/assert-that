use assert_that::numeric::simple_nums::NumericAssert;

#[test]
pub fn positive_numbers_should_be_equal() {
    NumericAssert::assert_that(1).is_equal().to(1);
}

#[test]
pub fn negative_numbers_should_be_equal() {
    NumericAssert::assert_that(-1).is_equal().to(-1);
}

#[test]
pub fn float_numbers_should_be_equal() {
    NumericAssert::assert_that(2.3).is_equal().to(2.3);
}

#[test]
pub fn big_numbers_should_be_equal() {
    NumericAssert::assert_that(u64::MAX.clone()).is_equal().to(u64::MAX.clone());
}

#[test]
#[should_panic]
pub fn numbers_not_equal() {
    NumericAssert::assert_that(-1).is_equal().to(1);
}