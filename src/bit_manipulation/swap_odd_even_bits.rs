/// Swaps odd and even bits in an integer.
///
/// This function separates the even bits (0, 2, 4, 6, etc.) and odd bits (1, 3, 5, 7, etc.)
/// using bitwise AND operations, then swaps them by shifting and combining with OR.
///
/// # Arguments
///
/// * `num` - A 32-bit unsigned integer
///
/// # Returns
///
/// A new integer with odd and even bits swapped
///
/// # Examples
///
/// ```
/// use the_algorithms_rust::bit_manipulation::swap_odd_even_bits;
///
/// assert_eq!(swap_odd_even_bits(0), 0);
/// assert_eq!(swap_odd_even_bits(1), 2);
/// assert_eq!(swap_odd_even_bits(2), 1);
/// assert_eq!(swap_odd_even_bits(3), 3);
/// assert_eq!(swap_odd_even_bits(4), 8);
/// assert_eq!(swap_odd_even_bits(5), 10);
/// assert_eq!(swap_odd_even_bits(6), 9);
/// assert_eq!(swap_odd_even_bits(23), 43);
/// ```
pub fn swap_odd_even_bits(num: u32) -> u32 {
    // Get all even bits - 0xAAAAAAAA is a 32-bit number with all even bits set to 1
    let even_bits = num & 0xAAAAAAAA;

    // Get all odd bits - 0x55555555 is a 32-bit number with all odd bits set to 1
    let odd_bits = num & 0x55555555;

    // Right shift even bits and left shift odd bits and swap them
    (even_bits >> 1) | (odd_bits << 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swap_odd_even_bits() {
        assert_eq!(swap_odd_even_bits(0), 0);
        assert_eq!(swap_odd_even_bits(1), 2);
        assert_eq!(swap_odd_even_bits(2), 1);
        assert_eq!(swap_odd_even_bits(3), 3);
        assert_eq!(swap_odd_even_bits(4), 8);
        assert_eq!(swap_odd_even_bits(5), 10);
        assert_eq!(swap_odd_even_bits(6), 9);
        assert_eq!(swap_odd_even_bits(23), 43);
        assert_eq!(swap_odd_even_bits(24), 36);
    }

    #[test]
    fn test_edge_cases() {
        // All bits set
        assert_eq!(swap_odd_even_bits(0xFFFFFFFF), 0xFFFFFFFF);

        // Alternating patterns
        assert_eq!(swap_odd_even_bits(0xAAAAAAAA), 0x55555555);
        assert_eq!(swap_odd_even_bits(0x55555555), 0xAAAAAAAA);
    }

    #[test]
    fn test_power_of_two() {
        assert_eq!(swap_odd_even_bits(16), 32);
        assert_eq!(swap_odd_even_bits(32), 16);
        assert_eq!(swap_odd_even_bits(64), 128);
    }
}
