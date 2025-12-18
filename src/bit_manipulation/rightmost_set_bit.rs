/// Finds the index (position) of the rightmost set bit in a number.
///
/// The index is 1-based, where position 1 is the least significant bit (rightmost).
/// This function uses the bitwise trick `n & -n` to isolate the rightmost set bit,
/// then calculates its position using logarithm base 2.
///
/// # Algorithm
///
/// 1. Use `n & -n` to isolate the rightmost set bit
/// 2. Calculate log2 of the result to get the 0-based position
/// 3. Add 1 to convert to 1-based indexing
///
/// # Arguments
///
/// * `num` - A positive integer
///
/// # Returns
///
/// * `Ok(u32)` - The 1-based position of the rightmost set bit
/// * `Err(String)` - An error message if the input is invalid
///
/// # Examples
///
/// ```
/// # use the_algorithms_rust::bit_manipulation::index_of_rightmost_set_bit;
/// // 18 in binary: 10010, rightmost set bit is at position 2
/// assert_eq!(index_of_rightmost_set_bit(18).unwrap(), 2);
///
/// // 12 in binary: 1100, rightmost set bit is at position 3
/// assert_eq!(index_of_rightmost_set_bit(12).unwrap(), 3);
///
/// // 5 in binary: 101, rightmost set bit is at position 1
/// assert_eq!(index_of_rightmost_set_bit(5).unwrap(), 1);
///
/// // 16 in binary: 10000, rightmost set bit is at position 5
/// assert_eq!(index_of_rightmost_set_bit(16).unwrap(), 5);
///
/// // 0 has no set bits
/// assert!(index_of_rightmost_set_bit(0).is_err());
/// ```
pub fn index_of_rightmost_set_bit(num: i32) -> Result<u32, String> {
    if num <= 0 {
        return Err("input must be a positive integer".to_string());
    }

    // Isolate the rightmost set bit using n & -n
    let rightmost_bit = num & -num;

    // Calculate position: log2(rightmost_bit) + 1
    // We use trailing_zeros which gives us the 0-based position
    // and add 1 to make it 1-based
    let position = rightmost_bit.trailing_zeros() + 1;

    Ok(position)
}

/// Alternative implementation using a different algorithm approach.
///
/// This version demonstrates the mathematical relationship between
/// the rightmost set bit position and log2.
///
/// # Examples
///
/// ```
/// # use the_algorithms_rust::bit_manipulation::index_of_rightmost_set_bit_log;
/// assert_eq!(index_of_rightmost_set_bit_log(18).unwrap(), 2);
/// assert_eq!(index_of_rightmost_set_bit_log(12).unwrap(), 3);
/// ```
pub fn index_of_rightmost_set_bit_log(num: i32) -> Result<u32, String> {
    if num <= 0 {
        return Err("input must be a positive integer".to_string());
    }

    // Isolate the rightmost set bit
    let rightmost_bit = num & -num;

    // Use f64 log2 and convert to position
    let position = (rightmost_bit as f64).log2() as u32 + 1;

    Ok(position)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_cases() {
        // 18 = 10010 in binary, rightmost set bit at position 2
        assert_eq!(index_of_rightmost_set_bit(18).unwrap(), 2);

        // 12 = 1100 in binary, rightmost set bit at position 3
        assert_eq!(index_of_rightmost_set_bit(12).unwrap(), 3);

        // 5 = 101 in binary, rightmost set bit at position 1
        assert_eq!(index_of_rightmost_set_bit(5).unwrap(), 1);
    }

    #[test]
    fn test_powers_of_two() {
        // 1 = 1 in binary, position 1
        assert_eq!(index_of_rightmost_set_bit(1).unwrap(), 1);

        // 2 = 10 in binary, position 2
        assert_eq!(index_of_rightmost_set_bit(2).unwrap(), 2);

        // 4 = 100 in binary, position 3
        assert_eq!(index_of_rightmost_set_bit(4).unwrap(), 3);

        // 8 = 1000 in binary, position 4
        assert_eq!(index_of_rightmost_set_bit(8).unwrap(), 4);

        // 16 = 10000 in binary, position 5
        assert_eq!(index_of_rightmost_set_bit(16).unwrap(), 5);

        // 32 = 100000 in binary, position 6
        assert_eq!(index_of_rightmost_set_bit(32).unwrap(), 6);
    }

    #[test]
    fn test_odd_numbers() {
        // All odd numbers have rightmost set bit at position 1
        assert_eq!(index_of_rightmost_set_bit(1).unwrap(), 1);
        assert_eq!(index_of_rightmost_set_bit(3).unwrap(), 1);
        assert_eq!(index_of_rightmost_set_bit(7).unwrap(), 1);
        assert_eq!(index_of_rightmost_set_bit(15).unwrap(), 1);
        assert_eq!(index_of_rightmost_set_bit(31).unwrap(), 1);
    }

    #[test]
    fn test_even_numbers() {
        // 6 = 110 in binary, rightmost set bit at position 2
        assert_eq!(index_of_rightmost_set_bit(6).unwrap(), 2);

        // 10 = 1010 in binary, rightmost set bit at position 2
        assert_eq!(index_of_rightmost_set_bit(10).unwrap(), 2);

        // 20 = 10100 in binary, rightmost set bit at position 3
        assert_eq!(index_of_rightmost_set_bit(20).unwrap(), 3);
    }

    #[test]
    fn test_zero() {
        assert!(index_of_rightmost_set_bit(0).is_err());
        assert_eq!(
            index_of_rightmost_set_bit(0).unwrap_err(),
            "input must be a positive integer"
        );
    }

    #[test]
    fn test_negative_numbers() {
        assert!(index_of_rightmost_set_bit(-1).is_err());
        assert!(index_of_rightmost_set_bit(-10).is_err());
        assert_eq!(
            index_of_rightmost_set_bit(-5).unwrap_err(),
            "input must be a positive integer"
        );
    }

    #[test]
    fn test_large_numbers() {
        // 1024 = 10000000000 in binary, position 11
        assert_eq!(index_of_rightmost_set_bit(1024).unwrap(), 11);

        // 1023 = 1111111111 in binary, position 1
        assert_eq!(index_of_rightmost_set_bit(1023).unwrap(), 1);

        // 2048 = 100000000000 in binary, position 12
        assert_eq!(index_of_rightmost_set_bit(2048).unwrap(), 12);
    }

    #[test]
    fn test_consecutive_numbers() {
        // Testing a range to ensure correctness
        assert_eq!(index_of_rightmost_set_bit(14).unwrap(), 2); // 1110
        assert_eq!(index_of_rightmost_set_bit(15).unwrap(), 1); // 1111
        assert_eq!(index_of_rightmost_set_bit(16).unwrap(), 5); // 10000
        assert_eq!(index_of_rightmost_set_bit(17).unwrap(), 1); // 10001
    }

    #[test]
    fn test_log_version() {
        // Test the alternative log-based implementation
        assert_eq!(index_of_rightmost_set_bit_log(18).unwrap(), 2);
        assert_eq!(index_of_rightmost_set_bit_log(12).unwrap(), 3);
        assert_eq!(index_of_rightmost_set_bit_log(5).unwrap(), 1);
        assert_eq!(index_of_rightmost_set_bit_log(16).unwrap(), 5);
    }

    #[test]
    fn test_both_implementations_match() {
        // Verify both implementations give the same results
        for i in 1..=100 {
            assert_eq!(
                index_of_rightmost_set_bit(i).unwrap(),
                index_of_rightmost_set_bit_log(i).unwrap()
            );
        }
    }
}
