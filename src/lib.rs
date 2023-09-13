//! ## Description:
//! easy-assert crate, which will help you to create more readable tests
//! For now it can help you assert simple numeric values, strings, vecs
//! and you can do your own custom assertions
//! Everything divided by modules. Check each module for usage example.

extern crate alloc;
extern crate core;
extern crate num_traits;

use core::fmt::Display;

pub mod bool_assertions;
pub mod custom_assertions;
pub mod list_assertions;
pub mod num_assertions;
pub mod option_assertions;
pub mod string_assertions;

mod assertions;
mod values;

/// Struct to wrap your value in order to have custom description if needed
/// and never think about where to put actual or expected value.
pub struct Actual<T> {
    value: T,
    description_func: fn(&T) -> String,
}

pub type Expected<T> = Actual<T>;

/// Creates a new Actual wrapper. Value should implement Display trait.
/// # Examples
///
/// Basic usage:
///
/// ``` rust
/// use easy_assert::actual;
/// let actual = actual(1);
/// ```
///
/// Test usage:
///
/// ``` rust
/// use easy_assert::{actual, expected};
/// use easy_assert::num_assertions::NumericAssert;
/// NumericAssert::assert_that(actual(1)).is_equal().to(expected(1));
/// ```
pub fn actual<T>(val: T) -> Actual<T>
where
    T: Display,
{
    Actual::create_for(val)
}

/// Creates a new Expected wrapper. Value should implement Display trait.
/// # Examples
///
/// Basic usage:
///
/// ``` rust
/// use easy_assert::expected;
/// let expected = expected(1);
/// ```
///
/// Test usage:
///
/// ``` rust
/// use easy_assert::{actual, expected};
/// use easy_assert::num_assertions::NumericAssert;
/// NumericAssert::assert_that(actual(1)).is_equal().to(expected(1));
/// ```
pub fn expected<T>(val: T) -> Expected<T>
where
    T: Display,
{
    Expected::create_for(val)
}

/// Creates a new Actual wrapper. You should provide your own function to represent value as String.
/// Can be used if you need custom description or value doesn't implement Display trait
/// # Examples
///
/// Basic usage:
///
/// ``` rust
/// use easy_assert::{actual_with};
///
/// fn my_description(value: &i32) -> String {
///     format!("My own description for value {}", value)
/// }
///
/// let actual = actual_with(1, my_description);
/// ```
///
/// Test usage:
///
/// ``` rust
/// use easy_assert::{actual_with, expected_with};
/// use easy_assert::num_assertions::NumericAssert;
///
/// fn my_description(value: &i32) -> String {
///     format!("My own description for value {}", value)
/// }
///
/// NumericAssert::assert_that(actual_with(1, my_description))
/// .is_equal().to(expected_with(1, my_description));
/// ```
pub fn actual_with<T>(val: T, description_func: fn(&T) -> String) -> Actual<T> {
    Actual::new(val, description_func)
}

/// Creates a new Expected wrapper. You should provide your own function to represent value as String.
/// Can be used if you need custom description or value doesn't implement Display trait
/// # Examples
///
/// Basic usage:
///
/// ``` rust
/// use easy_assert::{expected_with};
/// fn my_description(value: &i32) -> String {
///     format!("My own description for value {}", value)
/// }
///
/// let expected = expected_with(1, my_description);
/// ```
///
/// Test usage:
///
/// ``` rust
/// use easy_assert::{actual_with, expected_with};
/// use easy_assert::num_assertions::NumericAssert;
///
/// fn my_description(value: &i32) -> String {
///     format!("My own description for value {}", value)
/// }
///
/// NumericAssert::assert_that(actual_with(1, my_description))
/// .is_equal().to(expected_with(1, my_description));
/// ```
pub fn expected_with<T>(val: T, description_func: fn(&T) -> String) -> Expected<T> {
    Expected::new(val, description_func)
}

