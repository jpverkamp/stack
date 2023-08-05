use std::ops::*;

/// ----- Shared numeric tower implementation -----

/// The numeric tower
#[derive(Copy, Clone, Debug)]
pub enum Number {
    Integer(i64),
    Rational { numerator: i64, denominator: u64 },
    Float(f64),
    Complex { real: f64, imaginary: f64 },
}

impl Number {
    /// Coerces two numbers to the same type.
    /// TODO: This is ugly, but it works for now.
    pub fn coerce(a: Number, b: Number) -> (Number, Number) {
        use Number::*;

        match (a, b) {
            (Integer(_), Integer(_)) => (a, b),
            (Integer(_), Rational { .. }) => (a.to_rational(), b),
            (Integer(_), Float(_)) => (a.to_float(), b),
            (Integer(_), Complex { .. }) => (a.to_complex(), b),

            (Rational { .. }, Integer(_)) => (a, b.to_rational()),
            (Rational { .. }, Rational { .. }) => (a, b),
            (Rational { .. }, Float(_)) => (a.to_float(), b),
            (Rational { .. }, Complex { .. }) => (a.to_complex(), b),

            (Float(_), Integer(_)) => (a, b.to_float()),
            (Float(_), Rational { .. }) => (a, b.to_float()),
            (Float(_), Float(_)) => (a, b),
            (Float(_), Complex { .. }) => (a.to_complex(), b),

            (Complex { .. }, Integer(_)) => (a, b.to_complex()),
            (Complex { .. }, Rational { .. }) => (a, b.to_complex()),
            (Complex { .. }, Float(_)) => (a, b.to_complex()),
            (Complex { .. }, Complex { .. }) => (a, b),
        }
    }

    /// Forces a number to an integer
    /// Converting from a rational, float, or complex will truncate and/or drop the imaginary part
    pub fn to_integer(self) -> Number {
        use Number::*;

        match self {
            Integer(_) => self,
            Rational {
                numerator,
                denominator,
            } => Integer((numerator as f64 / denominator as f64) as i64),
            Float(v) => Integer(v as i64),
            Complex { real, .. } => Integer(real as i64),
        }
    }

    /// Forces a number to a rational
    /// Converting from a float or complex will truncate and/or drop the imaginary part
    /// TODO: Implement better float to rational conversion where possible
    pub fn to_rational(self) -> Number {
        use Number::*;

        match self {
            Integer(v) => Rational {
                numerator: v,
                denominator: 1,
            },
            Rational { .. } => self,
            Float(v) => Rational {
                numerator: v as i64,
                denominator: 1,
            },
            Complex { real, .. } => Rational {
                numerator: real as i64,
                denominator: 1,
            },
        }
    }

    /// Forces a number to a float
    /// Converting from a complex will drop the imaginary part
    pub fn to_float(self) -> Number {
        use Number::*;

        match self {
            Integer(v) => Float(v as f64),
            Rational {
                numerator,
                denominator,
            } => Float(numerator as f64 / denominator as f64),
            Float(_) => self,
            Complex { real, .. } => Float(real),
        }
    }

    /// Forces a number to a complex
    pub fn to_complex(self) -> Number {
        use Number::*;

        match self {
            Integer(v) => Complex {
                real: v as f64,
                imaginary: 0.0,
            },
            Rational {
                numerator,
                denominator,
            } => Complex {
                real: numerator as f64 / denominator as f64,
                imaginary: 0.0,
            },
            Float(v) => Complex {
                real: v,
                imaginary: 0.0,
            },
            Complex { .. } => self,
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
                    (
                        Number::Rational { numerator: an, denominator: ad },
                        Number::Rational { numerator: bn, denominator: bd }
                    ) => Number::Rational {
                        numerator: an $op bn,
                        denominator: (ad $op bd) as u64
                    },
                    (Number::Float(av), Number::Float(bv)) => Number::Float(av $op bv),
                    (
                        Number::Complex { real: ar, imaginary: ai },
                        Number::Complex { real: br, imaginary: bi }
                    ) => Number::Complex {
                        real: ar $op br,
                        imaginary: ai $op bi
                    },
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
