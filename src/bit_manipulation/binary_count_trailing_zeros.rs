/// Counts the number of trailing zeros in the binary representation of a number
///
/// Takes an unsigned integer and returns the count of trailing zeros (consecutive
/// zeros from the least significant bit) in its binary representation.
///
/// # Arguments
///
/// * `num` - The input number (unsigned integer)
///
/// # Returns
///
/// The number of trailing zeros in the binary representation
///
/// # Examples
///
/// ```
/// use the_algorithms_rust::bit_manipulation::binary_count_trailing_zeros;
///
/// assert_eq!(binary_count_trailing_zeros(25), 0);   // 11001 -> 0 trailing zeros
/// assert_eq!(binary_count_trailing_zeros(36), 2);   // 100100 -> 2 trailing zeros
/// assert_eq!(binary_count_trailing_zeros(16), 4);   // 10000 -> 4 trailing zeros
/// assert_eq!(binary_count_trailing_zeros(58), 1);   // 111010 -> 1 trailing zero
/// assert_eq!(binary_count_trailing_zeros(4294967296), 32);
/// assert_eq!(binary_count_trailing_zeros(0), 0);
/// ```
///
/// # Algorithm Explanation
///
/// This function uses Rust's built-in `trailing_zeros()` method, which efficiently
/// counts trailing zeros using CPU instructions. The method works by:
/// 1. Examining the binary representation from right to left
/// 2. Counting consecutive zeros until hitting the first 1 bit
/// 3. Returning the count
///
/// Alternative implementation using bit manipulation:
/// The count can also be computed using: `log2(num & -num)`
/// - `num & -num` isolates the rightmost set bit
/// - Taking log2 gives the position of that bit (= trailing zeros count)
pub fn binary_count_trailing_zeros(num: u64) -> u32 {
    // Handle zero case explicitly to match Python behavior
    if num == 0 {
        return 0;
    }

    // Rust's built-in method for counting trailing zeros
    // Note: trailing_zeros() on 0 returns the bit width (64 for u64)
    // but we handle zero separately above
    num.trailing_zeros()
}

/// Alternative implementation using bit manipulation and logarithm
///
/// This matches the Python implementation more closely by using the
/// bit manipulation trick: log2(num & -num)
///
/// # Examples
///
/// ```
/// # use the_algorithms_rust::bit_manipulation::binary_count_trailing_zeros;
/// // Note: This is an internal alternative implementation
/// // For actual use, call binary_count_trailing_zeros() instead
/// assert_eq!(binary_count_trailing_zeros(25), 0);
/// assert_eq!(binary_count_trailing_zeros(36), 2);
/// assert_eq!(binary_count_trailing_zeros(16), 4);
/// ```
#[allow(dead_code)]
pub fn binary_count_trailing_zeros_bitwise(num: u64) -> u32 {
    if num == 0 {
        return 0;
    }

    // The bit manipulation trick: num & -num isolates the rightmost set bit
    // Taking log2 gives us the position (number of trailing zeros)
    let rightmost_set_bit = num & (num.wrapping_neg());

    // Calculate log2 using leading zeros
    // log2(x) = 63 - leading_zeros(x) for u64
    63 - rightmost_set_bit.leading_zeros()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_cases() {
        assert_eq!(binary_count_trailing_zeros(25), 0); // 11001
        assert_eq!(binary_count_trailing_zeros(36), 2); // 100100
        assert_eq!(binary_count_trailing_zeros(16), 4); // 10000
        assert_eq!(binary_count_trailing_zeros(58), 1); // 111010
    }

    #[test]
    fn test_large_number() {
        assert_eq!(binary_count_trailing_zeros(4294967296), 32);
    }

    #[test]
    fn test_zero() {
        assert_eq!(binary_count_trailing_zeros(0), 0);
    }

    #[test]
    fn test_powers_of_two() {
        // Powers of 2 have trailing zeros equal to their exponent
        assert_eq!(binary_count_trailing_zeros(1), 0); // 2^0
        assert_eq!(binary_count_trailing_zeros(2), 1); // 2^1
        assert_eq!(binary_count_trailing_zeros(4), 2); // 2^2
        assert_eq!(binary_count_trailing_zeros(8), 3); // 2^3
        assert_eq!(binary_count_trailing_zeros(1024), 10); // 2^10
    }

    #[test]
    fn test_odd_numbers() {
        // Odd numbers always have 0 trailing zeros
        assert_eq!(binary_count_trailing_zeros(1), 0);
        assert_eq!(binary_count_trailing_zeros(3), 0);
        assert_eq!(binary_count_trailing_zeros(5), 0);
        assert_eq!(binary_count_trailing_zeros(7), 0);
        assert_eq!(binary_count_trailing_zeros(99), 0);
    }

    #[test]
    fn test_even_numbers() {
        assert_eq!(binary_count_trailing_zeros(2), 1); // 10
        assert_eq!(binary_count_trailing_zeros(4), 2); // 100
        assert_eq!(binary_count_trailing_zeros(6), 1); // 110
        assert_eq!(binary_count_trailing_zeros(8), 3); // 1000
        assert_eq!(binary_count_trailing_zeros(12), 2); // 1100
    }

    #[test]
    fn test_bitwise_implementation() {
        // Test that both implementations give the same results
        let test_cases = vec![0, 1, 2, 4, 8, 16, 25, 36, 58, 1024, 4294967296];

        for num in test_cases {
            assert_eq!(
                binary_count_trailing_zeros(num),
                binary_count_trailing_zeros_bitwise(num),
                "Mismatch for input: {}",
                num
            );
        }
    }

    #[test]
    fn test_max_value() {
        // Test with maximum u64 value
        let max_u64 = u64::MAX;
        assert_eq!(binary_count_trailing_zeros(max_u64), 0); // All 1s, no trailing zeros
    }

    #[test]
    fn test_patterns() {
        // Test specific binary patterns
        assert_eq!(binary_count_trailing_zeros(0b1000), 3);
        assert_eq!(binary_count_trailing_zeros(0b10000), 4);
        assert_eq!(binary_count_trailing_zeros(0b100000), 5);
        assert_eq!(binary_count_trailing_zeros(0b1111000), 3);
    }
}
