//! ## Usage
//!
//! ``` rust
//! use easy_assert::{actual_vec, expected_vec};
//! use easy_assert::list_assertions::ListAssert;
//!
//!  ListAssert::assert_that(actual_vec(vec![1, 2, 3]))
//!  .with_element_matcher(|a, b| a.eq(b))
//!  .is_equal_to(expected_vec(vec![3, 2, 1]))
//!  .in_any_order();
//!
//!  ListAssert::assert_that(actual_vec(vec![1, 2, 3]))
//!  .with_element_matcher(|a, b| a.eq(b))
//!  .is_equal_to(expected_vec(vec![1, 2, 3]))
//!  .in_order();
//!
//!  ListAssert::assert_that(actual_vec(vec![2, 3, 1, 2, 3, 4, 5]))
//!  .with_element_matcher(|a, b| a.eq(b))
//!  .contains(expected_vec(vec![2, 3, 4]))
//!  .in_exact_order();
//!
//!  ListAssert::assert_that(actual_vec(vec![2, 1, 3, 5, 4]))
//!  .with_element_matcher(|a, b| a.eq(b))
//!  .contains(expected_vec(vec![2, 3, 4]))
//!  .just_in_order();
//!
//!  ListAssert::assert_that(actual_vec(vec![2, 1, 3, 5, 4]))
//!  .with_element_matcher(|a, b| a.eq(b))
//!  .contains(expected_vec(vec![5, 1, 2]))
//!  .in_any_order();
//! ```
//!
//! ### Custom objects
//!
//! ``` rust
//! use easy_assert::{actual_vec_with, expected_vec_with};
//! use easy_assert::list_assertions::ListAssert;
//!
//! #[derive(Clone)]
//! struct CustomObject {
//!     a: String,
//!     b: u32,
//!     c: bool,
//! }
//!
//! fn custom_match(value1: &CustomObject, value2: &CustomObject) -> bool {
//!     value1.a.eq(&value2.a) && value1.b == value2.b && value1.c == value2.c
//! }
//!
//! fn custom_describe(val: &CustomObject) -> String {
//!     format!(
//!         "CustomObject: \n a: {} \n b: {} \n c: {}",
//!         val.a, val.b, val.c
//!     )
//! }
//!
//! #[test]
//! fn my_test() {
//!     let obj1 = CustomObject {a: String::from("a"),b: 1, c: true};
//!     let obj2 = CustomObject {a: String::from("b"),b: 2, c: true};
//!     let obj3 = CustomObject {a: String::from("c"),b: 3, c: false};
//!
//!      ListAssert::assert_that(actual_vec_with(
//!          vec![obj1.clone(), obj2.clone(), obj3.clone()],
//!          custom_describe,
//!      ))
//!      .with_element_matcher(custom_match)
//!      .is_equal_to(expected_vec_with(vec![obj1, obj2, obj3], custom_describe))
//!      .in_order()
//! }
//! ```

use crate::assertions::{
    CollectionContains, CollectionEqual, CollectionNotContain, CollectionNotEqual, Length,
};
use crate::{test_failed, Actual, Expected};
use std::collections::HashMap;

pub struct ListAssert<T> {
    actual: Vec<Actual<T>>,
}

impl<T> ListAssert<T>
where
    T: 'static,
{
    pub fn assert_that(actual: Vec<Actual<T>>) -> ListAssert<T> {
        ListAssert { actual }
    }

    pub fn with_element_matcher(self, element_matcher: fn(&T, &T) -> bool) -> ListElementAssert<T> {
        ListElementAssert {
            actual: self.actual,
            element_matcher,
        }
    }

    pub fn length(self) -> Box<dyn Length> {
        Box::new(self)
    }
}

pub struct ListElementAssert<T> {
    actual: Vec<Actual<T>>,
    element_matcher: fn(&T, &T) -> bool,
}

impl<T> ListElementAssert<T>
where
    T: 'static,
{
    pub fn contains(self, expected: Vec<Expected<T>>) -> Box<dyn CollectionContains<T>> {
        Box::new(ListFinalAssert {
            actual: self.actual,
            expected,
            element_matcher: self.element_matcher,
        })
    }

    pub fn does_not_contain(self, expected: Vec<Expected<T>>) -> Box<dyn CollectionNotContain<T>> {
        Box::new(ListFinalAssert {
            actual: self.actual,
            expected,
            element_matcher: self.element_matcher,
        })
    }

    pub fn is_equal_to(self, expected: Vec<Expected<T>>) -> Box<dyn CollectionEqual<T>> {
        Box::new(ListFinalAssert {
            actual: self.actual,
            expected,
            element_matcher: self.element_matcher,
        })
    }

    pub fn is_not_equal_to(self, expected: Vec<Expected<T>>) -> Box<dyn CollectionNotEqual<T>> {
        Box::new(ListFinalAssert {
            actual: self.actual,
            expected,
            element_matcher: self.element_matcher,
        })
    }
}

