use std::fmt::{Display, Formatter};
use crate::{Actual, Expected};

impl<T> Actual<T> {
    pub fn new(value: T, description_func: fn(&T) -> String) -> Actual<T> {
        Actual {
            value,
            description_func,
        }
    }

    pub fn matches_with(self, expected: Expected<T>) -> Values<T> {
        Values {
            actual: self,
            expected,
        }
    }
}

impl<T> Actual<T>
    where T: Display {
    pub fn create_for(value: T) -> Actual<T> {
        Actual {
            value,
            description_func: |v| v.to_string(),
        }
    }
}

impl<T> Display for Actual<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", (self.description_func)(&self.value))
    }
}

impl<T> PartialEq for Actual<T>
    where T: PartialEq {
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }

    fn ne(&self, other: &Self) -> bool {
        self.value.ne(&other.value)
    }
}

pub struct Values<T> {
    actual: Actual<T>,
    expected: Expected<T>,
}

impl<T> Values<T> {
    pub fn by(self, matcher: fn(&T, &T) -> bool) -> MatcherResult<T> {
        let success = (matcher)(&self.actual.value, &self.expected.value);
        let result = match success {
            true => Ok(()),
            false => Err(())
        };

        MatcherResult {
            result,
            values: self,
        }
    }
}

pub struct MatcherResult<T> {
    result: Result<(), ()>,
    values: Values<T>,
}