/// Cipolla algorithm
///
/// Solving quadratic residue problem:
///     x^2 = a (mod p) , p is an odd prime
/// with O(M*log(n)) time complexity, M depends on the complexity of complex numbers multiplication.
///
/// Wikipedia reference: https://en.wikipedia.org/wiki/Cipolla%27s_algorithm
/// When a is the primitive root modulo n, the answer is unique.
/// Otherwise it will return the smallest positive solution
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};

use rand::Rng;

use super::{fast_power, PCG32};

#[derive(Debug)]
struct CustomFiniteField {
    modulus: u64,
    i_square: u64,
}

impl CustomFiniteField {
    pub fn new(modulus: u64, i_square: u64) -> Self {
        Self { modulus, i_square }
    }
}

#[derive(Clone, Debug)]
struct CustomComplexNumber {
    real: u64,
    imag: u64,
    f: Rc<CustomFiniteField>,
}

impl CustomComplexNumber {
    pub fn new(real: u64, imag: u64, f: Rc<CustomFiniteField>) -> Self {
        Self { real, imag, f }
    }

    pub fn mult_other(&mut self, rhs: &Self) {
        let tmp = (self.imag * rhs.real + self.real * rhs.imag) % self.f.modulus;
        self.real = (self.real * rhs.real
            + ((self.imag * rhs.imag) % self.f.modulus) * self.f.i_square)
            % self.f.modulus;
        self.imag = tmp;
    }

    pub fn mult_self(&mut self) {
        let tmp = (self.imag * self.real + self.real * self.imag) % self.f.modulus;
        self.real = (self.real * self.real
            + ((self.imag * self.imag) % self.f.modulus) * self.f.i_square)
            % self.f.modulus;
        self.imag = tmp;
    }

    pub fn fast_power(mut base: Self, mut power: u64) -> Self {
        let mut result = CustomComplexNumber::new(1, 0, base.f.clone());
        while power != 0 {
            if (power & 1) != 0 {
                result.mult_other(&base); // result *= base;
            }
            base.mult_self(); // base *= base;
            power >>= 1;
        }
        result
    }
}

fn is_residue(x: u64, modulus: u64) -> bool {
    let power = (modulus - 1) >> 1;
    x != 0 && fast_power(x as usize, power as usize, modulus as usize) == 1
}

/// The Legendre symbol `(a | p)`
///
/// Returns 0 if a = 0 mod p, 1 if a is a square mod p, -1 if it not a square mod p.
///
/// <https://en.wikipedia.org/wiki/Legendre_symbol>
pub fn legendre_symbol(a: u64, odd_prime: u64) -> i64 {
    debug_assert!(odd_prime % 2 != 0, "prime must be odd");
    if a == 0 {
        0
    } else if is_residue(a, odd_prime) {
        1
    } else {
        -1
    }
}

// return two solutions (x1, x2) for Quadratic Residue problem x^2 = a (mod p), where p is an odd prime
// if a is Quadratic Nonresidues, return None
pub fn cipolla(a: u32, p: u32, seed: Option<u64>) -> Option<(u32, u32)> {
    // The params should be kept in u32 range for multiplication overflow issue
    // But inside we use u64 for convenience
    let a = a as u64;
    let p = p as u64;
    if a == 0 {
        return Some((0, 0));
    }
    if !is_residue(a, p) {
        return None;
    }
    let seed = match seed {
        Some(seed) => seed,
        None => SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    };
    let mut rng = PCG32::new_default(seed);
    let r = loop {
        let r = rng.get_u64() % p;
        if r == 0 || !is_residue((p + r * r - a) % p, p) {
            break r;
        }
    };
    let filed = Rc::new(CustomFiniteField::new(p, (p + r * r - a) % p));
    let comp = CustomComplexNumber::new(r, 1, filed);
    let power = (p + 1) >> 1;
    let x0 = CustomComplexNumber::fast_power(comp, power).real as u32;
    let x1 = p as u32 - x0;
    if x0 < x1 {
        Some((x0, x1))
    } else {
        Some((x1, x0))
    }
}

