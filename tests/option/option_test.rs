use easy_assert::option_assertions::OptionAssert;
use easy_assert::{expected, expected_vec};

#[test]
pub fn none_is_none() {
    OptionAssert::<&str>::assert_that(None).is_none();
}

#[test]
#[should_panic]
pub fn none_is_not_some() {
    OptionAssert::<&str>::assert_that(None).is_some();
}

#[test]
pub fn some_is_some() {
    OptionAssert::assert_that(Some("a")).is_some();
}

#[test]
#[should_panic]
pub fn some_is_not_none() {
    OptionAssert::assert_that(Some("a")).is_none();
}

#[test]
pub fn contains_value() {
    OptionAssert::assert_that(Some("aaa"))
        .contains()
        .value()
        .matches_by(|a, b| a == b)
        .to(expected("aaa"));
}

#[test]
#[should_panic]
pub fn does_not_contains_value() {
    OptionAssert::assert_that(Some(1))
        .contains()
        .value()
        .matches_by(|a, b| a == b)
        .to(expected(2));
}

#[test]
pub fn contains_vec() {
    OptionAssert::assert_that(Some(vec!['a', 'b', 'c']))
        .contains()
        .list()
        .with_element_matcher(|a, b| a == b)
        .is_equal_to(expected_vec(vec!['a', 'b', 'c']))
        .in_order();
}

#[test]
#[should_panic]
pub fn does_not_contains_vec() {
    OptionAssert::assert_that(Some(vec!['a', 'b', 'c']))
        .contains()
        .list()
        .with_element_matcher(|a, b| a == b)
        .is_equal_to(expected_vec(vec!['a', 'b', 'd']))
        .in_order();
}

#[test]
#[should_panic]
pub fn contains_for_none_will_raise_error() {
    OptionAssert::<i32>::assert_that(None)
        .contains()
        .value()
        .matches_by(|a, b| a == b)
        .to(expected(2));
}
