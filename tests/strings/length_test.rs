use easy_assert::string_assertions::StringAssert;
use easy_assert::{actual, expected};

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
