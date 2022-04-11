use assert_that::list_assertions::ListAssert;
use assert_that::{actual_vec, expected_vec};

#[test]
pub fn same_should_contains_in_any_order() {
    ListAssert::assert_that(actual_vec(vec![
        String::from("a"),
        String::from("b"),
        String::from("c"),
    ]))
    .with_element_matcher(|a, b| a.eq(b))
    .contains(expected_vec(vec![
        String::from("a"),
        String::from("b"),
        String::from("c"),
    ]))
    .in_any_order()
}

#[test]
pub fn same_should_contains_in_exact_order() {
    ListAssert::assert_that(actual_vec(vec![
        String::from("a"),
        String::from("b"),
        String::from("c"),
    ]))
        .with_element_matcher(|a, b| a.eq(b))
        .contains(expected_vec(vec![
            String::from("a"),
            String::from("b"),
            String::from("c"),
        ]))
        .in_exact_order()
}

#[test]
pub fn same_should_contains_just_in_order() {
    ListAssert::assert_that(actual_vec(vec![
        String::from("a"),
        String::from("b"),
        String::from("c"),
    ]))
        .with_element_matcher(|a, b| a.eq(b))
        .contains(expected_vec(vec![
            String::from("a"),
            String::from("b"),
            String::from("c"),
        ]))
        .just_in_order()
}

#[test]
pub fn bigger_should_contains_in_any_order() {
    ListAssert::assert_that(actual_vec(vec![
        String::from("a"),
        String::from("b"),
        String::from("c"),
        String::from("d"),
    ]))
        .with_element_matcher(|a, b| a.eq(b))
        .contains(expected_vec(vec![
            String::from("a"),
            String::from("b"),
            String::from("c"),
        ]))
        .in_any_order()
}

#[test]
pub fn bigger_should_contains_in_exact_order() {
    ListAssert::assert_that(actual_vec(vec![
        String::from("a"),
        String::from("b"),
        String::from("c"),
        String::from("d"),
    ]))
        .with_element_matcher(|a, b| a.eq(b))
        .contains(expected_vec(vec![
            String::from("a"),
            String::from("b"),
            String::from("c"),
        ]))
        .in_exact_order()
}

#[test]
pub fn bigger_should_contains_just_in_order() {
    ListAssert::assert_that(actual_vec(vec![
        String::from("a"),
        String::from("b"),
        String::from("c"),
        String::from("d"),
    ]))
        .with_element_matcher(|a, b| a.eq(b))
        .contains(expected_vec(vec![
            String::from("a"),
            String::from("b"),
            String::from("c"),
        ]))
        .just_in_order()
}

#[test]
#[should_panic]
pub fn smaller_should_not_contains_just_in_order() {
    ListAssert::assert_that(actual_vec(vec![
        String::from("a"),
        String::from("b"),
    ]))
        .with_element_matcher(|a, b| a.eq(b))
        .contains(expected_vec(vec![
            String::from("a"),
            String::from("b"),
            String::from("c"),
        ]))
        .just_in_order()
}

#[test]
#[should_panic]
pub fn smaller_should_not_contains_in_any_order() {
    ListAssert::assert_that(actual_vec(vec![
        String::from("a"),
        String::from("b"),
    ]))
        .with_element_matcher(|a, b| a.eq(b))
        .contains(expected_vec(vec![
            String::from("a"),
            String::from("b"),
            String::from("c"),
        ]))
        .in_any_order()
}

#[test]
#[should_panic]
pub fn smaller_should_not_contains_in_exact_order() {
    ListAssert::assert_that(actual_vec(vec![
        String::from("a"),
        String::from("b"),
    ]))
        .with_element_matcher(|a, b| a.eq(b))
        .contains(expected_vec(vec![
            String::from("a"),
            String::from("b"),
            String::from("c"),
        ]))
        .in_exact_order()
}

#[test]
pub fn different_ordered_should_contains_in_any_order() {
    ListAssert::assert_that(actual_vec(vec![
        String::from("c"),
        String::from("a"),
        String::from("b"),
    ]))
        .with_element_matcher(|a, b| a.eq(b))
        .contains(expected_vec(vec![
            String::from("a"),
            String::from("b"),
            String::from("c"),
        ]))
        .in_any_order()
}

