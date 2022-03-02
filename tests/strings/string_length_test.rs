use assert_that::{Actual, Expected};
use assert_that::string_assertion::StringAssert;

#[test]
pub fn correct_length() {
    let value = String::from("abcde");

    StringAssert::assert_that(Actual::create_for(value)).length().is(Expected::create_for(5));
}

#[test]
#[should_panic]
pub fn wrong_lenth() {
    let value = String::from("abcdef");

    StringAssert::assert_that(Actual::create_for(value)).length().is(Expected::create_for(5));
}