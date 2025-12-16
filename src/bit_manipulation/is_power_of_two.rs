//! Power of Two Check
//!
//! This module provides a function to determine if a given positive integer is a power of two
//! using efficient bit manipulation.
//!
//! # Algorithm
//!
//! The algorithm uses the property that powers of two have exactly one bit set in their
//! binary representation. When we subtract 1 from a power of two, all bits after the single
//! set bit become 1, and the set bit becomes 0:
//!
//! ```text
//! n     = 0..100..00  (power of 2)
//! n - 1 = 0..011..11
//! n & (n - 1) = 0     (no intersections)
//! ```
//!
//! For example:
//! - 8 in binary:  1000
//! - 7 in binary:  0111
//! - 8 & 7 = 0000 = 0 ✓
//!
//! Author: Alexander Pantyukhin
//! Date: November 1, 2022

/// Determines if a given number is a power of two.
///
/// This function uses bit manipulation to efficiently check if a number is a power of two.
/// A number is a power of two if it has exactly one bit set in its binary representation.
/// The check `number & (number - 1) == 0` leverages this property.
///
/// # Arguments
///
/// * `number` - An integer to check (must be non-negative)
///
/// # Returns
///
/// A `Result` containing:
/// - `Ok(true)` - If the number is a power of two (including 0 and 1)
/// - `Ok(false)` - If the number is not a power of two
/// - `Err(String)` - If the number is negative
///
/// # Examples
///
/// ```
/// use the_algorithms_rust::bit_manipulation::is_power_of_two;
///
/// assert_eq!(is_power_of_two(0).unwrap(), true);
/// assert_eq!(is_power_of_two(1).unwrap(), true);
/// assert_eq!(is_power_of_two(2).unwrap(), true);
/// assert_eq!(is_power_of_two(4).unwrap(), true);
/// assert_eq!(is_power_of_two(8).unwrap(), true);
/// assert_eq!(is_power_of_two(16).unwrap(), true);
///
/// assert_eq!(is_power_of_two(3).unwrap(), false);
/// assert_eq!(is_power_of_two(6).unwrap(), false);
/// assert_eq!(is_power_of_two(17).unwrap(), false);
///
/// // Negative numbers return an error
/// assert!(is_power_of_two(-1).is_err());
/// ```
///
/// # Errors
///
/// Returns an error if the input number is negative.
///
/// # Time Complexity
///
/// O(1) - The function performs a constant number of operations regardless of input size.
pub fn is_power_of_two(number: i32) -> Result<bool, String> {
    if number < 0 {
        return Err("number must not be negative".to_string());
    }

    // Convert to u32 for safe bit operations
    let num = number as u32;

    // Check if number & (number - 1) == 0
    // For powers of 2, this will always be true
    Ok(num & num.wrapping_sub(1) == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero() {
        // 0 is considered a power of 2 by the algorithm (2^(-∞) interpretation)
        assert_eq!(is_power_of_two(0).unwrap(), true);
    }

    #[test]
    fn test_one() {
        // 1 = 2^0
        assert_eq!(is_power_of_two(1).unwrap(), true);
    }

    #[test]
    fn test_powers_of_two() {
        assert_eq!(is_power_of_two(2).unwrap(), true); // 2^1
        assert_eq!(is_power_of_two(4).unwrap(), true); // 2^2
        assert_eq!(is_power_of_two(8).unwrap(), true); // 2^3
        assert_eq!(is_power_of_two(16).unwrap(), true); // 2^4
        assert_eq!(is_power_of_two(32).unwrap(), true); // 2^5
        assert_eq!(is_power_of_two(64).unwrap(), true); // 2^6
        assert_eq!(is_power_of_two(128).unwrap(), true); // 2^7
        assert_eq!(is_power_of_two(256).unwrap(), true); // 2^8
        assert_eq!(is_power_of_two(512).unwrap(), true); // 2^9
        assert_eq!(is_power_of_two(1024).unwrap(), true); // 2^10
        assert_eq!(is_power_of_two(2048).unwrap(), true); // 2^11
        assert_eq!(is_power_of_two(4096).unwrap(), true); // 2^12
        assert_eq!(is_power_of_two(8192).unwrap(), true); // 2^13
        assert_eq!(is_power_of_two(16384).unwrap(), true); // 2^14
        assert_eq!(is_power_of_two(32768).unwrap(), true); // 2^15
        assert_eq!(is_power_of_two(65536).unwrap(), true); // 2^16
    }

    #[test]
    fn test_non_powers_of_two() {
        assert_eq!(is_power_of_two(3).unwrap(), false);
        assert_eq!(is_power_of_two(5).unwrap(), false);
        assert_eq!(is_power_of_two(6).unwrap(), false);
        assert_eq!(is_power_of_two(7).unwrap(), false);
        assert_eq!(is_power_of_two(9).unwrap(), false);
        assert_eq!(is_power_of_two(10).unwrap(), false);
        assert_eq!(is_power_of_two(11).unwrap(), false);
        assert_eq!(is_power_of_two(12).unwrap(), false);
        assert_eq!(is_power_of_two(13).unwrap(), false);
        assert_eq!(is_power_of_two(14).unwrap(), false);
        assert_eq!(is_power_of_two(15).unwrap(), false);
        assert_eq!(is_power_of_two(17).unwrap(), false);
        assert_eq!(is_power_of_two(18).unwrap(), false);
    }

    #[test]
    fn test_specific_non_powers() {
        assert_eq!(is_power_of_two(6).unwrap(), false);
        assert_eq!(is_power_of_two(17).unwrap(), false);
        assert_eq!(is_power_of_two(100).unwrap(), false);
        assert_eq!(is_power_of_two(1000).unwrap(), false);
    }

    #[test]
    fn test_large_powers_of_two() {
        assert_eq!(is_power_of_two(131072).unwrap(), true); // 2^17
        assert_eq!(is_power_of_two(262144).unwrap(), true); // 2^18
        assert_eq!(is_power_of_two(524288).unwrap(), true); // 2^19
        assert_eq!(is_power_of_two(1048576).unwrap(), true); // 2^20
    }

    #[test]
    fn test_numbers_near_powers_of_two() {
        // One less than powers of 2
        assert_eq!(is_power_of_two(3).unwrap(), false); // 2^2 - 1
        assert_eq!(is_power_of_two(7).unwrap(), false); // 2^3 - 1
        assert_eq!(is_power_of_two(15).unwrap(), false); // 2^4 - 1
        assert_eq!(is_power_of_two(31).unwrap(), false); // 2^5 - 1
        assert_eq!(is_power_of_two(63).unwrap(), false); // 2^6 - 1
        assert_eq!(is_power_of_two(127).unwrap(), false); // 2^7 - 1
        assert_eq!(is_power_of_two(255).unwrap(), false); // 2^8 - 1

        // One more than powers of 2
        assert_eq!(is_power_of_two(3).unwrap(), false); // 2^1 + 1
        assert_eq!(is_power_of_two(5).unwrap(), false); // 2^2 + 1
        assert_eq!(is_power_of_two(9).unwrap(), false); // 2^3 + 1
        assert_eq!(is_power_of_two(17).unwrap(), false); // 2^4 + 1
        assert_eq!(is_power_of_two(33).unwrap(), false); // 2^5 + 1
        assert_eq!(is_power_of_two(65).unwrap(), false); // 2^6 + 1
        assert_eq!(is_power_of_two(129).unwrap(), false); // 2^7 + 1
    }

    #[test]
    fn test_negative_number_returns_error() {
        let result = is_power_of_two(-1);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "number must not be negative");
    }

    #[test]
    fn test_multiple_negative_numbers() {
        assert!(is_power_of_two(-1).is_err());
        assert!(is_power_of_two(-2).is_err());
        assert!(is_power_of_two(-4).is_err());
        assert!(is_power_of_two(-8).is_err());
        assert!(is_power_of_two(-100).is_err());
    }

    #[test]
    fn test_all_powers_of_two_up_to_30() {
        // Test 2^0 through 2^30
        for i in 0..=30 {
            let power = 1u32 << i; // 2^i
            assert_eq!(
                is_power_of_two(power as i32).unwrap(),
                true,
                "2^{i} = {power} should be a power of 2"
            );
        }
    }

    #[test]
    fn test_range_verification() {
        // Test that between consecutive powers of 2, only the powers return true
        for i in 1..10 {
            let power = 1 << i; // 2^i
            assert_eq!(is_power_of_two(power).unwrap(), true);

            // Check numbers between this power and the next
            let next_power = 1 << (i + 1);
            for num in (power + 1)..next_power {
                assert_eq!(
                    is_power_of_two(num).unwrap(),
                    false,
                    "{num} should not be a power of 2"
                );
            }
        }
    }

    #[test]
    fn test_bit_manipulation_correctness() {
        // Verify the bit manipulation logic for specific examples
        // For 8: 1000 & 0111 = 0000 ✓
        assert_eq!(8 & 7, 0);
        assert_eq!(is_power_of_two(8).unwrap(), true);

        // For 16: 10000 & 01111 = 00000 ✓
        assert_eq!(16 & 15, 0);
        assert_eq!(is_power_of_two(16).unwrap(), true);

        // For 6: 110 & 101 = 100 ✗
        assert_ne!(6 & 5, 0);
        assert_eq!(is_power_of_two(6).unwrap(), false);
    }

    #[test]
    fn test_edge_case_max_i32_power_of_two() {
        // Largest power of 2 that fits in i32: 2^30 = 1073741824
        assert_eq!(is_power_of_two(1073741824).unwrap(), true);
    }
}
