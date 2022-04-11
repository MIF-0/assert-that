use crate::assertions::{CollectionContains, CollectionEqual, Length};
use crate::{Actual, Expected};
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

    pub fn is_equal_to(self, expected: Vec<Expected<T>>) -> Box<dyn CollectionEqual<T>> {
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

impl<T> ListFinalAssert<T> {
    fn in_any_order_ignore_size(&self) {
        //[1,2,3,4,5].contains[5,3,1]
        let mut equals_indexes: HashMap<usize, usize> = HashMap::new();
        let mut errors: Vec<String> = vec![];

        for (expected_index, expected_elem) in self.expected.iter().enumerate() {
            let mut match_found = false;

            for (actual_index, actual_elem) in self.actual.iter().enumerate() {
                let actual_index_was_matched = equals_indexes.get(&actual_index).is_some();
                if actual_index_was_matched {
                    continue;
                }

                let elements_matches =
                    (self.element_matcher)(&actual_elem.value, &expected_elem.value);
                if elements_matches {
                    equals_indexes.insert(actual_index, expected_index);
                    match_found = true;
                    break;
                }
            }
            if !match_found {
                errors.push(format!(
                    "Expected element:\n [{}]\n was not found",
                    expected_elem
                ));
            }
        }

        if errors.len() != 0 {
            panic!(
                "\n {} \n Actual vec: \n[{}]\n",
                errors.join("\n"),
                Self::vec_to_string(&self.actual)
            )
        }
    }

    fn get_unchecked_index(&self, expected_elem: &Expected<T>, checked_actual_indexes: &Vec<usize>) -> Option<usize> {
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
            let actual_elem = &self.actual[actual_index];

            if (self.element_matcher)(&actual_elem.value, &expected_elem.value) {
                matches_length = matches_length + 1;
            } else {
                break;
            }
        }

        matches_length == self.expected.len()
    }

    fn vec_to_string(vec: &Vec<Actual<T>>) -> String {
        vec
            .iter()
            .map(|val| { val.to_string() })
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl<T> CollectionEqual<T> for ListFinalAssert<T> {
    fn in_any_order(&self) {
        if self.actual.len() != self.expected.len() {
            panic!(
                "\n Collection size: {} \n  not equal to expected \n {} \n",
                self.actual.len(),
                self.expected.len()
            );
        }
        self.in_any_order_ignore_size();
    }

    fn in_order(&self) {
        if self.actual.len() != self.expected.len() {
            panic!(
                "\n Collection size: {} \n  not equal to expected \n {} \n",
                self.actual.len(),
                self.expected.len()
            );
        }

        let mut errors: Vec<String> = vec![];

        for index in 0..self.actual.len() {
            let actual_elem = &self.actual[index];
            let expected_elem = &self.expected[index];
            if !(self.element_matcher)(&actual_elem.value, &expected_elem.value) {
                errors.push(format!(
                    "Actual element:\n [{}]\n not equal to expected: \n[{}]\n in position [{}]\n",
                    actual_elem, expected_elem, index
                ));
            }
        }

        if errors.len() != 0 {
            panic!("\n {}", errors.join("\n"));
        }
    }
}

impl<T> CollectionContains<T> for ListFinalAssert<T> {
    fn in_any_order(&self) {
        self.in_any_order_ignore_size();
    }

    fn in_exact_order(&self) {
        //[1,2,2,2,5,2,3,4,5].contains[2,3,4]

        let first_expected_elem = &self.expected[0];
        let mut checked_actual_indexes: Vec<usize> = Vec::new();
        let mut matched = false;
        loop {
            let starting_position = self.get_unchecked_index(first_expected_elem, &checked_actual_indexes);
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
            panic!("\n the actual vec \n {} \n doesn't contains expected vec {} \n in exact order",
                   Self::vec_to_string(&self.actual),
                   Self::vec_to_string(&self.expected)
            );
        }
    }

    fn just_in_order(&self) {
        //[1,2,8,9,5,7,3,1,4].contains[2,3,4]
        let mut prev_number_index: usize = 0;
        let mut matched_length: usize = 0;
        for expected_elem in &self.expected {
            for actual_index in prev_number_index..self.actual.len() {
                let actual_elem = &self.actual[actual_index];
                if (self.element_matcher)(&actual_elem.value, &expected_elem.value){
                    prev_number_index = actual_index;
                    matched_length = matched_length + 1;
                    break;
                }
            }
        }

        if matched_length != self.expected.len() {
            panic!("\n the actual vec \n {} \n doesn't contains expected vec {} \n just in order. Matched only {} elements",
                   Self::vec_to_string(&self.actual),
                   Self::vec_to_string(&self.expected),
                   matched_length
            );
        }
    }
}

impl<T> Length for ListAssert<T> {
    fn is(&self, expected: Expected<usize>) {
        if self.actual.len() != expected.value {
            panic!(
                "\n Collection size: {} \n  not equal to expected \n {} \n",
                self.actual.len(),
                expected
            );
        }
    }
}
