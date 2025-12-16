//! Previous Power of Two
//!
//! This module provides a function to find the largest power of two that is less than
//! or equal to a given non-negative integer.
//!
//! # Algorithm
//!
//! The algorithm works by repeatedly left-shifting (doubling) a power value starting
//! from 1 until it exceeds the input number, then returning the previous power (by
//! right-shifting once).
//!
//! For more information: <https://stackoverflow.com/questions/1322510>

/// Finds the largest power of two that is less than or equal to a given integer.
///
/// The function uses bit shifting to efficiently find the power of two. It starts
/// with 1 and keeps doubling (left shift) until it exceeds the input, then returns
/// the previous value (right shift).
///
/// # Arguments
///
/// * `number` - A non-negative integer
///
/// # Returns
///
/// A `Result` containing:
/// - `Ok(u32)` - The largest power of two ≤ the input number
/// - `Err(String)` - An error message if the input is negative
///
/// # Examples
///
/// ```
/// use the_algorithms_rust::bit_manipulation::find_previous_power_of_two;
///
/// assert_eq!(find_previous_power_of_two(0).unwrap(), 0);
/// assert_eq!(find_previous_power_of_two(1).unwrap(), 1);
/// assert_eq!(find_previous_power_of_two(2).unwrap(), 2);
/// assert_eq!(find_previous_power_of_two(3).unwrap(), 2);
/// assert_eq!(find_previous_power_of_two(4).unwrap(), 4);
/// assert_eq!(find_previous_power_of_two(5).unwrap(), 4);
/// assert_eq!(find_previous_power_of_two(8).unwrap(), 8);
/// assert_eq!(find_previous_power_of_two(15).unwrap(), 8);
/// assert_eq!(find_previous_power_of_two(16).unwrap(), 16);
/// assert_eq!(find_previous_power_of_two(17).unwrap(), 16);
///
/// // Negative numbers return an error
/// assert!(find_previous_power_of_two(-5).is_err());
/// ```
///
/// # Errors
///
/// Returns an error if the input number is negative.
pub fn find_previous_power_of_two(number: i32) -> Result<u32, String> {
    if number < 0 {
        return Err("Input must be a non-negative integer".to_string());
    }

    let number = number as u32;

    if number == 0 {
        return Ok(0);
    }

    let mut power = 1u32;
    while power <= number {
        power <<= 1; // Equivalent to multiplying by 2
    }

    Ok(if number > 1 { power >> 1 } else { 1 })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero() {
        assert_eq!(find_previous_power_of_two(0).unwrap(), 0);
    }

    #[test]
    fn test_one() {
        assert_eq!(find_previous_power_of_two(1).unwrap(), 1);
    }

    #[test]
    fn test_powers_of_two() {
        assert_eq!(find_previous_power_of_two(2).unwrap(), 2);
        assert_eq!(find_previous_power_of_two(4).unwrap(), 4);
        assert_eq!(find_previous_power_of_two(8).unwrap(), 8);
        assert_eq!(find_previous_power_of_two(16).unwrap(), 16);
        assert_eq!(find_previous_power_of_two(32).unwrap(), 32);
        assert_eq!(find_previous_power_of_two(64).unwrap(), 64);
        assert_eq!(find_previous_power_of_two(128).unwrap(), 128);
        assert_eq!(find_previous_power_of_two(256).unwrap(), 256);
        assert_eq!(find_previous_power_of_two(512).unwrap(), 512);
        assert_eq!(find_previous_power_of_two(1024).unwrap(), 1024);
    }

    #[test]
    fn test_numbers_between_powers() {
        // Between 2 and 4
        assert_eq!(find_previous_power_of_two(3).unwrap(), 2);

        // Between 4 and 8
        assert_eq!(find_previous_power_of_two(5).unwrap(), 4);
        assert_eq!(find_previous_power_of_two(6).unwrap(), 4);
        assert_eq!(find_previous_power_of_two(7).unwrap(), 4);

        // Between 8 and 16
        assert_eq!(find_previous_power_of_two(9).unwrap(), 8);
        assert_eq!(find_previous_power_of_two(10).unwrap(), 8);
        assert_eq!(find_previous_power_of_two(11).unwrap(), 8);
        assert_eq!(find_previous_power_of_two(12).unwrap(), 8);
        assert_eq!(find_previous_power_of_two(13).unwrap(), 8);
        assert_eq!(find_previous_power_of_two(14).unwrap(), 8);
        assert_eq!(find_previous_power_of_two(15).unwrap(), 8);

        // Between 16 and 32
        assert_eq!(find_previous_power_of_two(17).unwrap(), 16);
        assert_eq!(find_previous_power_of_two(20).unwrap(), 16);
        assert_eq!(find_previous_power_of_two(31).unwrap(), 16);
    }

    #[test]
    fn test_range_0_to_17() {
        // Test the exact output from the Python docstring
        let expected = vec![0, 1, 2, 2, 4, 4, 4, 4, 8, 8, 8, 8, 8, 8, 8, 8, 16, 16];
        let results: Vec<u32> = (0..18)
            .map(|i| find_previous_power_of_two(i).unwrap())
            .collect();
        assert_eq!(results, expected);
    }

    #[test]
    fn test_large_numbers() {
        assert_eq!(find_previous_power_of_two(100).unwrap(), 64);
        assert_eq!(find_previous_power_of_two(500).unwrap(), 256);
        assert_eq!(find_previous_power_of_two(1000).unwrap(), 512);
        assert_eq!(find_previous_power_of_two(2000).unwrap(), 1024);
        assert_eq!(find_previous_power_of_two(10000).unwrap(), 8192);
    }

    #[test]
    fn test_max_safe_values() {
        assert_eq!(find_previous_power_of_two(1023).unwrap(), 512);
        assert_eq!(find_previous_power_of_two(2047).unwrap(), 1024);
        assert_eq!(find_previous_power_of_two(4095).unwrap(), 2048);
    }

    #[test]
    fn test_negative_number_returns_error() {
        let result = find_previous_power_of_two(-1);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Input must be a non-negative integer");
    }

    #[test]
    fn test_negative_numbers_return_errors() {
        assert!(find_previous_power_of_two(-5).is_err());
        assert!(find_previous_power_of_two(-10).is_err());
        assert!(find_previous_power_of_two(-100).is_err());
    }

    #[test]
    fn test_edge_cases() {
        // One less than powers of two
        assert_eq!(find_previous_power_of_two(127).unwrap(), 64);
        assert_eq!(find_previous_power_of_two(255).unwrap(), 128);
        assert_eq!(find_previous_power_of_two(511).unwrap(), 256);
    }
}
