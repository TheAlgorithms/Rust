// Binary exponentiation is an algorithm to compute a power in O(logN) where N is the power.
//
// For example, to naively compute n^100, we multiply n 99 times for a O(N) algorithm.
//
// With binary exponentiation we can reduce the number of muliplications by only finding the binary
// exponents. n^100 = n^64 * n^32 * n^4. We can compute n^64 by ((((n^2)^2)^2)...), which is
// logN multiplications.
//
// We know which binary exponents to add by looking at the set bits in the power. For 100, we know
// the bits for 64, 32, and 4 are set.

// Computes n^p
pub fn binary_exponentiation(mut n: u64, mut p: u32) -> u64 {
    let mut result_pow: u64 = 1;
    while p > 0 {
        if p & 1 == 1 {
            result_pow *= n;
        }
        p >>= 1;
        n *= n;
    }
    result_pow
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        // Need to be careful about large exponents. It is easy to hit overflows.
        assert_eq!(binary_exponentiation(2, 3), 8);
        assert_eq!(binary_exponentiation(4, 12), 16777216);
        assert_eq!(binary_exponentiation(6, 12), 2176782336);
        assert_eq!(binary_exponentiation(10, 4), 10000);
        assert_eq!(binary_exponentiation(20, 3), 8000);
        assert_eq!(binary_exponentiation(3, 21), 10460353203);
    }

    #[test]
    fn up_to_ten() {
        // Compute all powers from up to ten, using the standard library as the source of truth.
        for i in 0..10 {
            for j in 0..10 {
                println!("{i}, {j}");
                assert_eq!(binary_exponentiation(i, j), u64::pow(i, j))
            }
        }
    }
}
