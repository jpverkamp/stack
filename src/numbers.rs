use std::ops::*;

/// The numeric tower
#[derive(Copy, Clone, Debug)]
pub enum Number {
    Integer(i64),
    // Rational(i64, i64),
    Float(f64),
    // Complex(f64, f64),
}

impl Number {
    /// Coerces two numbers to the same type.
    /// TODO: This is ugly, but it works for now.
    pub fn coerce(a: Number, b: Number) -> (Number, Number) {
        match (a, b) {
            (Number::Integer(_), Number::Integer(_)) => (a, b),
            (Number::Integer(av), Number::Float(_)) => (Number::Float(av as f64), b),
            (Number::Float(_), Number::Integer(bv)) => (a, Number::Float(bv as f64)),
            (Number::Float(_), Number::Float(_)) => (a, b),
        }
    }

    /// Forces a number to an integer
    pub fn to_integer(self) -> Number {
        match self {
            Number::Integer(_) => self,
            Number::Float(v) => Number::Integer(v as i64),
        }
    }

    /// Forces a number to a float
    pub fn to_float(self) -> Number {
        match self {
            Number::Integer(v) => Number::Float(v as f64),
            Number::Float(_) => self,
        }
    }
}

macro_rules! do_op {
    ($trait:ty, $f:ident, $op:tt) => {
        impl $trait for Number {
            type Output = Number;

            fn $f(self, rhs: Self) -> Self::Output {
                let (a, b) = Number::coerce(self, rhs);
                match (a, b) {
                    (Number::Integer(av), Number::Integer(bv)) => Number::Integer(av $op bv),
                    (Number::Float(av), Number::Float(bv)) => Number::Float(av $op bv),
                    _ => unreachable!(),
                }
            }
        }
    };
}

do_op!(Add, add, +);
do_op!(Sub, sub, -);
do_op!(Mul, mul, *);
do_op!(Div, div, /);
do_op!(Rem, rem, %);

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        let (a, b) = Number::coerce(*self, *other);
        match (a, b) {
            (Number::Integer(av), Number::Integer(bv)) => av == bv,
            (Number::Float(av), Number::Float(bv)) => av == bv,
            _ => unreachable!(),
        }
    }
}

impl Eq for Number {}

impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let (a, b) = Number::coerce(*self, *other);
        match (a, b) {
            (Number::Integer(av), Number::Integer(bv)) => av.partial_cmp(&bv),
            (Number::Float(av), Number::Float(bv)) => av.partial_cmp(&bv),
            _ => unreachable!(),
        }
    }
}
