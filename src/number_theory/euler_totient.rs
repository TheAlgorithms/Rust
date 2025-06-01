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
    macro_rules! test_euler_totient {
        ($($name:ident: $test_case:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (input, expected) = $test_case;
                    assert_eq!(euler_totient(input), expected)
                }
            )*
        };
    }

    test_euler_totient! {
        prime_2: (2, 1),
        prime_3: (3, 2),
        prime_5: (5, 4),
        prime_7: (7, 6),
        prime_11: (11, 10),
        prime_13: (13, 12),
        prime_17: (17, 16),
        prime_19: (19, 18),

        // Small
        small_4: (4, 2),
        small_6: (6, 2),

        composite_10: (10, 4), // 2 * 5
        composite_15: (15, 8), // 3 * 5
        composite_12: (12, 4),   // 2^2 * 3
        composite_18: (18, 6),   // 2 * 3^2
        composite_20: (20, 8),   // 2^2 * 5
        composite_30: (30, 8),   // 2 * 3 * 5
        // ...
        prime_power_2_to_3: (8, 4),
        prime_power_3_to_2: (9, 6),
        prime_power_2_to_4: (16, 8),   // 2^4
        prime_power_5_to_2: (25, 20),  // 5^2
        prime_power_3_to_3: (27, 18),  // 3^3
        prime_power_2_to_5: (32, 16),  // 2^5

        // Large numbers
        large_50: (50, 20),      // 2 * 5^2
        large_100: (100, 40),    // 2^2 * 5^2
        large_1000: (1000, 400), // 2^3 * 5^3
    }
}