#[test]
#[should_panic]
pub fn different_ordered_should_not_contains_in_exact_order() {
    ListAssert::assert_that(actual_vec(vec![
        String::from("c"),
        String::from("a"),
        String::from("b"),
    ]))
        .with_element_matcher(|a, b| a.eq(b))
        .contains(expected_vec(vec![
            String::from("a"),
            String::from("b"),
            String::from("c"),
        ]))
        .in_exact_order()
}

#[test]
#[should_panic]
pub fn different_ordered_should_not_contains_just_in_order() {
    ListAssert::assert_that(actual_vec(vec![
        String::from("c"),
        String::from("a"),
        String::from("b"),
    ]))
        .with_element_matcher(|a, b| a.eq(b))
        .contains(expected_vec(vec![
            String::from("a"),
            String::from("b"),
            String::from("c"),
        ]))
        .just_in_order()
}

#[test]
pub fn exact_slice_should_contains_in_any_order() {
    ListAssert::assert_that(actual_vec(vec![
        String::from("c"),
        String::from("a"),
        String::from("b"),
        String::from("b"),
        String::from("a"),
        String::from("b"),
        String::from("c"),
        String::from("d"),
    ]))
        .with_element_matcher(|a, b| a.eq(b))
        .contains(expected_vec(vec![
            String::from("a"),
            String::from("b"),
            String::from("c"),
        ]))
        .in_any_order()
}

#[test]
pub fn exact_slice_should_contains_in_exact_order() {
    ListAssert::assert_that(actual_vec(vec![
        String::from("c"),
        String::from("a"),
        String::from("b"),
        String::from("b"),
        String::from("a"),
        String::from("b"),
        String::from("c"),
        String::from("d"),
    ]))
        .with_element_matcher(|a, b| a.eq(b))
        .contains(expected_vec(vec![
            String::from("a"),
            String::from("b"),
            String::from("c"),
        ]))
        .in_exact_order()
}

#[test]
pub fn exact_slice_should_contains_just_in_order() {
    ListAssert::assert_that(actual_vec(vec![
        String::from("c"),
        String::from("a"),
        String::from("b"),
        String::from("b"),
        String::from("a"),
        String::from("b"),
        String::from("c"),
        String::from("d"),
    ]))
        .with_element_matcher(|a, b| a.eq(b))
        .contains(expected_vec(vec![
            String::from("a"),
            String::from("b"),
            String::from("c"),
        ]))
        .just_in_order()
}

#[test]
pub fn same_order_with_gaps_should_contains_in_any_order() {
    ListAssert::assert_that(actual_vec(vec![
        String::from("c"),
        String::from("a"), //
        String::from("d"),
        String::from("g"),
        String::from("j"),
        String::from("b"), //
        String::from("q"),
        String::from("c"), //
    ]))
        .with_element_matcher(|a, b| a.eq(b))
        .contains(expected_vec(vec![
            String::from("a"),
            String::from("b"),
            String::from("c"),
        ]))
        .in_any_order()
}

#[test]
#[should_panic]
pub fn same_order_with_gaps_should_not_contains_in_exact_order() {
    ListAssert::assert_that(actual_vec(vec![
        String::from("c"),
        String::from("a"), //
        String::from("d"),
        String::from("g"),
        String::from("j"),
        String::from("b"), //
        String::from("q"),
        String::from("c"), //
    ]))
        .with_element_matcher(|a, b| a.eq(b))
        .contains(expected_vec(vec![
            String::from("a"),
            String::from("b"),
            String::from("c"),
        ]))
        .in_exact_order()
}

#[test]
pub fn same_order_with_gaps_should_contains_just_in_order() {
    ListAssert::assert_that(actual_vec(vec![
        String::from("c"),
        String::from("a"), //
        String::from("d"),
        String::from("g"),
        String::from("j"),
        String::from("b"), //
        String::from("q"),
        String::from("c"), //
    ]))
        .with_element_matcher(|a, b| a.eq(b))
        .contains(expected_vec(vec![
            String::from("a"),
            String::from("b"),
            String::from("c"),
        ]))
        .just_in_order()
}
