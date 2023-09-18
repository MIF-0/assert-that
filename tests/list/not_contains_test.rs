use easy_assert::list_assertions::ListAssert;
use easy_assert::{actual_vec, expected_vec};

#[test]
#[should_panic]
pub fn same_should_contains_all() {
    ListAssert::assert_that(actual_vec(vec![
        String::from("a"),
        String::from("b"),
        String::from("c"),
    ]))
    .with_element_matcher(|a, b| a.eq(b))
    .does_not_contain(expected_vec(vec![
        String::from("a"),
        String::from("b"),
        String::from("c"),
    ]))
    .all()
}

#[test]
#[should_panic]
pub fn same_should_contains_all_in_any_order() {
    ListAssert::assert_that(actual_vec(vec![
        String::from("a"),
        String::from("b"),
        String::from("c"),
    ]))
    .with_element_matcher(|a, b| a.eq(b))
    .does_not_contain(expected_vec(vec![
        String::from("b"),
        String::from("c"),
        String::from("a"),
    ]))
    .all()
}

#[test]
#[should_panic]
pub fn same_should_contains_all_for_at_lest_one() {
    ListAssert::assert_that(actual_vec(vec![
        String::from("a"),
        String::from("b"),
        String::from("c"),
    ]))
    .with_element_matcher(|a, b| a.eq(b))
    .does_not_contain(expected_vec(vec![
        String::from("a"),
        String::from("b"),
        String::from("c"),
    ]))
    .at_least_one()
}

#[test]
#[should_panic]
pub fn same_should_contains_all_in_any_order_for_at_lest_one() {
    ListAssert::assert_that(actual_vec(vec![
        String::from("a"),
        String::from("b"),
        String::from("c"),
    ]))
    .with_element_matcher(|a, b| a.eq(b))
    .does_not_contain(expected_vec(vec![
        String::from("b"),
        String::from("c"),
        String::from("a"),
    ]))
    .at_least_one()
}

#[test]
#[should_panic]
pub fn bigger_should_contains_all() {
    ListAssert::assert_that(actual_vec(vec![
        String::from("a"),
        String::from("b"),
        String::from("c"),
    ]))
    .with_element_matcher(|a, b| a.eq(b))
    .does_not_contain(expected_vec(vec![String::from("a"), String::from("b")]))
    .all()
}

#[test]
#[should_panic]
pub fn bigger_should_contains_all_in_any_order() {
    ListAssert::assert_that(actual_vec(vec![
        String::from("a"),
        String::from("b"),
        String::from("c"),
    ]))
    .with_element_matcher(|a, b| a.eq(b))
    .does_not_contain(expected_vec(vec![String::from("b"), String::from("c")]))
    .all()
}

#[test]
#[should_panic]
pub fn bigger_should_contains_all_for_at_lest_one() {
    ListAssert::assert_that(actual_vec(vec![
        String::from("a"),
        String::from("b"),
        String::from("c"),
    ]))
    .with_element_matcher(|a, b| a.eq(b))
    .does_not_contain(expected_vec(vec![String::from("a"), String::from("b")]))
    .at_least_one()
}

#[test]
#[should_panic]
pub fn bigger_should_contains_all_in_any_order_for_at_lest_one() {
    ListAssert::assert_that(actual_vec(vec![
        String::from("a"),
        String::from("b"),
        String::from("c"),
    ]))
    .with_element_matcher(|a, b| a.eq(b))
    .does_not_contain(expected_vec(vec![String::from("b"), String::from("c")]))
    .at_least_one()
}

#[test]
#[should_panic]
pub fn does_not_contains_all_failing_if_one_exist() {
    ListAssert::assert_that(actual_vec(vec![
        String::from("a"),
        String::from("b"),
        String::from("c"),
    ]))
    .with_element_matcher(|a, b| a.eq(b))
    .does_not_contain(expected_vec(vec![
        String::from("a"),
        String::from("d"),
        String::from("e"),
    ]))
    .all()
}

#[test]
pub fn does_not_contains_all_succeed_if_all_diff() {
    ListAssert::assert_that(actual_vec(vec![
        String::from("a"),
        String::from("b"),
        String::from("c"),
    ]))
    .with_element_matcher(|a, b| a.eq(b))
    .does_not_contain(expected_vec(vec![String::from("d"), String::from("e")]))
    .all()
}

#[test]
pub fn does_not_contains_at_least_one_pass_even_if_just_one_differ() {
    ListAssert::assert_that(actual_vec(vec![
        String::from("a"),
        String::from("b"),
        String::from("c"),
    ]))
    .with_element_matcher(|a, b| a.eq(b))
    .does_not_contain(expected_vec(vec![String::from("a"), String::from("d")]))
    .at_least_one()
}

#[test]
pub fn does_not_contains_at_least_one_pass_even_if_all_differ() {
    ListAssert::assert_that(actual_vec(vec![
        String::from("a"),
        String::from("b"),
        String::from("c"),
    ]))
    .with_element_matcher(|a, b| a.eq(b))
    .does_not_contain(expected_vec(vec![String::from("d"), String::from("e")]))
    .at_least_one()
}
