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
/// A tuple (gcd, x) such that:
/// gcd - the greatest common divisor of a and m.
/// x - the coefficient such that `a * x` is equivalent to `gcd` modulo `m`.
pub fn gcd_extended(a: i64, m: i64) -> (i64, i64) {
    if a == 0 {
        (m, 0)
    } else {
        let (gcd, x1) = gcd_extended(m % a, a);
        let x = x1 - (m / a) * x1;
        (gcd, x)
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
    let (gcd, x) = gcd_extended(b, m);
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
        assert_eq!(modular_exponential(2, 3, 5), 3); 
        assert_eq!(modular_exponential(7, 2, 13), 10); 
        assert_eq!(modular_exponential(5, 5, 31), 25); 
        assert_eq!(modular_exponential(10, 8, 11), 1); 
        assert_eq!(modular_exponential(123, 45, 67), 62); 
    }

    #[test]
    fn test_modular_exponential_negative() {
        assert_eq!(modular_exponential(2, -3, 5), mod_inverse(2, 5).pow(3) % 5); 
        assert_eq!(modular_exponential(7, -2, 13), mod_inverse(7, 13).pow(2) % 13); 
        assert_eq!(modular_exponential(5, -5, 31), mod_inverse(5, 31).pow(5) % 31); 
        assert_eq!(modular_exponential(10, -8, 11), mod_inverse(10, 11).pow(8) % 11); 
        assert_eq!(modular_exponential(123, -45, 67), mod_inverse(123, 67).pow(45) % 67); 
    }

    #[test]
    fn test_modular_exponential_edge_cases() {
        assert_eq!(modular_exponential(0, 0, 1), 0);
        assert_eq!(modular_exponential(0, 10, 1), 0); 
        assert_eq!(modular_exponential(10, 0, 1), 0);
        assert_eq!(modular_exponential(1, 1, 1), 0); 
        assert_eq!(modular_exponential(-1, 2, 1), 0); 
    }
}
