use std::fmt::Display;

use num_traits::Num;

pub struct NumericAssert<T: Num + Display + PartialOrd> {
 value: T,
}

impl<T> NumericAssert<T>
 where T: Num + Display + PartialOrd {
 pub fn assert_that(value: T) -> NumericAssert<T> {
  NumericAssert { value }
 }

 pub fn is_equal(self) -> NumericEqualAssertion<T> {
  NumericEqualAssertion { value: self.value }
 }

 pub fn is_less(self) -> NumericLessAssertion<T> {
  NumericLessAssertion { value: self.value }
 }

 pub fn is_less_or_equal(self) -> NumericLessEqualAssertion<T> {
  NumericLessEqualAssertion { value: self.value }
 }

 pub fn is_greater(self) -> NumericGreaterAssertion<T> {
  NumericGreaterAssertion { value: self.value }
 }

 pub fn is_greater_or_equal(self) -> NumericGreaterEqualAssertion<T> {
  NumericGreaterEqualAssertion { value: self.value }
 }
}

pub struct NumericEqualAssertion<T: Num + Display + PartialOrd> {
 value: T,
}

impl<T> NumericEqualAssertion<T>
 where
     T: Num + Display + PartialOrd,
{
 pub fn to(&self, expected: T) {
  if !self.value.eq(&expected) {
   panic!("Result \n {} \n not equal to expected \n {}", self.value, expected);
  }
 }
}

pub struct NumericLessAssertion<T: Num + Display + PartialOrd> {
 value: T,
}

impl<T> NumericLessAssertion<T>
 where T: Num + Display + PartialOrd {
 pub fn then(&self, expected: T) {
  if self.value >= expected {
   panic!("Result \n {} \n is bigger or equal to expected \n {}", self.value, expected);
  }
 }
}

pub struct NumericLessEqualAssertion<T: Num + Display + PartialOrd> {
 value: T,
}

impl<T> NumericLessEqualAssertion<T>
 where T: Num + Display + PartialOrd {
 pub fn to(&self, expected: T) {
  if self.value > expected {
   panic!("Result \n {} \n is bigger to expected \n {}", self.value, expected);
  }
 }
}

pub struct NumericGreaterAssertion<T: Num + Display + PartialOrd> {
 value: T,
}

impl<T> NumericGreaterAssertion<T>
 where T: Num + Display + PartialOrd {
 pub fn then(&self, expected: T) {
  if self.value <= expected {
   panic!("Result \n {} \n is smaller or equal to expected \n {}", self.value, expected);
  }
 }
}

pub struct NumericGreaterEqualAssertion<T: Num + Display + PartialOrd> {
 value: T,
}

impl<T> NumericGreaterEqualAssertion<T>
 where T: Num + Display + PartialOrd {
 pub fn to(&self, expected: T) {
  if self.value < expected {
   panic!("Result \n {} \n is smaller to expected \n {}", self.value, expected);
  }
 }
}
