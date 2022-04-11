use assert_that::custom_matcher::CustomAssert;
use assert_that::{actual, actual_with, expected, expected_with};

#[test]
pub fn same_should_be_equal() {
    let value = String::from("a");
    CustomAssert::assert_that(actual(value.clone()))
        .matches_by(|a, b| a.eq(b))
        .to(expected(value.clone()));
}

#[test]
#[should_panic]
pub fn not_equal() {
    let val1 = String::from("a");
    let val2 = String::from("c");

    CustomAssert::assert_that(actual(val1))
        .matches_by(|a, b| a.eq(b))
        .to(expected(val2));
}

#[test]
pub fn same_custom_struct_should_be_equal() {
    let val1 = TestStruct {
        a: 1,
        b: String::from("a"),
        c: false,
    };
    let val2 = TestStruct {
        a: 1,
        b: String::from("a"),
        c: false,
    };

    CustomAssert::assert_that(actual_with(val1, custom_description))
        .matches_by(custom_match)
        .to(expected_with(val2, custom_description));
}

#[test]
#[should_panic]
pub fn different_custom_struct_not_equal() {
    let val1 = TestStruct {
        a: 1,
        b: String::from("a"),
        c: false,
    };
    let val2 = TestStruct {
        a: 1,
        b: String::from("b"),
        c: false,
    };

    CustomAssert::assert_that(actual_with(val1, custom_description))
        .matches_by(custom_match)
        .to(expected_with(val2, custom_description));
}

struct TestStruct {
    a: i32,
    b: String,
    c: bool,
}

fn custom_match(val1: &TestStruct, val2: &TestStruct) -> bool {
    val1.a == val2.a && val1.b.eq(&val2.b) && val1.c == val2.c
}

fn custom_description(value: &TestStruct) -> String {
    format!(
        "TestStruct:\n a = {}\n, b = {}\n, c = {}",
        value.a, value.b, value.c
    )
}
