use easy_assert::list_assertions::ListAssert;
use easy_assert::{actual_vec, expected_vec};

#[test]
#[should_panic]
pub fn same_should_be_equal_in_order() {
    ListAssert::assert_that(actual_vec(vec![
        String::from("a"),
        String::from("b"),
        String::from("c"),
    ]))
    .with_element_matcher(|a, b| a.eq(b))
    .is_not_equal_to(expected_vec(vec![
        String::from("a"),
        String::from("b"),
        String::from("c"),
    ]))
    .in_order()
}

#[test]
pub fn should_not_be_equal_in_order() {
    ListAssert::assert_that(actual_vec(vec![
        String::from("c"),
        String::from("b"),
        String::from("a"),
    ]))
    .with_element_matcher(|a, b| a.eq(b))
    .is_not_equal_to(expected_vec(vec![
        String::from("a"),
        String::from("b"),
        String::from("c"),
    ]))
    .in_order()
}

#[test]
pub fn not_equal_in_order_for_different_length() {
    ListAssert::assert_that(actual_vec(vec![
        String::from("a"),
        String::from("b"),
        String::from("c"),
        String::from("d"),
    ]))
    .with_element_matcher(|a, b| a.eq(b))
    .is_not_equal_to(expected_vec(vec![
        String::from("a"),
        String::from("b"),
        String::from("c"),
    ]))
    .in_order()
}

#[test]
#[should_panic]
pub fn same_should_be_equal_in_any_order() {
    ListAssert::assert_that(actual_vec(vec![
        String::from("a"),
        String::from("b"),
        String::from("c"),
    ]))
    .with_element_matcher(|a, b| a.eq(b))
    .is_not_equal_to(expected_vec(vec![
        String::from("a"),
        String::from("b"),
        String::from("c"),
    ]))
    .in_any_order()
}

#[test]
#[should_panic]
pub fn should_be_equal_in_any_order() {
    ListAssert::assert_that(actual_vec(vec![1, 2, 3]))
        .with_element_matcher(|a, b| a.eq(b))
        .is_not_equal_to(expected_vec(vec![3, 2, 1]))
        .in_any_order()
}

#[test]
pub fn should_not_be_equal_in_any_order() {
    ListAssert::assert_that(actual_vec(vec![
        String::from("c"),
        String::from("b"),
        String::from("a"),
    ]))
    .with_element_matcher(|a, b| a.eq(b))
    .is_not_equal_to(expected_vec(vec![
        String::from("b"),
        String::from("b"),
        String::from("c"),
    ]))
    .in_any_order()
}

#[test]
pub fn not_equal_in_any_order_for_different_length() {
    ListAssert::assert_that(actual_vec(vec![
        String::from("a"),
        String::from("b"),
        String::from("c"),
        String::from("d"),
    ]))
    .with_element_matcher(|a, b| a.eq(b))
    .is_not_equal_to(expected_vec(vec![
        String::from("a"),
        String::from("b"),
        String::from("c"),
    ]))
    .in_any_order()
}
