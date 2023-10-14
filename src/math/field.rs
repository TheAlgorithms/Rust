use core::fmt;
use std::hash::{Hash, Hasher};
use std::ops::{Add, Div, Mul, Neg, Sub};

/// A field
///
/// <https://en.wikipedia.org/wiki/Field_(mathematics)>
pub trait Field:
    Neg<Output = Self>
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Eq
    + Copy
    + fmt::Debug
{
    const CHARACTERISTIC: u64;
    const ZERO: Self;
    const ONE: Self;

    /// Multiplicative inverse
    fn inverse(self) -> Self;

    /// Z-mod structure
    fn integer_mul(self, a: i64) -> Self;
    fn from_integer(a: i64) -> Self {
        Self::ONE.integer_mul(a)
    }

    /// Iterate over all elements in this field
    ///
    /// The iterator finishes only for finite fields.
    type ElementsIter: Iterator<Item = Self>;
    fn elements() -> Self::ElementsIter;
}

/// Prime field of order `P`, that is, finite field `GF(P) = ℤ/Pℤ`
///
/// Only primes `P` <= 2^63 - 25 are supported, because the field elements are represented by `i64`.
// TODO: Extend field implementation for any prime `P` by e.g. using u32 blocks.
#[derive(Clone, Copy)]
pub struct PrimeField<const P: u64> {
    a: i64,
}

impl<const P: u64> PrimeField<P> {
    /// Reduces the representation into the range [0, p)
    fn reduce(self) -> Self {
        let Self { a } = self;
        let p: i64 = P.try_into().expect("module not fitting into signed 64 bit");
        let a = a.rem_euclid(p);
        assert!(a >= 0);
        Self { a }
    }

    /// Returns the positive integer in the range [0, p) representing this element
    pub fn to_integer(&self) -> u64 {
        self.reduce().a as u64
    }
}

impl<const P: u64> From<i64> for PrimeField<P> {
    fn from(a: i64) -> Self {
        Self { a }
    }
}

impl<const P: u64> PartialEq for PrimeField<P> {
    fn eq(&self, other: &Self) -> bool {
        self.reduce().a == other.reduce().a
    }
}

impl<const P: u64> Eq for PrimeField<P> {}

impl<const P: u64> Neg for PrimeField<P> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self { a: -self.a }
    }
}

impl<const P: u64> Add for PrimeField<P> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            a: self.a.checked_add(rhs.a).unwrap_or_else(|| {
                let x = self.reduce();
                let y = rhs.reduce();
                x.a + y.a
            }),
        }
    }
}

impl<const P: u64> Sub for PrimeField<P> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            a: self.a.checked_sub(rhs.a).unwrap_or_else(|| {
                let x = self.reduce();
                let y = rhs.reduce();
                x.a - y.a
            }),
        }
    }
}

impl<const P: u64> Mul for PrimeField<P> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            a: self.a.checked_mul(rhs.a).unwrap_or_else(|| {
                let x = self.reduce();
                let y = rhs.reduce();
                x.a * y.a
            }),
        }
    }
}

impl<const P: u64> Div for PrimeField<P> {
    type Output = Self;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.inverse()
    }
}

impl<const P: u64> fmt::Debug for PrimeField<P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let x = self.reduce();
        write!(f, "{}", x.reduce().a)
    }
}

impl<const P: u64> Field for PrimeField<P> {
    const CHARACTERISTIC: u64 = P;
    const ZERO: Self = Self { a: 0 };
    const ONE: Self = Self { a: 1 };

    fn inverse(self) -> Self {
        assert_ne!(self.a, 0);
        Self {
            a: mod_inverse(
                self.a,
                P.try_into().expect("module not fitting into signed 64 bit"),
            ),
        }
    }

    fn integer_mul(self, mut n: i64) -> Self {
        if n == 0 {
            return Self::ZERO;
        }
        let mut x = self;
        if n < 0 {
            x = -x;
            n = -n;
        }
        let mut y = Self::ZERO;
        while n > 1 {
            if n % 2 == 1 {
                y = y + x;
                n -= 1;
            }
            x = x + x;
            n /= 2;
        }
        x + y
    }

    type ElementsIter = PrimeFieldElementsIter<P>;

    fn elements() -> Self::ElementsIter {
        PrimeFieldElementsIter::default()
    }
}

