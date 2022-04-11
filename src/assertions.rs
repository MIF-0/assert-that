use crate::Expected;

pub trait Equals<T> {
    fn to(&self, expected: Expected<T>);
}

pub trait NotEquals<T> {
    fn to(&self, expected: Expected<T>);
}

pub trait Greater<T> {
    fn than(&self, expected: Expected<T>);
}

pub trait Less<T> {
    fn than(&self, expected: Expected<T>);
}

pub trait LessOrEqual<T> {
    fn to(&self, expected: Expected<T>);
}

pub trait GreaterOrEqual<T> {
    fn to(&self, expected: Expected<T>);
}

pub trait Length {
    fn is(&self, expected: Expected<usize>);
}

pub trait Matches<T> {
    fn to(&self, expected: Expected<T>);
}

pub trait Contains<T> {
    fn contains(&self, expected: Expected<T>);
}

pub trait CollectionContains<T> {
    // [A,B,C,A,D].contains([D,A,C]).in_any_order() = true
    fn in_any_order(&self);

    // [A,B,C,A,D].contains([B,C]).in_exact_order() = true
    // [A,B,C,A,D].contains([B,D]).in_exact_order() = false
    fn in_exact_order(&self);

    // [A,B,C,A,D].contains([B,C]).in_exact_order() = true
    // [A,B,C,A,D].contains([B,D]).in_exact_order() = true
    fn just_in_order(&self);
}

pub trait CollectionEqual<T> {
    fn in_any_order(&self);

    fn in_order(&self);
}
