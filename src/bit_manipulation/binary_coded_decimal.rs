//! Binary Coded Decimal (BCD) conversion
//!
//! This module provides a function to convert decimal integers to Binary Coded Decimal (BCD) format.
//! In BCD, each decimal digit is represented by its 4-bit binary equivalent.
//!
//! # Examples
//!
//! ```
//! use the_algorithms_rust::bit_manipulation::binary_coded_decimal;
//!
//! assert_eq!(binary_coded_decimal(12), "0b00010010");
//! assert_eq!(binary_coded_decimal(987), "0b100110000111");
//! ```

use std::fmt::Write;

/// Converts a decimal integer to Binary Coded Decimal (BCD) format.
///
/// Each digit of the input number is represented by a 4-bit binary value.
/// Negative numbers are treated as 0.
///
/// # Arguments
///
/// * `number` - An integer to be converted to BCD format
///
/// # Returns
///
/// A `String` representing the BCD encoding with "0b" prefix
///
/// # Examples
///
/// ```
/// use the_algorithms_rust::bit_manipulation::binary_coded_decimal;
///
/// assert_eq!(binary_coded_decimal(0), "0b0000");
/// assert_eq!(binary_coded_decimal(3), "0b0011");
/// assert_eq!(binary_coded_decimal(12), "0b00010010");
/// assert_eq!(binary_coded_decimal(987), "0b100110000111");
/// assert_eq!(binary_coded_decimal(-5), "0b0000");
/// ```
///
/// # Algorithm
///
/// 1. Convert the number to its absolute value (negative numbers become 0)
/// 2. For each decimal digit:
///    - Convert the digit to binary
///    - Pad to 4 bits with leading zeros
///    - Concatenate to the result
/// 3. Prepend "0b" to the final binary string
pub fn binary_coded_decimal(number: i32) -> String {
    // Handle negative numbers by converting to 0
    let num = if number < 0 { 0 } else { number };

    // Convert to string to process each digit
    let digits = num.to_string();

    // Build the BCD string using fold for efficiency
    let bcd = digits.chars().fold(String::new(), |mut acc, digit| {
        // Convert char to digit value and format as 4-bit binary
        let digit_value = digit.to_digit(10).unwrap();
        write!(acc, "{digit_value:04b}").unwrap();
        acc
    });

    format!("0b{bcd}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero() {
        assert_eq!(binary_coded_decimal(0), "0b0000");
    }

    #[test]
    fn test_single_digit() {
        assert_eq!(binary_coded_decimal(1), "0b0001");
        assert_eq!(binary_coded_decimal(2), "0b0010");
        assert_eq!(binary_coded_decimal(3), "0b0011");
        assert_eq!(binary_coded_decimal(4), "0b0100");
        assert_eq!(binary_coded_decimal(5), "0b0101");
        assert_eq!(binary_coded_decimal(6), "0b0110");
        assert_eq!(binary_coded_decimal(7), "0b0111");
        assert_eq!(binary_coded_decimal(8), "0b1000");
        assert_eq!(binary_coded_decimal(9), "0b1001");
    }

    #[test]
    fn test_two_digits() {
        assert_eq!(binary_coded_decimal(10), "0b00010000");
        assert_eq!(binary_coded_decimal(12), "0b00010010");
        assert_eq!(binary_coded_decimal(25), "0b00100101");
        assert_eq!(binary_coded_decimal(99), "0b10011001");
    }

    #[test]
    fn test_three_digits() {
        assert_eq!(binary_coded_decimal(100), "0b000100000000");
        assert_eq!(binary_coded_decimal(123), "0b000100100011");
        assert_eq!(binary_coded_decimal(456), "0b010001010110");
        assert_eq!(binary_coded_decimal(987), "0b100110000111");
    }

    #[test]
    fn test_large_numbers() {
        assert_eq!(binary_coded_decimal(1234), "0b0001001000110100");
        assert_eq!(binary_coded_decimal(9999), "0b1001100110011001");
    }

    #[test]
    fn test_negative_numbers() {
        // Negative numbers should be treated as 0
        assert_eq!(binary_coded_decimal(-1), "0b0000");
        assert_eq!(binary_coded_decimal(-2), "0b0000");
        assert_eq!(binary_coded_decimal(-100), "0b0000");
    }

    #[test]
    fn test_each_digit_encoding() {
        // Verify that each digit is encoded correctly in a multi-digit number
        // 67 should be: 6 (0110) and 7 (0111)
        assert_eq!(binary_coded_decimal(67), "0b01100111");

        // 305 should be: 3 (0011), 0 (0000), 5 (0101)
        assert_eq!(binary_coded_decimal(305), "0b001100000101");
    }
}
