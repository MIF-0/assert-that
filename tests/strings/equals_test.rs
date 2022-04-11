use easy_assert::string_assertions::StringAssert;
use easy_assert::{actual, expected};

#[test]
pub fn same_should_be_equal() {
    let value = String::from("a");

    StringAssert::assert_that(actual(value.clone()))
        .is_equal()
        .to(expected(value.clone()));
}

#[test]
#[should_panic]
pub fn not_equal() {
    let val1 = String::from("a");
    let val2 = String::from("c");

    StringAssert::assert_that(actual(val1))
        .is_equal()
        .to(expected(val2));
}

#[test]
#[should_panic]
pub fn same_should_not_be_not_equal() {
    let value = String::from("a");

    StringAssert::assert_that(actual(value.clone()))
        .is_not_equal()
        .to(expected(value.clone()));
}

#[test]
pub fn different_should_be_not_equal() {
    let val1 = String::from("a");
    let val2 = String::from("c");

    StringAssert::assert_that(actual(val1))
        .is_not_equal()
        .to(expected(val2));
}