/// Returns one of the two possible solutions of _x² = a mod p_, if any.
///
/// The other solution is _-x mod p_. If there is no solution, returns `None`.
///
/// Reference: H. Cohen, _A course in computational algebraic number theory_, Algorithm 1.4.3
///
/// ## Implementation details
///
/// To avoid multiplication overflows, internally the algorithm uses the `128`-bit arithmetic.
///
/// Also see [`cipolla`].
pub fn tonelli_shanks(a: i64, odd_prime: u64) -> Option<u64> {
    let p: u128 = odd_prime as u128;
    let e = (p - 1).trailing_zeros();
    let q = (p - 1) >> e; // p = 2^e * q, with q odd

    let a = if a < 0 {
        a.rem_euclid(p as i64) as u128
    } else {
        a as u128
    };

    let power_mod_p = |b, e| fast_power(b as usize, e as usize, p as usize) as u128;

    // find generator: choose a random non-residue n mod p
    let mut rng = rand::thread_rng();
    let n = loop {
        let n = rng.gen_range(0..p);
        if legendre_symbol(n as u64, p as u64) == -1 {
            break n;
        }
    };
    let z = power_mod_p(n, q);

    // init
    let mut y = z;
    let mut r = e;
    let mut x = power_mod_p(a, (q - 1) / 2) % p;
    let mut b = (a * x * x) % p;
    x = (a * x) % p;

    while b % p != 1 {
        // find exponent
        let m = (1..r)
            .scan(b, |prev, m| {
                *prev = (*prev * *prev) % p;
                Some((m, *prev == 1))
            })
            .find_map(|(m, cond)| cond.then_some(m));
        let Some(m) = m else {
            return None; // non-residue
        };

        // reduce exponent
        let t = power_mod_p(y as u128, 2_u128.pow(r - m - 1));
        y = (t * t) % p;
        r = m;
        x = (x * t) % p;
        b = (b * y) % p;
    }

    Some(x as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tonelli_shanks_residues(x: u64, odd_prime: u64) -> Option<(u64, u64)> {
        let x = tonelli_shanks(x as i64, odd_prime)?;
        let x2 = (-(x as i64)).rem_euclid(odd_prime as i64) as u64;
        Some(if x < x2 { (x, x2) } else { (x2, x) })
    }

    #[test]
    fn cipolla_small_numbers() {
        assert_eq!(cipolla(1, 43, None), Some((1, 42)));
        assert_eq!(cipolla(2, 23, None), Some((5, 18)));
        assert_eq!(cipolla(17, 83, Some(42)), Some((10, 73)));
    }

    #[test]
    fn tonelli_shanks_small_numbers() {
        assert_eq!(tonelli_shanks_residues(1, 43).unwrap(), (1, 42));
        assert_eq!(tonelli_shanks_residues(2, 23).unwrap(), (5, 18));
        assert_eq!(tonelli_shanks_residues(17, 83).unwrap(), (10, 73));
    }

    #[test]
    fn cipolla_random_numbers() {
        assert_eq!(cipolla(392203, 852167, None), Some((413252, 438915)));
        assert_eq!(
            cipolla(379606557, 425172197, None),
            Some((143417827, 281754370))
        );
        assert_eq!(
            cipolla(585251669, 892950901, None),
            Some((192354555, 700596346))
        );
        assert_eq!(
            cipolla(404690348, 430183399, Some(19260817)),
            Some((57227138, 372956261))
        );
        assert_eq!(
            cipolla(210205747, 625380647, Some(998244353)),
            Some((76810367, 548570280))
        );
    }

    #[test]
    fn tonelli_shanks_random_numbers() {
        assert_eq!(
            tonelli_shanks_residues(392203, 852167),
            Some((413252, 438915))
        );
        assert_eq!(
            tonelli_shanks_residues(379606557, 425172197),
            Some((143417827, 281754370))
        );
        assert_eq!(
            tonelli_shanks_residues(585251669, 892950901),
            Some((192354555, 700596346))
        );
        assert_eq!(
            tonelli_shanks_residues(404690348, 430183399),
            Some((57227138, 372956261))
        );
        assert_eq!(
            tonelli_shanks_residues(210205747, 625380647),
            Some((76810367, 548570280))
        );
    }

    #[test]
    fn no_answer() {
        assert_eq!(cipolla(650927, 852167, None), None);
        assert_eq!(tonelli_shanks(650927, 852167), None);
    }
}
