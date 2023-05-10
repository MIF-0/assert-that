use easy_assert::bool_assertions::BooleanAssert;

#[test]
pub fn true_is_true() {
    BooleanAssert::assert_that(true).is_true();
}

#[test]
pub fn false_is_false() {
    BooleanAssert::assert_that(false).is_false();
}

#[test]
#[should_panic]
pub fn false_is_not_true() {
    BooleanAssert::assert_that(false).is_true();
}

#[test]
#[should_panic]
pub fn true_is_not_false() {
    BooleanAssert::assert_that(true).is_false();
}
