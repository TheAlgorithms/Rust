/// Calculate the greatest common divisor (GCD) of two numbers and the
/// coefficients of Bézout's identity using the Extended Euclidean Algorithm.
///
/// # Arguments
///
/// * `a` - One of the numbers to find the GCD of
/// * `m` - The other number to find the GCD of
///
/// # Returns
///
/// A tuple (gcd, x1, x2) such that:
/// gcd - the greatest common divisor of a and m.
/// x1, x2 - the coefficients such that `a * x1 + m * x2` is equivalent to `gcd` modulo `m`.
pub fn gcd_extended(a: i64, m: i64) -> (i64, i64, i64) {
    if a == 0 {
        (m, 0, 1)
    } else {
        let (gcd, x1, x2) = gcd_extended(m % a, a);
        let x = x2 - (m / a) * x1;
        (gcd, x, x1)
    }
}

/// Find the modular multiplicative inverse of a number modulo `m`.
///
/// # Arguments
///
/// * `b` - The number to find the modular inverse of
/// * `m` - The modulus
///
/// # Returns
///
/// The modular inverse of `b` modulo `m`.
///
/// # Panics
///
/// Panics if the inverse does not exist (i.e., `b` and `m` are not coprime).
pub fn mod_inverse(b: i64, m: i64) -> i64 {
    let (gcd, x, _) = gcd_extended(b, m);
    if gcd != 1 {
        panic!("Inverse does not exist");
    } else {
        // Ensure the modular inverse is positive
        (x % m + m) % m
    }
}

/// Perform modular exponentiation of a number raised to a power modulo `m`.
/// This function handles both positive and negative exponents.
///
/// # Arguments
///
/// * `base` - The base number to be raised to the `power`
/// * `power` - The exponent to raise the `base` to
/// * `modulus` - The modulus to perform the operation under
///
/// # Returns
///
/// The result of `base` raised to `power` modulo `modulus`.
pub fn modular_exponential(base: i64, mut power: i64, modulus: i64) -> i64 {
    if modulus == 1 {
        return 0; // Base case: any number modulo 1 is 0
    }

    // Adjust if the exponent is negative by finding the modular inverse
    let mut base = if power < 0 {
        mod_inverse(base, modulus)
    } else {
        base % modulus
    };

    let mut result = 1; // Initialize result
    power = power.abs(); // Work with the absolute value of the exponent

    // Perform the exponentiation
    while power > 0 {
        if power & 1 == 1 {
            result = (result * base) % modulus;
        }
        power >>= 1; // Divide the power by 2
        base = (base * base) % modulus; // Square the base
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modular_exponential_positive() {
        assert_eq!(modular_exponential(2, 3, 5), 3); // 2^3 % 5 = 8 % 5 = 3
        assert_eq!(modular_exponential(7, 2, 13), 10); // 7^2 % 13 = 49 % 13 = 10
        assert_eq!(modular_exponential(5, 5, 31), 25); // 5^5 % 31 = 3125 % 31 = 25
        assert_eq!(modular_exponential(10, 8, 11), 1); // 10^8 % 11 = 100000000 % 11 = 1
        assert_eq!(modular_exponential(123, 45, 67), 62); // 123^45 % 67
    }

    #[test]
    fn test_modular_inverse() {
        assert_eq!(mod_inverse(7, 13), 2); // Inverse of 7 mod 13 is 2
        assert_eq!(mod_inverse(5, 31), 25); // Inverse of 5 mod 31 is 25
        assert_eq!(mod_inverse(10, 11), 10); // Inverse of 10 mod 1 is 10
        assert_eq!(mod_inverse(123, 67), 6); // Inverse of 123 mod 67 is 6
        assert_eq!(mod_inverse(9, 17), 2); // Inverse of 9 mod 17 is 2
    }

    #[test]
    fn test_modular_exponential_negative() {
        assert_eq!(
            modular_exponential(7, -2, 13),
            mod_inverse(7, 13).pow(2) % 13
        ); // Inverse of 7 mod 13 is 2, 2^2 % 13 = 4 % 13 = 4
        assert_eq!(
            modular_exponential(5, -5, 31),
            mod_inverse(5, 31).pow(5) % 31
        ); // Inverse of 5 mod 31 is 25, 25^5 % 31 = 25
        assert_eq!(
            modular_exponential(10, -8, 11),
            mod_inverse(10, 11).pow(8) % 11
        ); // Inverse of 10 mod 11 is 10, 10^8 % 11 = 10
        assert_eq!(
            modular_exponential(123, -5, 67),
            mod_inverse(123, 67).pow(5) % 67
        ); // Inverse of 123 mod 67 is calculated via the function
    }

    #[test]
    fn test_modular_exponential_edge_cases() {
        assert_eq!(modular_exponential(0, 0, 1), 0); // 0^0 % 1 should be 0 as the modulus is 1
        assert_eq!(modular_exponential(0, 10, 1), 0); // 0^n % 1 should be 0 for any n
        assert_eq!(modular_exponential(10, 0, 1), 0); // n^0 % 1 should be 0 for any n
        assert_eq!(modular_exponential(1, 1, 1), 0); // 1^1 % 1 should be 0
        assert_eq!(modular_exponential(-1, 2, 1), 0); // (-1)^2 % 1 should be 0
    }
}
