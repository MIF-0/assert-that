use crate::Expected;

pub trait Equals<T> {
    fn to(&self, expected: Expected<T>);
}

pub trait NotEquals<T> {
    fn to(&self, expected: Expected<T>);
}

pub trait Greater<T> {
    fn then(&self, expected: Expected<T> );
}

pub trait Less<T> {
    fn then(&self, expected: Expected<T>);
}

pub trait LessOrEqual<T> {
    fn to(&self, expected: Expected<T>);
}

pub trait GreaterOrEqual<T> {
    fn to(&self, expected: Expected<T> );
}

pub trait Length<T> {
    fn is(&self, expected: Expected<T>);
}

pub trait Matches<T> {
    fn to(&self, expected: Expected<T>);
}