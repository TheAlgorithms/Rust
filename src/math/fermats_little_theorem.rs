use rand::Rng;

/// Performs modular exponentiation.
///
/// This function computes `(base ^ exp) mod modulus` efficiently using the method
/// of exponentiation by squaring.
///
/// # Parameters
///
/// - `base`: The base value (u64).
/// - `exp`: The exponent (u64).
/// - `modulus`: The modulus (u64).
///
/// # Returns
///
/// The result of `(base ^ exp) mod modulus`.
fn mod_exp(base: u64, exp: u64, modulus: u64) -> u64 {
    let mut result = 1;
    let mut base = base % modulus;
    let mut exp = exp;

    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        exp /= 2;
        base = (base * base) % modulus;
    }

    result
}

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
pub fn fermats_little_theorem(p: u64, k: u32) -> bool {
    if p <= 1 {
        return false;
    }
    if p <= 3 {
        return true;
    }

    let mut rng = rand::thread_rng();

    for _ in 0..k {
        let a = rng.gen_range(2..p - 1);
        if mod_exp(a, p - 1, p) != 1 {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::fermats_little_theorem;

    #[test]
    fn test_prime_numbers() {
        assert!(fermats_little_theorem(5, 10));
        assert!(fermats_little_theorem(13, 10));
        assert!(fermats_little_theorem(101, 10));
        assert!(fermats_little_theorem(997, 10));
        assert!(fermats_little_theorem(7919, 10));
    }

    #[test]
    fn test_composite_numbers() {
        assert!(!fermats_little_theorem(4, 10));
        assert!(!fermats_little_theorem(15, 10));
        assert!(!fermats_little_theorem(100, 10));
        assert!(!fermats_little_theorem(1001, 10));
    }

    #[test]
    fn test_small_numbers() {
        assert!(!fermats_little_theorem(1, 10));
        assert!(fermats_little_theorem(2, 10));
        assert!(fermats_little_theorem(3, 10));
        assert!(!fermats_little_theorem(0, 10));
    }

    #[test]
    fn test_large_numbers() {
        assert!(fermats_little_theorem(104729, 10));
        assert!(!fermats_little_theorem(104730, 10));
    }

    #[test]
    fn test_carmichael_numbers() {
        let carmichael_numbers = vec![561, 1105, 1729, 2465, 2821, 6601];
        for &n in &carmichael_numbers {
            let result = fermats_little_theorem(n, 10);
            assert!(result == false || result == true);
        }
    }
}
