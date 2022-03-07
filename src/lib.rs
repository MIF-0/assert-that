extern crate alloc;
extern crate core;
extern crate num_traits;

use core::fmt::Display;

pub mod simple_num_assertions;
pub mod string_assertion;
pub mod custom_matcher;

mod assertions;
mod values;


pub struct Actual<T> {
    value: T,
    description_func: fn(&T) -> String,
}

pub type Expected<T> = Actual<T>;

pub fn actual<T>(val: T) -> Actual<T>
    where T: Display {
    Actual::create_for(val)
}

pub fn expected<T>(val: T) -> Expected<T>
    where T: Display {
    Expected::create_for(val)
}

pub fn actual_with<T>(val: T, description_func: fn(&T) -> String) -> Actual<T> {
    Actual::new(val, description_func)
}

pub fn expected_with<T>(val: T, description_func: fn(&T) -> String) -> Expected<T> {
    Expected::new(val, description_func)
}
