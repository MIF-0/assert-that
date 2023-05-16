easy-assert crate, which will help you to create better tests

Add dependency to your `Cargo.toml`
```
[dev-dependencies]
assert-that = "0.1.5"
```


Example:
    `NumericAssert::assert_that(actual(-1)).is_equal().to(expected(-1));`

If you need custom description, or you can't implement Display trait)
 `StringAssert::assert_that(actual_with(value, description_fucntion))
 .length()
 .is(expected_with(5, description_fucntion));`

If you need custom match:

`    CustomAssert::assert_that(actual(val1))
.matches_by(|a, b| a.eq(b))
.to(expected(val2));`

Available assertions:
`NumericAssert -> is_equal, is_not_equal, is_less, is_less_or_equal, is_greater, is_greater_or_equal`

`StringAssert -> is_equal, is_not_equal, length, contains`

`BooleanAssert -> is_true, is_false`

`OptionAssert -> is_none, is_some, contains` for now only with custom assertion or with list assertion

```
ListAssert -> 
    length;
    with_element_matcher ->
        is_equal_to ->
            in_any_order;
            in_order
        contains ->
            in_any_order;
            in_exact_order; [A,B,C,A,D].contains([B,C]).in_exact_order() = true/[A,B,C,A,D].contains([B,D]).in_exact_order() = false
            just_in_order; [A,B,C,A,D].contains([B,C]).in_exact_order() = true/[A,B,C,A,D].contains([B,D]).in_exact_order() = true
```
`CustomAssert -> matches_by`

It is open source, feel free to submit merge requests or propose improvements.