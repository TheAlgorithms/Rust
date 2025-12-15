//! Two's Complement Representation
//!
//! Two's complement is a mathematical operation on binary numbers and a binary signed
//! number representation. It is widely used in computing as the most common method of
//! representing signed integers on computers.
//!
//! For more information: <https://en.wikipedia.org/wiki/Two%27s_complement>

/// Takes a negative integer and returns its two's complement binary representation.
///
/// The two's complement of a negative number is calculated by finding the binary
/// representation that, when added to the positive value with the same magnitude,
/// equals 2^n (where n is the number of bits).
///
/// # Arguments
///
/// * `number` - A non-positive integer (0 or negative)
///
/// # Returns
///
/// A `Result` containing:
/// - `Ok(String)` - The two's complement representation with "0b" prefix
/// - `Err(String)` - An error message if the input is positive
///
/// # Examples
///
/// ```
/// use the_algorithms_rust::bit_manipulation::twos_complement;
///
/// assert_eq!(twos_complement(0).unwrap(), "0b0");
/// assert_eq!(twos_complement(-1).unwrap(), "0b11");
/// assert_eq!(twos_complement(-5).unwrap(), "0b1011");
/// assert_eq!(twos_complement(-17).unwrap(), "0b101111");
/// assert_eq!(twos_complement(-207).unwrap(), "0b100110001");
///
/// // Positive numbers return an error
/// assert!(twos_complement(1).is_err());
/// ```
///
/// # Errors
///
/// Returns an error if the input number is positive.
pub fn twos_complement(number: i32) -> Result<String, String> {
    if number > 0 {
        return Err("input must be a negative integer".to_string());
    }

    if number == 0 {
        return Ok("0b0".to_string());
    }

    // Calculate the number of bits needed for the binary representation
    // (excluding the sign bit in the original representation)
    let binary_number_length = format!("{:b}", number.abs()).len();

    // Calculate two's complement value
    // This is equivalent to: abs(number) - 2^binary_number_length
    let twos_complement_value = (number.abs() as i64) - (1_i64 << binary_number_length);

    // Format as binary string (removing the negative sign)
    let mut twos_complement_str = format!("{:b}", twos_complement_value.abs());

    // Add leading zeros if necessary
    let padding_zeros = binary_number_length.saturating_sub(twos_complement_str.len());
    if padding_zeros > 0 {
        twos_complement_str = format!("{}{twos_complement_str}", "0".repeat(padding_zeros));
    }

    // Add leading '1' to indicate negative number in two's complement
    Ok(format!("0b1{twos_complement_str}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero() {
        assert_eq!(twos_complement(0).unwrap(), "0b0");
    }

    #[test]
    fn test_negative_one() {
        assert_eq!(twos_complement(-1).unwrap(), "0b11");
    }

    #[test]
    fn test_negative_five() {
        assert_eq!(twos_complement(-5).unwrap(), "0b1011");
    }

    #[test]
    fn test_negative_seventeen() {
        assert_eq!(twos_complement(-17).unwrap(), "0b101111");
    }

    #[test]
    fn test_negative_two_hundred_seven() {
        assert_eq!(twos_complement(-207).unwrap(), "0b100110001");
    }

    #[test]
    fn test_negative_small_values() {
        assert_eq!(twos_complement(-2).unwrap(), "0b110");
        assert_eq!(twos_complement(-3).unwrap(), "0b101");
        assert_eq!(twos_complement(-4).unwrap(), "0b1100");
    }

    #[test]
    fn test_negative_larger_values() {
        assert_eq!(twos_complement(-128).unwrap(), "0b110000000");
        assert_eq!(twos_complement(-255).unwrap(), "0b100000001");
        assert_eq!(twos_complement(-1000).unwrap(), "0b10000011000");
    }

    #[test]
    fn test_positive_number_returns_error() {
        let result = twos_complement(1);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "input must be a negative integer");
    }

    #[test]
    fn test_large_positive_number_returns_error() {
        let result = twos_complement(100);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "input must be a negative integer");
    }

    #[test]
    fn test_edge_case_negative_powers_of_two() {
        assert_eq!(twos_complement(-8).unwrap(), "0b11000");
        assert_eq!(twos_complement(-16).unwrap(), "0b110000");
        assert_eq!(twos_complement(-32).unwrap(), "0b1100000");
        assert_eq!(twos_complement(-64).unwrap(), "0b11000000");
    }
}
