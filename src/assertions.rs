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

pub trait BooleanCheck {
    fn is_true(&self);

    fn is_false(&self);
}

pub trait OptionCheck {
    fn is_none(&self);

    fn is_some(&self);
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

pub trait CollectionNotContain<T> {
    // [A,B,C,D,E].does_not_contain([A,B,C]).all() = false
    // [A,B,C,D,E].does_not_contain([A,B,F]).all() = false
    // [A,B,C,D,E].does_not_contain([F,G,J]).all() = true
    fn all(&self);

    // [A,B,C,D,E].does_not_contain([A,B,C]).at_least_one() = false
    // [A,B,C,D,E].does_not_contain([A,B,F]).at_least_one() = true
    // [A,B,C,D,E].does_not_contain([F,G,J]).at_least_one() = true
    fn at_least_one(&self);
}

pub trait CollectionEqual<T> {
    fn in_any_order(&self);

    fn in_order(&self);
}

pub trait CollectionNotEqual<T> {
    // [A,B,C,D].is_not_equal(A,B,C,D).in_any_order() = false
    // [A,B,C,D].is_not_equal(D,C,B,A).in_any_order() = false
    // [A,B,C,D].is_not_equal(B,B,C,D).in_any_order() = true
    fn in_any_order(&self);

    // [A,B,C,D].is_not_equal(A,B,C,D).in_order() = false
    // [A,B,C,D].is_not_equal(D,C,B,A).in_order() = true
    fn in_order(&self);
}
