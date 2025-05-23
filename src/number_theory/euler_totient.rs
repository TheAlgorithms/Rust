pub fn euler_totient(n: u64) -> u64 {
    if n == 1 {
        return 1;
    }

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
    fn test_small_numbers() {
        assert_eq!(euler_totient(1), 1);
        assert_eq!(euler_totient(2), 1);
        assert_eq!(euler_totient(3), 2);
        assert_eq!(euler_totient(4), 2);
        assert_eq!(euler_totient(5), 4);
        assert_eq!(euler_totient(6), 2);
    }

    #[test]
    fn test_prime_numbers() {
        // For prime p, φ(p) = p - 1
        assert_eq!(euler_totient(7), 6);
        assert_eq!(euler_totient(11), 10);
        assert_eq!(euler_totient(13), 12);
        assert_eq!(euler_totient(17), 16);
    }

    #[test]
    fn test_prime_powers() {
        // For prime power p^k, φ(p^k) = p^(k-1) * (p-1)
        assert_eq!(euler_totient(9), 6); // 3^2, φ(9) = 3^1 * 2 = 6
        assert_eq!(euler_totient(25), 20); // 5^2, φ(25) = 5^1 * 4 = 20
        assert_eq!(euler_totient(8), 4); // 2^3, φ(8) = 2^2 * 1 = 4
    }

    #[test]
    fn test_larger_numbers() {
        assert_eq!(euler_totient(10), 4);
        assert_eq!(euler_totient(12), 4);
        assert_eq!(euler_totient(100), 40);
        assert_eq!(euler_totient(1000), 400);
    }
}
