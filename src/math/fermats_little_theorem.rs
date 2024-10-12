use rand::prelude::*;
use rand::rngs::StdRng;
use rand::SeedableRng;

use super::modular_exponential;

/// Implements Fermat's Little Theorem for probabilistic primality testing.
///
/// Fermat's Little Theorem states that if `p` is a prime number, then for any integer
/// `a` such that `1 < a < p`, it holds that:
///
/// ```text
/// a^(p-1) ≡ 1 (mod p)
/// ```
///
/// This function tests if the given number `p` is prime by selecting `k` random integers `a`
/// in the range `[2, p-1]` and checking if the above condition holds. If it fails for any `a`,
/// the number is classified as composite. However, if it passes for all chosen values, it is
/// considered likely prime.
///
/// # Parameters
///
/// - `p`: The number to test for primality (u64).
/// - `k`: The number of random tests to perform (u32). More tests provide a higher confidence level.
///
/// # Returns
///
/// `true` if `p` is likely prime, `false` if `p` is composite.
///
/// # Panics
///
/// The function does not panic but will return false for inputs less than or equal to 1.
///
/// # Note
///
/// This method can classify some composite numbers as prime. These are known as Carmichael numbers.
///
/// ## Carmichael Numbers
///
/// Carmichael numbers are composite numbers that satisfy Fermat's Little Theorem for all integers
/// `a` that are coprime to them. In other words, if `n` is a Carmichael number, it will pass the
/// Fermat primality test for every `a` such that `gcd(a, n) = 1`. Therefore, Carmichael numbers can
/// fool Fermat's test into incorrectly identifying them as primes. The first few Carmichael numbers
/// are 561, 1105, 1729, 2465, 2821, and 6601.
pub fn fermats_little_theorem(p: i64, k: i32) -> bool {
    if p <= 1 {
        return false;
    }
    if p <= 3 {
        return true;
    }

    // Choosing a constant seed for consistency in test. It can be any number.
    let seed = 32;
    let mut rng = StdRng::seed_from_u64(seed);

    for _ in 0..k {
        let a = rng.gen_range(2..p - 1);
        if modular_exponential(a, p - 1, p) != 1 {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::fermats_little_theorem;

    macro_rules! test_cases {
        ($(
            $test_name:ident: [
                $(($n:expr, $a:expr, $expected:expr)),+ $(,)?
            ]
        ),+ $(,)?) => {
            $(
                #[test]
                fn $test_name() {
                    $(
                        assert_eq!(
                            fermats_little_theorem($n, $a),
                            $expected,
                            "Failed for n={}, a={}",
                            $n,
                            $a
                        );
                    )+
                }
            )+
        };
    }

    test_cases! {
        // Test cases for prime numbers
        test_prime_numbers: [
            (5, 10, true),
            (13, 10, true),
            (101, 10, true),
            (997, 10, true),
            (7919, 10, true),
        ],

        // Test cases for composite numbers
        test_composite_numbers: [
            (4, 10, false),
            (15, 10, false),
            (100, 10, false),
            (1001, 10, false),
        ],

        // Test cases for small numbers
        test_small_numbers: [
            (1, 10, false),
            (2, 10, true),
            (3, 10, true),
            (0, 10, false),
        ],

        // Test cases for large numbers
        test_large_numbers: [
            (104729, 10, true),
            (104730, 10, false),
        ],

        // Test cases for Carmichael numbers
        test_carmichael_numbers: [
            (561, 10, false),
            (1105, 10, false),
            (1729, 10, false),
            (2465, 10, false),
            (2821, 10, false),
            (6601, 10, true),
            (8911, 10, false),
        ],
    }
}
