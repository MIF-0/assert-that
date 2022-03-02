extern crate num_traits;

pub mod simple_num_assertions;
pub mod string_assertion;

mod assertions;
mod values;

pub struct Actual<T> {
    value: T,
    description_func: fn(&T) -> String,
}

pub type Expected<T> = Actual<T>;