/// Convert vec with values into vec of Actual wrappers. You should provide your own function to represent value as String.
/// # Examples
///
/// Basic usage:
///
/// ``` rust
/// use easy_assert::actual_vec_with;
/// fn my_description(value: &i32) -> String {
///     format!("My own description for value {}", value)
/// }
///
/// let actual = actual_vec_with(vec![1, 2, 3], my_description);
/// ```
///
/// Test usage:
///
/// ``` rust
///
/// use easy_assert::{actual_vec_with, expected_vec_with};
/// use easy_assert::list_assertions::ListAssert;
///
/// fn my_description(value: &i32) -> String {
///     format!("My own description for value {}", value)
/// }
///
/// ListAssert::assert_that(actual_vec_with(vec![1, 2, 3], my_description))
///  .with_element_matcher(|a, b| a.eq(b))
///  .is_equal_to(expected_vec_with(vec![3, 2, 1], my_description))
///  .in_any_order()
/// ```
pub fn actual_vec_with<T>(values: Vec<T>, description_func: fn(&T) -> String) -> Vec<Actual<T>> {
    let mut result: Vec<Actual<T>> = Vec::new();
    for value in values {
        result.push(Actual::new(value, description_func));
    }

    result
}

/// Convert vec with values into vec of Expected wrappers. You should provide your own function to represent value as String.
/// # Examples
///
/// Basic usage:
///
/// ``` rust
/// use easy_assert::expected_vec_with;
/// fn my_description(value: &i32) -> String {
///     format!("My own description for value {}", value)
/// }
///
/// let expected = expected_vec_with(vec![1, 2, 3], my_description);
/// ```
///
/// Test usage:
///
/// ``` rust
///
/// use easy_assert::{actual_vec_with, expected_vec_with};
/// use easy_assert::list_assertions::ListAssert;
///
/// fn my_description(value: &i32) -> String {
///     format!("My own description for value {}", value)
/// }
///
/// ListAssert::assert_that(actual_vec_with(vec![1, 2, 3], my_description))
///  .with_element_matcher(|a, b| a.eq(b))
///  .is_equal_to(expected_vec_with(vec![3, 2, 1], my_description))
///  .in_any_order()
/// ```
pub fn expected_vec_with<T>(
    values: Vec<T>,
    description_func: fn(&T) -> String,
) -> Vec<Expected<T>> {
    let mut result: Vec<Actual<T>> = Vec::new();
    for value in values {
        result.push(Expected::new(value, description_func));
    }

    result
}

/// Convert vec with values into vec of Actual wrappers. Value should implement Display trait.
/// # Examples
///
/// Basic usage:
///
/// ``` rust
/// use easy_assert::actual_vec;
/// let actual = actual_vec(vec![1, 2, 3]);
/// ```
///
/// Test usage:
///
/// ``` rust
///
/// use easy_assert::{actual_vec, expected_vec};
/// use easy_assert::list_assertions::ListAssert;
/// ListAssert::assert_that(actual_vec(vec![1, 2, 3]))
///  .with_element_matcher(|a, b| a.eq(b))
///  .is_equal_to(expected_vec(vec![3, 2, 1]))
///  .in_any_order()
/// ```
pub fn actual_vec<T>(values: Vec<T>) -> Vec<Actual<T>>
where
    T: Display,
{
    let mut result: Vec<Actual<T>> = Vec::new();
    for value in values {
        result.push(Actual::create_for(value));
    }

    result
}

/// Convert vec with values into vec of Expected wrappers. Value should implement Display trait.
/// # Examples
///
/// Basic usage:
///
/// ``` rust
/// use easy_assert::expected_vec;
/// let expected = expected_vec(vec![1, 2, 3]);
/// ```
///
/// Test usage:
///
/// ``` rust
///
/// use easy_assert::{actual_vec, expected_vec};
/// use easy_assert::list_assertions::ListAssert;
/// ListAssert::assert_that(actual_vec(vec![1, 2, 3]))
///  .with_element_matcher(|a, b| a.eq(b))
///  .is_equal_to(expected_vec(vec![3, 2, 1]))
///  .in_any_order()
/// ```
pub fn expected_vec<T>(values: Vec<T>) -> Vec<Expected<T>>
where
    T: Display,
{
    let mut result: Vec<Actual<T>> = Vec::new();
    for value in values {
        result.push(Expected::create_for(value));
    }

    result
}

fn test_failed(error_message: &str) {
    let addition = "==================================================";
    panic!("\n\n\n{}\n{}\n{}\n\n\n", addition, error_message, addition)
}
