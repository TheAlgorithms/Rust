//! This module provides a function to reverse the bits of a 32-bit unsigned integer.
//!
//! The algorithm works by iterating through each of the 32 bits from least
//! significant to most significant, extracting each bit and placing it in the
//! reverse position.
//!
//! # Algorithm
//!
//! For each of the 32 bits:
//! 1. Shift the result left by 1 to make room for the next bit
//! 2. Extract the least significant bit of the input using bitwise AND with 1
//! 3. OR that bit into the result
//! 4. Shift the input right by 1 to process the next bit
//!
//! # Time Complexity
//!
//! O(1) - Always processes exactly 32 bits
//!
//! # Space Complexity
//!
//! O(1) - Uses a constant amount of extra space
//!
//! # Example
//!
//! ```
//! use the_algorithms_rust::bit_manipulation::reverse_bits;
//!
//! let n = 43261596;  // Binary: 00000010100101000001111010011100
//! let reversed = reverse_bits(n);
//! assert_eq!(reversed, 964176192);  // Binary: 00111001011110000010100101000000
//! ```

/// Reverses the bits of a 32-bit unsigned integer.
///
/// # Arguments
///
/// * `n` - A 32-bit unsigned integer whose bits are to be reversed
///
/// # Returns
///
/// A 32-bit unsigned integer with bits in reverse order
///
/// # Examples
///
/// ```
/// use the_algorithms_rust::bit_manipulation::reverse_bits;
///
/// let n = 43261596;  // 00000010100101000001111010011100 in binary
/// let result = reverse_bits(n);
/// assert_eq!(result, 964176192);  // 00111001011110000010100101000000 in binary
/// ```
///
/// ```
/// use the_algorithms_rust::bit_manipulation::reverse_bits;
///
/// let n = 1;  // 00000000000000000000000000000001 in binary
/// let result = reverse_bits(n);
/// assert_eq!(result, 2147483648);  // 10000000000000000000000000000000 in binary
/// ```
pub fn reverse_bits(n: u32) -> u32 {
    let mut result: u32 = 0;
    let mut num = n;

    // Process all 32 bits
    for _ in 0..32 {
        // Shift result left to make room for next bit
        result <<= 1;

        // Extract the least significant bit of num and add it to result
        result |= num & 1;

        // Shift num right to process the next bit
        num >>= 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_bits_basic() {
        // Test case 1: 43261596 (00000010100101000001111010011100)
        // Expected: 964176192 (00111001011110000010100101000000)
        assert_eq!(reverse_bits(43261596), 964176192);
    }

    #[test]
    fn test_reverse_bits_one() {
        // Test case 2: 1 (00000000000000000000000000000001)
        // Expected: 2147483648 (10000000000000000000000000000000)
        assert_eq!(reverse_bits(1), 2147483648);
    }

    #[test]
    fn test_reverse_bits_all_ones() {
        // Test case 3: 4294967293 (11111111111111111111111111111101)
        // Expected: 3221225471 (10111111111111111111111111111111)
        assert_eq!(reverse_bits(4294967293), 3221225471);
    }

    #[test]
    fn test_reverse_bits_zero() {
        // Test case 4: 0 (00000000000000000000000000000000)
        // Expected: 0 (00000000000000000000000000000000)
        assert_eq!(reverse_bits(0), 0);
    }

    #[test]
    fn test_reverse_bits_max() {
        // Test case 5: u32::MAX (11111111111111111111111111111111)
        // Expected: u32::MAX (11111111111111111111111111111111)
        assert_eq!(reverse_bits(u32::MAX), u32::MAX);
    }

    #[test]
    fn test_reverse_bits_alternating() {
        // Test case 6: 2863311530 (10101010101010101010101010101010)
        // Expected: 1431655765 (01010101010101010101010101010101)
        assert_eq!(reverse_bits(2863311530), 1431655765);
    }

    #[test]
    fn test_reverse_bits_symmetric() {
        // Test case 7: reversing twice should give original number
        let n = 12345678;
        assert_eq!(reverse_bits(reverse_bits(n)), n);
    }
}
