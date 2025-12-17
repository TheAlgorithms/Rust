/// Counts the number of trailing zeros in the binary representation of a number
///
/// # Arguments
///
/// * `num` - The input number
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
/// assert_eq!(binary_count_trailing_zeros(25), 0);
/// assert_eq!(binary_count_trailing_zeros(36), 2);
/// assert_eq!(binary_count_trailing_zeros(16), 4);
/// assert_eq!(binary_count_trailing_zeros(58), 1);
/// ```
pub fn binary_count_trailing_zeros(num: u64) -> u32 {
    if num == 0 {
        return 0;
    }
    num.trailing_zeros()
}

/// Alternative implementation using bit manipulation
///
/// Uses the bit manipulation trick: log2(num & -num)
///
/// # Examples
///
/// ```
/// # use the_algorithms_rust::bit_manipulation::binary_count_trailing_zeros_bitwise;
/// assert_eq!(binary_count_trailing_zeros_bitwise(25), 0);
/// assert_eq!(binary_count_trailing_zeros_bitwise(36), 2);
/// ```
pub fn binary_count_trailing_zeros_bitwise(num: u64) -> u32 {
    if num == 0 {
        return 0;
    }
    
    let rightmost_set_bit = num & (num.wrapping_neg());
    63 - rightmost_set_bit.leading_zeros()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_cases() {
        assert_eq!(binary_count_trailing_zeros(25), 0);
        assert_eq!(binary_count_trailing_zeros(36), 2);
        assert_eq!(binary_count_trailing_zeros(16), 4);
        assert_eq!(binary_count_trailing_zeros(58), 1);
        assert_eq!(binary_count_trailing_zeros(4294967296), 32);
    }

    #[test]
    fn test_zero() {
        assert_eq!(binary_count_trailing_zeros(0), 0);
    }

    #[test]
    fn test_powers_of_two() {
        assert_eq!(binary_count_trailing_zeros(1), 0);
        assert_eq!(binary_count_trailing_zeros(2), 1);
        assert_eq!(binary_count_trailing_zeros(4), 2);
        assert_eq!(binary_count_trailing_zeros(8), 3);
        assert_eq!(binary_count_trailing_zeros(1024), 10);
    }

    #[test]
    fn test_bitwise_implementation() {
        let test_cases = vec![0, 1, 2, 4, 8, 16, 25, 36, 58, 1024, 4294967296];
        
        for num in test_cases {
            assert_eq!(
                binary_count_trailing_zeros(num),
                binary_count_trailing_zeros_bitwise(num),
                "Mismatch for input: {num}"
            );
        }
    }
}
