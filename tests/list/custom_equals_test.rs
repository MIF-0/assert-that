use assert_that::list_assertions::ListAssert;
use assert_that::{actual_vec_with, expected_vec_with};

#[test]
pub fn same_should_be_equal_in_order() {
    let obj1 = CustomObject {
        a: String::from("a"),
        b: 1,
        c: true,
    };
    let obj2 = CustomObject {
        a: String::from("b"),
        b: 2,
        c: true,
    };
    let obj3 = CustomObject {
        a: String::from("c"),
        b: 3,
        c: false,
    };
    ListAssert::assert_that(actual_vec_with(
        vec![obj1.clone(), obj2.clone(), obj3.clone()],
        custom_describe,
    ))
    .with_element_matcher(custom_match)
    .is_equal_to(expected_vec_with(vec![obj1, obj2, obj3], custom_describe))
    .in_order()
}

#[test]
#[should_panic]
pub fn should_not_be_equal_in_order() {
    let obj1 = CustomObject {
        a: String::from("a"),
        b: 1,
        c: true,
    };
    let obj2 = CustomObject {
        a: String::from("b"),
        b: 2,
        c: true,
    };
    let obj3 = CustomObject {
        a: String::from("c"),
        b: 3,
        c: false,
    };
    ListAssert::assert_that(actual_vec_with(
        vec![obj1.clone(), obj2.clone(), obj3.clone()],
        custom_describe,
    ))
    .with_element_matcher(custom_match)
    .is_equal_to(expected_vec_with(vec![obj2, obj1, obj3], custom_describe))
    .in_order()
}

#[test]
#[should_panic]
pub fn in_order_should_panic_for_different_length() {
    let obj1 = CustomObject {
        a: String::from("a"),
        b: 1,
        c: true,
    };
    let obj2 = CustomObject {
        a: String::from("b"),
        b: 2,
        c: true,
    };
    let obj3 = CustomObject {
        a: String::from("c"),
        b: 3,
        c: false,
    };
    ListAssert::assert_that(actual_vec_with(
        vec![obj1.clone(), obj2.clone(), obj3.clone(), obj3.clone()],
        custom_describe,
    ))
    .with_element_matcher(custom_match)
    .is_equal_to(expected_vec_with(vec![obj2, obj1, obj3], custom_describe))
    .in_order()
}

#[test]
pub fn same_should_be_equal_in_any_order() {
    let obj1 = CustomObject {
        a: String::from("a"),
        b: 1,
        c: true,
    };
    let obj2 = CustomObject {
        a: String::from("b"),
        b: 2,
        c: true,
    };
    let obj3 = CustomObject {
        a: String::from("c"),
        b: 3,
        c: false,
    };
    ListAssert::assert_that(actual_vec_with(
        vec![obj1.clone(), obj2.clone(), obj3.clone()],
        custom_describe,
    ))
    .with_element_matcher(custom_match)
    .is_equal_to(expected_vec_with(vec![obj1, obj2, obj3], custom_describe))
    .in_any_order()
}

#[test]
pub fn should_be_equal_in_any_order() {
    let obj1 = CustomObject {
        a: String::from("a"),
        b: 1,
        c: true,
    };
    let obj2 = CustomObject {
        a: String::from("b"),
        b: 2,
        c: true,
    };
    let obj3 = CustomObject {
        a: String::from("c"),
        b: 3,
        c: false,
    };
    ListAssert::assert_that(actual_vec_with(
        vec![obj1.clone(), obj2.clone(), obj3.clone()],
        custom_describe,
    ))
    .with_element_matcher(custom_match)
    .is_equal_to(expected_vec_with(vec![obj2, obj1, obj3], custom_describe))
    .in_any_order()
}

#[test]
#[should_panic]
pub fn should_not_be_equal_in_any_order() {
    let obj1 = CustomObject {
        a: String::from("a"),
        b: 1,
        c: true,
    };
    let obj2 = CustomObject {
        a: String::from("b"),
        b: 2,
        c: true,
    };
    let obj3 = CustomObject {
        a: String::from("c"),
        b: 3,
        c: false,
    };
    ListAssert::assert_that(actual_vec_with(
        vec![obj1.clone(), obj2.clone(), obj3.clone()],
        custom_describe,
    ))
    .with_element_matcher(custom_match)
    .is_equal_to(expected_vec_with(
        vec![obj1.clone(), obj1, obj3],
        custom_describe,
    ))
    .in_any_order()
}

#[test]
#[should_panic]
pub fn in_any_order_should_panic_for_different_length() {
    let obj1 = CustomObject {
        a: String::from("a"),
        b: 1,
        c: true,
    };
    let obj2 = CustomObject {
        a: String::from("b"),
        b: 2,
        c: true,
    };
    let obj3 = CustomObject {
        a: String::from("c"),
        b: 3,
        c: false,
    };
    ListAssert::assert_that(actual_vec_with(
        vec![obj1.clone(), obj2.clone(), obj3.clone(), obj3.clone()],
        custom_describe,
    ))
    .with_element_matcher(custom_match)
    .is_equal_to(expected_vec_with(vec![obj2, obj1, obj3], custom_describe))
    .in_any_order()
}

#[derive(Clone)]
struct CustomObject {
    a: String,
    b: u32,
    c: bool,
}

fn custom_match(value1: &CustomObject, value2: &CustomObject) -> bool {
    value1.a.eq(&value2.a) && value1.b == value2.b && value1.c == value2.c
}

fn custom_describe(val: &CustomObject) -> String {
    format!(
        "CustomObject: \n a: {} \n b: {} \n c: {}",
        val.a, val.b, val.c
    )
}
