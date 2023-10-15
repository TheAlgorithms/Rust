/// This Rust program calculates the n-th Frizzy number for a given base.
/// A Frizzy number is defined as the n-th number that is a sum of powers
/// of the given base, with the powers corresponding to the binary representation
/// of n.

/// The `get_nth_frizzy` function takes two arguments:
/// * `base` - The base whose n-th sum of powers is required.
/// * `n` - Index from ascending order of the sum of powers of the base.

/// It returns the n-th sum of powers of the base.

/// # Example
/// To find the Frizzy number with a base of 3 and n equal to 4:
/// - Ascending order of sums of powers of 3: 3^0 = 1, 3^1 = 3, 3^1 + 3^0 = 4, 3^2 + 3^0 = 9.
/// - The answer is 9.
///
/// # Arguments
/// * `base` - The base whose n-th sum of powers is required.
/// * `n` - Index from ascending order of the sum of powers of the base.
///
/// # Returns
/// The n-th sum of powers of the base.

pub fn get_nth_frizzy(base: i32, mut n: i32) -> f64 {
    let mut final1 = 0.0;
    let mut i = 0;
    while n > 0 {
        final1 += (base.pow(i) as f64) * ((n % 2) as f64);
        i += 1;
        n /= 2;
    }
    final1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_nth_frizzy() {
        // Test case 1: base = 3, n = 4
        // 3^2 + 3^0 = 9
        assert_eq!(get_nth_frizzy(3, 4), 9.0);

        // Test case 2: base = 2, n = 5
        // 2^2 + 2^0 = 5
        assert_eq!(get_nth_frizzy(2, 5), 5.0);

        // Test case 3: base = 4, n = 3
        // 4^1 + 4^0 = 5
        assert_eq!(get_nth_frizzy(4, 3), 5.0);

        // Test case 4: base = 5, n = 2
        // 5^1 + 5^0 = 5
        assert_eq!(get_nth_frizzy(5, 2), 5.0);

        // Test case 5: base = 6, n = 1
        // 6^0 = 1
        assert_eq!(get_nth_frizzy(6, 1), 1.0);
    }
}
