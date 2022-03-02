use assert_that::{Actual, Expected};
use assert_that::string_assertion::StringAssert;

#[test]
pub fn same_should_be_equal() {
    let value = String::from("a");

    StringAssert::assert_that(Actual::create_for(value.clone())).is_equal().to(Expected::create_for(value.clone()));
}

#[test]
#[should_panic]
pub fn not_equal() {
    let val1 = String::from("a");
    let val2 = String::from("c");

    StringAssert::assert_that(Actual::create_for(val1)).is_equal().to(Expected::create_for(val2));
}

#[test]
#[should_panic]
pub fn same_should_not_be_not_equal() {
    let value = String::from("a");

    StringAssert::assert_that(Actual::create_for(value.clone())).is_not_equal().to(Expected::create_for(value.clone()));
}

#[test]
pub fn different_should_be_not_equal() {
    let val1 = String::from("a");
    let val2 = String::from("c");

    StringAssert::assert_that(Actual::create_for(val1)).is_not_equal().to(Expected::create_for(val2));
}