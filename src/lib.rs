extern crate alloc;
extern crate core;
extern crate num_traits;

use core::fmt::Display;

pub mod custom_matcher;
pub mod list_assertions;
pub mod num_assertions;
pub mod string_assertions;

mod assertions;
mod values;

pub struct Actual<T> {
    value: T,
    description_func: fn(&T) -> String,
}

pub type Expected<T> = Actual<T>;

pub fn actual<T>(val: T) -> Actual<T>
where
    T: Display,
{
    Actual::create_for(val)
}

pub fn expected<T>(val: T) -> Expected<T>
where
    T: Display,
{
    Expected::create_for(val)
}

pub fn actual_with<T>(val: T, description_func: fn(&T) -> String) -> Actual<T> {
    Actual::new(val, description_func)
}

pub fn expected_with<T>(val: T, description_func: fn(&T) -> String) -> Expected<T> {
    Expected::new(val, description_func)
}

pub fn actual_vec_with<T>(values: Vec<T>, description_func: fn(&T) -> String) -> Vec<Actual<T>> {
    let mut result: Vec<Actual<T>> = Vec::new();
    for value in values {
        result.push(Actual::new(value, description_func));
    }

    result
}

pub fn expected_vec_with<T>(
    values: Vec<T>,
    description_func: fn(&T) -> String,
) -> Vec<Expected<T>> {
    let mut result: Vec<Actual<T>> = Vec::new();
    for value in values {
        result.push(Expected::new(value, description_func));
    }

    result
}

pub fn actual_vec<T>(values: Vec<T>) -> Vec<Actual<T>>
where
    T: Display,
{
    let mut result: Vec<Actual<T>> = Vec::new();
    for value in values {
        result.push(Actual::create_for(value));
    }

    result
}

pub fn expected_vec<T>(values: Vec<T>) -> Vec<Expected<T>>
where
    T: Display,
{
    let mut result: Vec<Actual<T>> = Vec::new();
    for value in values {
        result.push(Expected::create_for(value));
    }

    result
}
