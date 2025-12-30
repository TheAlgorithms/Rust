//! Hamming Distance
//!
//! This module implements the [Hamming distance](https://en.wikipedia.org/wiki/Hamming_distance)
//! algorithm for both integers and strings.
//!
//! The Hamming distance between two values is the number of positions at which
//! the corresponding symbols differ.

/// Counts the number of set bits (1s) in a 64-bit unsigned integer.
///
/// # Arguments
///
/// * `value` - The number to count set bits in
///
/// # Returns
///
/// The number of set bits in the value
///
/// # Example
///
/// ```
/// // This is a private helper function
/// let value: u64 = 11; // 1011 in binary has 3 set bits
/// ```
fn bit_count(mut value: u64) -> u64 {
    let mut count = 0;
    while value != 0 {
        if value & 1 == 1 {
            count += 1;
        }
        value >>= 1;
    }
    count
}

/// Calculates the Hamming distance between two unsigned 64-bit integers.
///
/// The Hamming distance is the number of bit positions at which the
/// corresponding bits differ. This is computed by taking the XOR of the
/// two numbers and counting the set bits.
///
/// # Arguments
///
/// * `a` - The first integer
/// * `b` - The second integer
///
/// # Returns
///
/// The number of differing bits between `a` and `b`
///
/// # Example
///
/// ```
/// use the_algorithms_rust::bit_manipulation::hamming_distance;
///
/// let distance = hamming_distance(11, 2);
/// assert_eq!(distance, 2);
/// ```
pub fn hamming_distance(a: u64, b: u64) -> u64 {
    bit_count(a ^ b)
}

/// Calculates the Hamming distance between two strings of equal length.
///
/// The Hamming distance is the number of positions at which the
/// corresponding characters differ.
///
/// # Arguments
///
/// * `a` - The first string
/// * `b` - The second string
///
/// # Returns
///
/// The number of differing characters between `a` and `b`
///
/// # Panics
///
/// Panics if the strings have different lengths
///
/// # Example
///
/// ```
/// use the_algorithms_rust::bit_manipulation::hamming_distance_str;
///
/// let distance = hamming_distance_str("1101", "1111");
/// assert_eq!(distance, 1);
/// ```
pub fn hamming_distance_str(a: &str, b: &str) -> u64 {
    assert_eq!(
        a.len(),
        b.len(),
        "Strings must have the same length for Hamming distance calculation"
    );

    a.chars()
        .zip(b.chars())
        .filter(|(ch_a, ch_b)| ch_a != ch_b)
        .count() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bit_count() {
        assert_eq!(bit_count(0), 0);
        assert_eq!(bit_count(11), 3); // 1011 in binary
        assert_eq!(bit_count(15), 4); // 1111 in binary
    }

    #[test]
    fn test_hamming_distance_integers() {
        assert_eq!(hamming_distance(11, 2), 2);
        assert_eq!(hamming_distance(2, 0), 1);
        assert_eq!(hamming_distance(11, 0), 3);
        assert_eq!(hamming_distance(0, 0), 0);
    }

    #[test]
    fn test_hamming_distance_strings() {
        assert_eq!(hamming_distance_str("1101", "1111"), 1);
        assert_eq!(hamming_distance_str("1111", "1111"), 0);
        assert_eq!(hamming_distance_str("0000", "1111"), 4);
        assert_eq!(hamming_distance_str("alpha", "alphb"), 1);
        assert_eq!(hamming_distance_str("abcd", "abcd"), 0);
        assert_eq!(hamming_distance_str("dcba", "abcd"), 4);
    }

    #[test]
    #[should_panic(expected = "Strings must have the same length")]
    fn test_hamming_distance_strings_different_lengths() {
        hamming_distance_str("abc", "abcd");
    }
}