#[derive(Default)]
pub struct PrimeFieldElementsIter<const P: u64> {
    x: i64,
}

impl<const P: u64> Iterator for PrimeFieldElementsIter<P> {
    type Item = PrimeField<P>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x as u64 == P {
            None
        } else {
            let res = PrimeField::from_integer(self.x);
            self.x += 1;
            Some(res)
        }
    }
}

impl<const P: u64> Hash for PrimeField<P> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let Self { a } = self.reduce();
        state.write_i64(a);
    }
}

// TODO: should we use extended_euclidean_algorithm adjusted to i64?
fn mod_inverse(mut a: i64, mut b: i64) -> i64 {
    let mut s = 1;
    let mut t = 0;
    let step = |x, y, q| (y, x - q * y);
    while b != 0 {
        let q = a / b;
        (a, b) = step(a, b, q);
        (s, t) = step(s, t, q);
    }
    assert!(a == 1 || a == -1);
    a * s
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_field_elements() {
        fn test<const P: u64>() {
            let expected: HashSet<PrimeField<P>> = (0..P as i64).map(Into::into).collect();
            for gen in 1..P - 1 {
                // every field element != 0 generates the whole field additively
                let gen = PrimeField::from(gen as i64);
                let mut generated: HashSet<PrimeField<P>> = [gen].into_iter().collect();
                let mut x = gen;
                for _ in 0..P {
                    x = x + gen;
                    generated.insert(x);
                }
                assert_eq!(generated, expected);
            }
        }
        test::<5>();
        test::<7>();
        test::<11>();
        test::<13>();
        test::<17>();
        test::<19>();
        test::<23>();
        test::<71>();
        test::<101>();
    }

    #[test]
    fn large_prime_field() {
        const P: u64 = 2_u64.pow(63) - 25; // largest prime fitting into i64
        type F = PrimeField<P>;
        let x = F::from(P as i64 - 1);
        let y = x.inverse();
        assert_eq!(x * y, F::ONE);
    }

    #[test]
    fn inverse() {
        fn test<const P: u64>() {
            for x in -7..7 {
                let x = PrimeField::<P>::from(x);
                if x != PrimeField::ZERO {
                    // multiplicative
                    assert_eq!(x.inverse() * x, PrimeField::ONE);
                    assert_eq!(x * x.inverse(), PrimeField::ONE);
                    assert_eq!((x.inverse().a * x.a).rem_euclid(P as i64), 1);
                    assert_eq!(x / x, PrimeField::ONE);
                }
                // additive
                assert_eq!(x + (-x), PrimeField::ZERO);
                assert_eq!((-x) + x, PrimeField::ZERO);
                assert_eq!(x - x, PrimeField::ZERO);
            }
        }
        test::<5>();
        test::<7>();
        test::<11>();
        test::<13>();
        test::<17>();
        test::<19>();
        test::<23>();
        test::<71>();
        test::<101>();
    }

    #[test]
    fn test_mod_inverse() {
        assert_eq!(mod_inverse(-6, 7), 1);
        assert_eq!(mod_inverse(-5, 7), -3);
        assert_eq!(mod_inverse(-4, 7), -2);
        assert_eq!(mod_inverse(-3, 7), 2);
        assert_eq!(mod_inverse(-2, 7), 3);
        assert_eq!(mod_inverse(-1, 7), -1);
        assert_eq!(mod_inverse(1, 7), 1);
        assert_eq!(mod_inverse(2, 7), -3);
        assert_eq!(mod_inverse(3, 7), -2);
        assert_eq!(mod_inverse(4, 7), 2);
        assert_eq!(mod_inverse(5, 7), 3);
        assert_eq!(mod_inverse(6, 7), -1);
    }

    #[test]
    fn integer_mul() {
        type F = PrimeField<23>;
        for x in 0..23 {
            let x = F { a: x };
            for n in -7..7 {
                assert_eq!(x.integer_mul(n), F { a: n * x.a });
            }
        }
    }

    #[test]
    fn from_integer() {
        type F = PrimeField<23>;
        for x in -100..100 {
            assert_eq!(F::from_integer(x), F { a: x });
        }
        assert_eq!(F::from(0), F::ZERO);
        assert_eq!(F::from(1), F::ONE);
    }
}
