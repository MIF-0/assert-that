assert-that crate, which will help you to create better tests

Example:
    `NumericAssert::assert_that(actual(-1)).is_equal().to(actual(-1));`

If you need custom description, or you can't implement Display trait)
 `StringAssert::assert_that(actual_with(value, description_fucntion))
 .length()
 .is(expected_with(5, description_fucntion));`

If you need custom match:

`    CustomAssert::assert_that(actual(val1))
.matches_by(|a, b| a.eq(b))
.to(expected(val2));`

It is open source, feel free to submit merge requests or propose improvements.