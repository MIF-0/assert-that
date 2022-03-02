extern crate num_traits;
extern crate alloc;

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
