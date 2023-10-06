use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::{Add, Div, Mul, Neg, Sub};

/// Element of a field which can be represented by i64
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
}

/// Prime field of order `P`, that is, finite field `GF(P) = ℤ/Pℤ`
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

    pub fn elements() -> impl Iterator<Item = Self> {
        (0..P.try_into().expect("module not fitting into signed 64 bit")).map(Self::from)
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

/// Elliptic curve defined by `y^2 = x^3 + Ax + B` over a prime field `F` of
/// characteristic != 2, 3
///
/// The coefficients of the elliptic curve are the constant parameters `A` and `B`.
///
/// Points form an abelian group with the neutral element [`EllipticCurve::infinity`]. The points
/// are represented via affine coordinates ([`EllipticCurve::new`]) except for the points
/// at infinity ([`EllipticCurve::infinity`]).
///
/// # Example
///
/// ```
/// use the_algorithms_rust::math::{EllipticCurve, PrimeField};
/// type E = EllipticCurve<PrimeField<7>, 1, 0>;
/// let P = E::new(0, 0).expect("not on curve E");
/// assert_eq!(P + P, E::infinity());
/// ```
#[derive(Clone, Copy)]
pub struct EllipticCurve<F, const A: i64, const B: i64> {
    infinity: bool,
    x: F,
    y: F,
}

impl<F: Field, const A: i64, const B: i64> EllipticCurve<F, A, B> {
    /// Point at infinity also the neutral element of the group
    pub fn infinity() -> Self {
        Self::check_invariants();
        Self {
            infinity: true,
            x: F::ZERO,
            y: F::ZERO,
        }
    }

    /// Affine point
    ///
    ///
    /// Return `None` if the coordinates are not on the curve
    pub fn new(x: impl Into<F>, y: impl Into<F>) -> Option<Self> {
        Self::check_invariants();
        let x = x.into();
        let y = y.into();
        if Self::contains(x, y) {
            Some(Self {
                infinity: false,
                x,
                y,
            })
        } else {
            None
        }
    }

    /// Return `true` if this is the point at infinity
    pub fn is_infinity(&self) -> bool {
        self.infinity
    }

    /// The affine x-coordinate of the point
    pub fn x(&self) -> &F {
        &self.x
    }

    /// The affine y-coordinate of the point
    pub fn y(&self) -> &F {
        &self.y
    }

    /// The discrimant of the elliptic curve
    pub const fn discriminant() -> i64 {
        // Note: we can't return an element of F here, because it is not
        // possible to declare a trait function as const (cf.
        // <https://doc.rust-lang.org/error_codes/E0379.html>)
        (-16 * (4 * A * A * A + 27 * B * B)) % (F::CHARACTERISTIC as i64)
    }

    fn contains(x: F, y: F) -> bool {
        y * y == x * x * x + x.integer_mul(A) + F::ONE.integer_mul(B)
    }

    /// Naive calculation of points via enumeration
    // TODO: Implement via generators
    pub fn points() -> impl Iterator<Item = Self> {
        std::iter::once(Self::infinity()).chain((0..F::CHARACTERISTIC as i64).flat_map(|x| {
            (0..F::CHARACTERISTIC as i64)
                .filter_map(move |y| Self::new(F::from_integer(x), F::from_integer(y)))
        }))
    }

    const fn check_invariants() {
        assert!(F::CHARACTERISTIC != 2);
        assert!(F::CHARACTERISTIC != 3);
        assert!(Self::discriminant() != 0);
    }
}

/// Group law
impl<F: Field, const A: i64, const B: i64> Add for EllipticCurve<F, A, B> {
    type Output = Self;

    fn add(self, p: Self) -> Self::Output {
        dbg!(self, p);
        if self.infinity {
            p
        } else if p.infinity {
            self
        } else if self.x == p.x && self.y == -p.y {
            // mirrored
            Self::infinity()
        } else {
            let slope = if self.x != p.x {
                (self.y - p.y) / (self.x - p.x)
            } else {
                ((self.x * self.x).integer_mul(3) + F::from_integer(A)) / self.y.integer_mul(2)
            };
            let x = slope * slope - self.x - p.x;
            let y = -self.y + slope * (self.x - x);
            Self::new(x, y).expect("elliptic curve group law failed")
        }
    }
}

/// Inverse
impl<F: Field, const A: i64, const B: i64> Neg for EllipticCurve<F, A, B> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        if self.infinity {
            self
        } else {
            Self::new(self.x, -self.y).expect("elliptic curves are x-axis symmetric")
        }
    }
}

