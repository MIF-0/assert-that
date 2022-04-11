use assert_that::string_assertions::StringAssert;
use assert_that::{actual, expected};

#[test]
pub fn correct_length() {
    let value = String::from("abcde");

    StringAssert::assert_that(actual(value))
        .length()
        .is(expected(5));
}

#[test]
#[should_panic]
pub fn wrong_lenth() {
    let value = String::from("abcdef");

    StringAssert::assert_that(actual(value))
        .length()
        .is(expected(5));
}
