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
        // ...
        composite_10: (10, 4), // 2 * 5
        composite_15: (15, 8), // 3 * 5
        // ...
        prime_power_2_to_3: (8, 4),
        prime_power_3_to_2: (9, 6),
        // ...
    }
}
