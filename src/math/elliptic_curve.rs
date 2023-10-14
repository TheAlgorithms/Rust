use std::collections::HashSet;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::{Add, Neg, Sub};

use crate::math::field::{Field, PrimeField};
use crate::math::quadratic_residue::legendre_symbol;

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

    const fn check_invariants() {
        assert!(F::CHARACTERISTIC != 2);
        assert!(F::CHARACTERISTIC != 3);
        assert!(Self::discriminant() != 0);
    }
}

/// Elliptic curve methods over a prime field
impl<const P: u64, const A: i64, const B: i64> EllipticCurve<PrimeField<P>, A, B> {
    /// Naive calculation of points via enumeration
    // TODO: Implement via generators
    pub fn points() -> impl Iterator<Item = Self> {
        std::iter::once(Self::infinity()).chain(
            PrimeField::elements()
                .flat_map(|x| PrimeField::elements().filter_map(move |y| Self::new(x, y))),
        )
    }

    /// Number of points on the elliptic curve over `F`, that is, `#E(F)`
    pub fn cardinality() -> usize {
        // TODO: implement counting for big P
        Self::cardinality_counted_legendre()
    }

    /// Number of points on the elliptic curve over `F`, that is, `#E(F)`
    ///
    /// We simply count the number of points for each x coordinate and sum them up.
    /// For that, we first precompute the table of all squares in `F`.
    ///
    /// Time complexity: O(P) <br>
    /// Space complexity: O(P)
    ///
    /// Only fast for small fields.
    pub fn cardinality_counted_table() -> usize {
        let squares: HashSet<_> = PrimeField::<P>::elements().map(|x| x * x).collect();
        1 + PrimeField::elements()
            .map(|x| {
                let y_square = x * x * x + x.integer_mul(A) + PrimeField::from_integer(B);
                if y_square == PrimeField::ZERO {
                    1
                } else if squares.contains(&y_square) {
                    2
                } else {
                    0
                }
            })
            .sum::<usize>()
    }

    /// Number of points on the elliptic curve over `F`, that is, `#E(F)`
    ///
    /// We count the number of points for each x coordinate by using the [Legendre symbol] _(X |
    /// P)_:
    ///
    /// _1 + (x^3 + Ax + B | P),_
    ///
    /// The total number of points is then:
    ///
    /// _#E(F) = 1 + P + Σ_x (x^3 + Ax + B | P)_ for _x_ in _F_.
    ///
    /// Time complexity: O(P) <br>
    /// Space complexity: O(1)
    ///
    /// Only fast for small fields.
    ///
    /// [Legendre symbol]: https://en.wikipedia.org/wiki/Legendre_symbol
    pub fn cardinality_counted_legendre() -> usize {
        let cardinality: i64 = 1
            + P as i64
            + PrimeField::<P>::elements()
                .map(|x| {
                    let y_square = x * x * x + x.integer_mul(A) + PrimeField::from_integer(B);
                    let y_square_int = y_square.to_integer();
                    legendre_symbol(y_square_int, P)
                })
                .sum::<i64>();
        cardinality
            .try_into()
            .expect("invalid legendre cardinality")
    }
}

/// Group law
impl<F: Field, const A: i64, const B: i64> Add for EllipticCurve<F, A, B> {
    type Output = Self;

    fn add(self, p: Self) -> Self::Output {
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
    use std::time::Instant;

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
    fn cardinality() {
        fn test<const P: u64>(expected: usize) {
            type E<const P: u64> = EllipticCurve<PrimeField<P>, 1, 0>;
            assert_eq!(E::<P>::cardinality(), expected);
            assert_eq!(E::<P>::cardinality_counted_table(), expected);
            assert_eq!(E::<P>::cardinality_counted_legendre(), expected);
        }
        test::<5>(4);
        test::<7>(8);
        test::<11>(12);
        test::<13>(20);
        test::<17>(16);
        test::<19>(20);
        test::<23>(24);
    }

    #[test]
    #[ignore = "slow test for measuring time"]
    fn cardinality_perf() {
        const P: u64 = 1000003;
        type E = EllipticCurve<PrimeField<P>, 1, 0>;
        const EXPECTED: usize = 1000004;

        let now = Instant::now();
        assert_eq!(E::cardinality_counted_table(), EXPECTED);
        println!("cardinality_counted_table    : {:?}", now.elapsed());
        let now = Instant::now();
        assert_eq!(E::cardinality_counted_legendre(), EXPECTED);
        println!("cardinality_counted_legendre : {:?}", now.elapsed());
    }

    #[test]
    #[ignore = "slow test showing that cadinality is not yet feasible to compute for a large prime"]
    fn cardinality_large_prime() {
        const P: u64 = 2_u64.pow(63) - 25; // largest prime fitting into i64
        type E = EllipticCurve<PrimeField<P>, 1, 0>;
        const EXPECTED: usize = 9223372041295506260;

        let now = Instant::now();
        assert_eq!(E::cardinality(), EXPECTED);
        println!("cardinality: {:?}", now.elapsed());
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
}
