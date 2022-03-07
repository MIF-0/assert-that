use num_traits::Num;
use crate::{Actual, Expected};

use crate::assertions::{Equals, Greater, GreaterOrEqual, Less, LessOrEqual, NotEquals};

pub struct NumericAssert<T>
where T: Num + PartialOrd {
 actual: Actual<T>,
}

impl<T> NumericAssert<T>
 where T: Num + PartialOrd ,
       T: 'static {
 pub fn assert_that(actual: Actual<T>) -> NumericAssert<T> {
  NumericAssert { actual }
 }

 pub fn is_equal(self) -> Box<dyn Equals<T>> {
  Box::new(self)
 }

 pub fn is_not_equal(self) -> Box<dyn NotEquals<T>> {
  Box::new(self)
 }

 pub fn is_less(self) -> Box<dyn Less<T>> {
  Box::new(self)
 }

 pub fn is_less_or_equal(self) -> Box<dyn LessOrEqual<T>> {
  Box::new(self)
 }

 pub fn is_greater(self) -> Box<dyn Greater<T>> {
  Box::new(self)
 }

 pub fn is_greater_or_equal(self) -> Box<dyn GreaterOrEqual<T>> {
  Box::new(self)
 }
}

impl<T> Equals<T> for NumericAssert<T>
 where T: Num + PartialOrd {
 fn to(&self, expected: Expected<T>) {
  if self.actual.ne(&expected) {
   panic!("\n Actual: {} \n not equal to expected \n {} \n", self.actual, expected);
  }
 }
}

impl<T> NotEquals<T> for NumericAssert<T>
 where T: Num + PartialOrd {
 fn to(&self, expected: Expected<T>) {
  if self.actual.eq(&expected) {
   panic!("\n Actual: {} \n is equal to expected \n {} \n", self.actual, expected);
  }
 }
}

impl<T> Less<T> for NumericAssert<T>
 where T: Num + PartialOrd {
 fn than(&self, expected: Expected<T>) {
  if self.actual.value >= expected.value {
   panic!("\n Actual: {} \n is bigger or equal to expected \n {} \n", self.actual, expected);
  }
 }
}

impl<T> LessOrEqual<T> for NumericAssert<T>
 where T: Num + PartialOrd {
 fn to(&self, expected: Expected<T>) {
  if self.actual.value > expected.value {
   panic!("\n Actual: {} \n is bigger to expected \n {} \n", self.actual, expected);
  }
 }
}

impl<T> Greater<T> for NumericAssert<T>
 where T: Num + PartialOrd {
 fn than(&self, expected: Expected<T>) {
  if self.actual.value <= expected.value {
   panic!("\n Actual: {} \n is smaller or equal to expected \n {} \n", self.actual, expected);
  }
 }
}

impl<T> GreaterOrEqual<T> for NumericAssert<T>
 where T: Num + PartialOrd {
 fn to(&self, expected: Expected<T>) {
  if self.actual.value < expected.value {
   panic!("\n Actual: {} \n is smaller to expected \n {} \n", self.actual, expected);
  }
 }
}