struct ListFinalAssert<T> {
    actual: Vec<Actual<T>>,
    expected: Vec<Expected<T>>,
    element_matcher: fn(&T, &T) -> bool,
}

type ActualIndex = usize;
type ExpectedIndex = usize;

impl<T> ListFinalAssert<T> {
    fn vec_to_string(vec: &[Actual<T>]) -> String {
        vec.iter()
            .map(|val| val.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }

    fn contains_in_any_order(&self) -> Result<(), String> {
        //[1,2,3,4,5].contains[5,3,1]

        let mut errors: Vec<String> = vec![];
        let expected_diff = self.only_in_expected();
        for expected_index in expected_diff {
            errors.push(format!(
                "   - Expected element: ({}) - Not found",
                self.expected[expected_index]
            ));
        }

        if errors.is_empty() {
            return Ok(());
        }
        Err(errors.join("\n"))
    }

    fn get_unchecked_index(
        &self,
        expected_elem: &Expected<T>,
        checked_actual_indexes: &[usize],
    ) -> Option<usize> {
        for (actual_index, actual_elem) in self.actual.iter().enumerate() {
            let actual_index_was_checked = checked_actual_indexes.get(actual_index).is_some();
            if actual_index_was_checked {
                continue;
            }
            if (self.element_matcher)(&actual_elem.value, &expected_elem.value) {
                return Some(actual_index);
            }
        }

        None
    }

    fn actual_contains_exact_expected_slice(&self, starting_position: usize) -> bool {
        let mut matches_length = 0;
        for (expected_index, expected_elem) in self.expected.iter().enumerate() {
            let actual_index = expected_index + starting_position;
            if actual_index >= self.actual.len() {
                return false;
            }
            let actual_elem = &self.actual[actual_index];

            if (self.element_matcher)(&actual_elem.value, &expected_elem.value) {
                matches_length += 1;
            } else {
                break;
            }
        }

        matches_length == self.expected.len()
    }

    fn actual_len_ge_expected(&self) -> Result<(), String> {
        if self.actual.len() < self.expected.len() {
            return Err(format!(
                "\n The actual vec size ({}) is LESS then expected vec ({})\n",
                self.actual.len(),
                self.expected.len()
            ));
        };
        Ok(())
    }

    fn list_test_failed(&self, error_message: &str) {
        let list_error_message = format!(
            "Actual vec: [{}] \n Expected vec: [{}] \n Detailed error: \n {}",
            Self::vec_to_string(&self.actual),
            Self::vec_to_string(&self.expected),
            error_message
        );
        test_failed(&list_error_message);
    }

    fn only_in_expected(&self) -> Vec<ExpectedIndex> {
        let (_, only_in_expected) = self.difference_ignoring_position();

        only_in_expected
    }

    fn difference_ignoring_position(&self) -> (Vec<ActualIndex>, Vec<ExpectedIndex>) {
        let intersection = self.intersection_indexes();
        if intersection.len() == self.actual.len() && intersection.len() == self.expected.len() {
            return (vec![], vec![]);
        }

        let mut actual_existing: Vec<bool> = vec![false; self.actual.len()];
        let mut expected_existing: Vec<bool> = vec![false; self.expected.len()];
        for (actual_index, expected_index) in intersection {
            actual_existing[actual_index] = true;
            expected_existing[expected_index] = true;
        }

        let mut only_in_actual = vec![];
        for (index, exist) in actual_existing.iter().enumerate() {
            if !exist {
                only_in_actual.push(index);
            }
        }

        let mut only_in_expected = vec![];
        for (index, exist) in expected_existing.iter().enumerate() {
            if !exist {
                only_in_expected.push(index);
            }
        }

        (only_in_actual, only_in_expected)
    }

    fn intersection_indexes(&self) -> HashMap<ActualIndex, ExpectedIndex> {
        let mut equals_indexes: HashMap<usize, usize> = HashMap::new();

        for (expected_index, expected_elem) in self.expected.iter().enumerate() {
            for (actual_index, actual_elem) in self.actual.iter().enumerate() {
                let actual_index_was_matched = equals_indexes.get(&actual_index).is_some();
                if actual_index_was_matched {
                    continue;
                }

                let elements_matches =
                    (self.element_matcher)(&actual_elem.value, &expected_elem.value);
                if elements_matches {
                    equals_indexes.insert(actual_index, expected_index);
                    break;
                }
            }
        }

        equals_indexes
    }
}

impl<T> CollectionEqual<T> for ListFinalAssert<T> {
    fn in_any_order(&self) {
        let error = length_check(self.actual.len(), self.expected.len()).err();
        if let Some(error_message) = error {
            self.list_test_failed(&error_message);
        }
        let result = self.contains_in_any_order();
        if let Some(error) = result.err() {
            self.list_test_failed(&error);
        }
    }