/// Difference
impl<F: Field, const A: i64, const B: i64> Sub for EllipticCurve<F, A, B> {
    type Output = Self;

    fn sub(self, p: Self) -> Self::Output {
        self + (-p)
    }
}

/// Debug representation via projective coordinates
impl<F: fmt::Debug, const A: i64, const B: i64> fmt::Debug for EllipticCurve<F, A, B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.infinity {
            f.write_str("(0:0:1)")
        } else {
            write!(f, "({:?}:{:?}:1)", self.x, self.y)
        }
    }
}

/// Equality of the elliptic curve points (short-circuit at infinity)
impl<F: Field, const A: i64, const B: i64> PartialEq for EllipticCurve<F, A, B> {
    fn eq(&self, other: &Self) -> bool {
        (self.infinity && other.infinity)
            || (self.infinity == other.infinity && self.x == other.x && self.y == other.y)
    }
}

impl<F: Field, const A: i64, const B: i64> Eq for EllipticCurve<F, A, B> {}

impl<F: Field + Hash, const A: i64, const B: i64> Hash for EllipticCurve<F, A, B> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if self.infinity {
            state.write_u8(1);
            F::ZERO.hash(state);
            F::ZERO.hash(state);
        } else {
            state.write_u8(0);
            self.x.hash(state);
            self.y.hash(state);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    #[should_panic]
    fn test_char_2_panic() {
        EllipticCurve::<PrimeField<2>, -1, 1>::infinity();
    }

    #[test]
    #[should_panic]
    fn test_char_3_panic() {
        EllipticCurve::<PrimeField<2>, -1, 1>::infinity();
    }

    #[test]
    #[should_panic]
    fn test_singular_panic() {
        EllipticCurve::<PrimeField<5>, 0, 0>::infinity();
    }

    #[test]
    fn e_5_1_0_group_table() {
        type F = PrimeField<5>;
        type E = EllipticCurve<F, 1, 0>;

        assert_eq!(E::points().count(), 4);
        let [a, b, c, d] = [
            E::new(0, 0).unwrap(),
            E::infinity(),
            E::new(2, 0).unwrap(),
            E::new(3, 0).unwrap(),
        ];

        assert_eq!(a + a, b);
        assert_eq!(a + b, a);
        assert_eq!(a + c, d);
        assert_eq!(a + d, c);
        assert_eq!(b + a, a);
        assert_eq!(b + b, b);
        assert_eq!(b + c, c);
        assert_eq!(b + d, d);
        assert_eq!(c + a, d);
        assert_eq!(c + b, c);
        assert_eq!(c + c, b);
        assert_eq!(c + d, a);
        assert_eq!(d + a, c);
        assert_eq!(d + b, d);
        assert_eq!(d + c, a);
        assert_eq!(d + d, b);
    }

    #[test]
    fn group_law() {
        fn test<const P: u64>() {
            type E<const P: u64> = EllipticCurve<PrimeField<P>, 1, 0>;

            let o = E::<P>::infinity();
            assert_eq!(-o, o);

            let points: Vec<_> = E::points().collect();
            for &p in &points {
                assert_eq!(p + (-p), o); // inverse
                assert_eq!((-p) + p, o); // inverse
                assert_eq!(p - p, o); //inverse
                assert_eq!(p + o, p); // neutral
                assert_eq!(o + p, p); //neutral

                for &q in &points {
                    assert_eq!(p + q, q + p); // commutativity

                    // associativity
                    for &s in &points {
                        assert_eq!((p + q) + s, p + (q + s));
                    }
                }
            }
        }
        test::<5>();
        test::<7>();
        test::<11>();
        test::<13>();
        test::<17>();
        test::<19>();
        test::<23>();
    }

    #[test]
    fn test_points() {
        type F = PrimeField<5>;
        type E = EllipticCurve<F, 1, 0>;

        let points: HashSet<_> = E::points().collect();
        let expected: HashSet<_> = [
            E::infinity(),
            E::new(0, 0).unwrap(),
            E::new(2, 0).unwrap(),
            E::new(3, 0).unwrap(),
        ]
        .into_iter()
        .collect();
        assert_eq!(points, expected);
    }

    #[test]
    fn test_field_elements() {
        fn test<const P: u64>() {
            let expected: HashSet<PrimeField<P>> = (0..P as i64).map(Into::into).collect();
            for gen in 1..P - 1 {
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
                    dbg!(x, x.inverse());
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
