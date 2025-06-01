pub fn euler_totient(n: u64) -> u64 {
    let mut result = n;
    let mut num = n;
    let mut p = 2;

    // Find  all prime factors and apply formula
    while p * p <= num {
        // Check if p is a divisor of n
        if num % p == 0 {
            // If yes, then it is a prime factor
            // Apply the formula: result = result * (1 - 1/p)
            while num % p == 0 {
                num /= p;
            }
            result -= result / p;
        }
        p += 1;
    }

    // If num > 1, then it is a prime factor
    if num > 1 {
        result -= result / num;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_euler_totient_comprehensive() {
        let test_cases = vec![
            // Edges cases
            (1, 1),
            // small numbers
            (2, 1),
            (3, 2),
            (4, 2),
            (5, 4),
            (6, 2),
            //Prime numbers (φ(p) = p - 1)
            (7, 6),
            (11, 10),
            (13, 12),
            (17, 16),
            (19, 18),
            // Prime powers (φ(p^k) = p^(k-1) * (p-1))
            (8, 4),   // 2^3
            (9, 6),   // 3^2
            (16, 8),  // 2^4
            (25, 20), // 5^2
            (27, 18), // 3^3
            (32, 16), // 2^5
            // Composite numbers
            (10, 4), // 2 * 5
            (12, 4), // 2^2 * 3
            (15, 8), // 3 * 5
            (18, 6), // 2 * 3^2
            (20, 8), // 2^2 * 5
            (30, 8), // 2 * 3 * 5
            // Large numbers
            (50, 20),    // 2 * 5^2
            (100, 40),   // 2^2 * 5^2
            (1000, 400), // 2^3 * 5^3
        ];

        for (input, expected) in test_cases {
            assert_eq!(
                euler_totient(input),
                expected,
                "φ({input}) should be {expected}"
            );
        }
    }

    #[test]
    fn test_edge_cases() {
        let edge_cases = vec![
            (2, 1),    // Smallest prime
            (4, 2),    // Power of 2
            (6, 2),    // 2 * 3 (two small primes)
            (35, 24),  // 5 * 7 (two larger primes)
            (77, 60),  // 7 * 11 (ensures the final `if num > 1` branch)
            (128, 64), // Large power of 2
        ];

        for (input, expected) in edge_cases {
            assert_eq!(euler_totient(input), expected);
        }
    }

    #[test]
    fn test_prime_property() {
        // For any prime p, φ(p) = p - 1
        let primes = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31];
        for p in primes {
            assert_eq!(euler_totient(p), p - 1, "φ({p}) should be {}", p - 1);
        }
    }
}