    fn in_order(&self) {
        let error = length_check(self.actual.len(), self.expected.len()).err();
        if let Some(error_message) = error {
            self.list_test_failed(&error_message);
        }
        let mut errors: Vec<String> = vec![];

        for index in 0..self.actual.len() {
            let actual_elem = &self.actual[index];
            let expected_elem = &self.expected[index];
            if !(self.element_matcher)(&actual_elem.value, &expected_elem.value) {
                errors.push(format!(
                    "Actual element: ({}) not equal to Expected: ({}) in position ({})\n",
                    actual_elem, expected_elem, index
                ));
            }
        }

        if !errors.is_empty() {
            let error_message = format!("\n {}", errors.join("\n"));
            self.list_test_failed(&error_message);
        }
    }
}

impl<T> CollectionNotEqual<T> for ListFinalAssert<T> {
    fn in_any_order(&self) {
        if self.actual.len() != self.expected.len() {
            return;
        }
        let result = self.contains_in_any_order();
        if result.is_ok() {
            self.list_test_failed("Collections are equals, but should not. Collections considered equals if they has same elements in any order.");
        }
    }

    fn in_order(&self) {
        if self.actual.len() != self.expected.len() {
            return;
        }

        for index in 0..self.actual.len() {
            let actual_elem = &self.actual[index];
            let expected_elem = &self.expected[index];
            if !(self.element_matcher)(&actual_elem.value, &expected_elem.value) {
                return;
            }
        }

        self.list_test_failed("Collections has same elements in same order, they are equal");
    }
}

impl<T> CollectionContains<T> for ListFinalAssert<T> {
    fn in_any_order(&self) {
        let result = self.contains_in_any_order();
        if let Some(error) = result.err() {
            self.list_test_failed(&error);
        }
    }

    fn in_exact_order(&self) {
        //[1,2,2,2,5,2,3,4,5].contains[2,3,4]
        let error = self.actual_len_ge_expected().err();
        if let Some(error_message) = error {
            self.list_test_failed(&error_message);
        }

        let first_expected_elem = &self.expected[0];
        let mut checked_actual_indexes: Vec<usize> = Vec::new();
        let mut matched = false;
        loop {
            let starting_position =
                self.get_unchecked_index(first_expected_elem, &checked_actual_indexes);
            if starting_position.is_none() {
                break;
            }
            let starting_position = starting_position.unwrap();
            checked_actual_indexes.push(starting_position);

            matched = self.actual_contains_exact_expected_slice(starting_position);
            if matched {
                break;
            }
        }

        if !matched {
            self.list_test_failed(
                "\n the Actual vec  doesn't contains Expected vec in Exact order",
            );
        }
    }

    fn just_in_order(&self) {
        //[1,2,8,9,5,7,3,1,4].contains[2,3,4]
        let result = self.actual_len_ge_expected();
        if let Some(error_message) = result.err() {
            self.list_test_failed(&error_message);
        }
        let mut prev_number_index: usize = 0;
        let mut matched_length: usize = 0;
        for expected_elem in &self.expected {
            let start_index = prev_number_index;
            for actual_index in start_index..self.actual.len() {
                let actual_elem = &self.actual[actual_index];
                if (self.element_matcher)(&actual_elem.value, &expected_elem.value) {
                    prev_number_index = actual_index;
                    matched_length += 1;
                    break;
                }
            }
        }

        if matched_length != self.expected.len() {
            let error_message = format!("\n the Actual vec doesn't contains Expected vec Just in order. Matched only {} elements",
                   matched_length
            );
            self.list_test_failed(&error_message);
        }
    }
}

impl<T> CollectionNotContain<T> for ListFinalAssert<T> {
    fn all(&self) {
        if self.actual.len() < self.expected.len() {
            return;
        }

        let intersection = self.intersection_indexes();
        if intersection.is_empty() {
            return;
        }

        let mut errors: Vec<String> = vec![];
        for (actual_index, expected_index) in intersection {
            errors.push(format!(
                "   - Element was found ({}). Actual index ({}), Expected index ({})",
                &self.actual[actual_index], actual_index, expected_index,
            ));
        }
        self.list_test_failed(&errors.join("\n"));
    }

    fn at_least_one(&self) {
        let intersection = self.intersection_indexes();
        if intersection.len() == self.expected.len() {
            self.list_test_failed("All expected elements were found in Actual vec");
        }
    }
}

impl<T> Length for ListAssert<T> {
    fn is(&self, expected: Expected<usize>) {
        let error = length_check(self.actual.len(), expected.value).err();
        if let Some(error_message) = error {
            test_failed(&error_message);
        }
    }
}

fn length_check(actual: usize, expected: usize) -> Result<(), String> {
    if actual != expected {
        return Err(format!(
            "\n Actual collection size: {} \n          not equal to \n Expected: {} \n",
            actual, expected,
        ));
    };
    Ok(())
}
