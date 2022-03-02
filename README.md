assert-that crate, which will help you to create better tests

Example:
    `NumericAssert::assert_that(Actual::create_for(-1)).is_equal().to(Actual::create_for(-1));`

If you need custom description, or you can't implement Display trait)
 `StringAssert::assert_that(Actual::new(value, description_fucntion))
 .length()
 .is(Expected::new(5, description_fucntion));`

If you need custom match:

`    CustomAssert::assert_that(Actual::create_for(val1))
.matches_by(|a, b| a.eq(b))
.to(Expected::create_for(val2));`