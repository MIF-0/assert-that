use crate::{Actual, Expected};
use crate::assertions::{Contains, Equals, Length, NotEquals};

pub struct StringAssert {
    actual: Actual<String>,
}

impl StringAssert {
    pub fn assert_that(actual: Actual<String>) -> StringAssert {
        StringAssert { actual }
    }

    pub fn is_equal(self) -> Box<dyn Equals<String>> {
        Box::new(self)
    }

    pub fn length(self) -> Box<dyn Length<usize>> {
        Box::new(self)
    }

    pub fn is_not_equal(self) -> Box<dyn NotEquals<String>> {
        Box::new(self)
    }

    pub fn contains(self, expected: Expected<String>) {
        Contains::<String>::contains(&self, expected);
    }
}

impl Equals<String> for StringAssert
{
    fn to(&self, expected: Expected<String>) {
        if self.actual.ne(&expected) {
            panic!("\n Actual: {} \n not equal to expected \n {} \n", self.actual, expected);
        }
    }
}

impl NotEquals<String> for StringAssert
{
    fn to(&self, expected: Expected<String>) {
        if self.actual.eq(&expected) {
            panic!("\n Actual: {} \n not equal to expected \n {} \n", self.actual, expected);
        }
    }
}

impl Length<usize> for StringAssert
{
    fn is(&self, expected: Expected<usize>) {
        if self.actual.value.len() != expected.value {
            panic!("\n Actual: {} \n length not equal to expected \n {} \n", self.actual, expected);
        }
    }
}

impl Contains<String> for StringAssert {
    fn contains(&self, expected: Expected<String>) {
        if !self.actual.value.contains(&expected.value) {
            panic!("\n Actual: {} \n does not contains expected \n {} \n", self.actual, expected);
        }
    }
}