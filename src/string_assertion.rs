use crate::{Actual, Expected};
use crate::assertions::{Equals, Length, NotEquals};

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
}

impl Equals<String> for StringAssert
{
    fn to(&self, expected: Expected<String>) {
        if self.actual.ne(&expected) {
            panic!("Result \n {} \n not equal to expected \n {}", self.actual, expected);
        }
    }
}

impl NotEquals<String> for StringAssert
{
    fn to(&self, expected: Expected<String>) {
        if self.actual.eq(&expected) {
            panic!("Result \n {} \n not equal to expected \n {}", self.actual, expected);
        }
    }
}

impl Length<usize> for StringAssert
{
    fn is(&self, expected: Expected<usize>) {
        if self.actual.value.len() != expected.value {
            panic!("Result \n {} \n length not equal to \n {}", self.actual, expected);
        }
    }
}