extern crate num_traits;

pub mod simple_nums;

mod assertions;
mod values;

pub struct Actual<T> {
    value: T,
    description_func: fn(&T) -> String,
}

pub type Expected<T> = Actual<T>;
