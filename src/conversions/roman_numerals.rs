//! Roman Numeral Conversion
//!
//! This module provides conversion between Roman numerals and integers.
//!
//! Roman numerals use combinations of letters from the Latin alphabet:
//! I, V, X, L, C, D, and M to represent numbers.
//!
//! # Rules
//!
//! - I = 1, V = 5, X = 10, L = 50, C = 100, D = 500, M = 1000
//! - When a smaller value appears before a larger value, subtract the smaller
//!   (e.g., IV = 4, IX = 9)
//! - When a smaller value appears after a larger value, add the smaller
//!   (e.g., VI = 6, XI = 11)
//!
//! # References
//!
//! - [Roman Numerals - Wikipedia](https://en.wikipedia.org/wiki/Roman_numerals)
//! - [LeetCode #13 - Roman to Integer](https://leetcode.com/problems/roman-to-integer/)

/// Roman numeral symbols and their corresponding values in descending order.
/// Used for conversion from integer to Roman numeral.
const ROMAN_NUMERALS: [(u32, &str); 13] = [
    (1000, "M"),
    (900, "CM"),
    (500, "D"),
    (400, "CD"),
    (100, "C"),
    (90, "XC"),
    (50, "L"),
    (40, "XL"),
    (10, "X"),
    (9, "IX"),
    (5, "V"),
    (4, "IV"),
    (1, "I"),
];

/// Converts a Roman numeral string to an integer.
///
/// # Arguments
///
/// * `roman` - A string slice containing a valid Roman numeral
///
/// # Returns
///
/// `Ok(u32)` with the integer value, or `Err(String)` if the input is invalid
///
/// # Rules
///
/// - Valid Roman numerals are in range 1-3999
/// - Uses standard subtractive notation (IV, IX, XL, XC, CD, CM)
/// - Input must contain only valid Roman numeral characters: I, V, X, L, C, D, M
///
/// # Example
///
/// ```
/// use the_algorithms_rust::conversions::roman_to_int;
///
/// assert_eq!(roman_to_int("III").unwrap(), 3);
/// assert_eq!(roman_to_int("CLIV").unwrap(), 154);
/// assert_eq!(roman_to_int("MIX").unwrap(), 1009);
/// assert_eq!(roman_to_int("MMMCMXCIX").unwrap(), 3999);
///
/// // Invalid input returns error
/// assert!(roman_to_int("INVALID").is_err());
/// ```
pub fn roman_to_int(roman: &str) -> Result<u32, String> {
    if roman.is_empty() {
        return Err("Roman numeral cannot be empty".to_string());
    }

    // Convert to uppercase for case-insensitive processing
    let roman = roman.to_uppercase();
    let chars: Vec<char> = roman.chars().collect();

    // Validate that all characters are valid Roman numerals
    for ch in &chars {
        if !matches!(ch, 'I' | 'V' | 'X' | 'L' | 'C' | 'D' | 'M') {
            return Err(format!("Invalid Roman numeral character: '{ch}'"));
        }
    }

    let mut total: u32 = 0;
    let mut place = 0;

    while place < chars.len() {
        let current_val = char_to_value(chars[place]);

        // Check if we need to use subtractive notation
        if place + 1 < chars.len() {
            let next_val = char_to_value(chars[place + 1]);

            if current_val < next_val {
                // Subtractive case (e.g., IV, IX, XL, XC, CD, CM)
                total += next_val - current_val;
                place += 2;
                continue;
            }
        }

        // Normal case - just add the value
        total += current_val;
        place += 1;
    }

    if total == 0 || total > 3999 {
        return Err(format!(
            "Result {total} is out of valid range (1-3999) for Roman numerals"
        ));
    }

    Ok(total)
}

/// Converts an integer to a Roman numeral string.
///
/// # Arguments
///
/// * `number` - An integer in the range 1-3999
///
/// # Returns
///
/// `Ok(String)` with the Roman numeral representation, or `Err(String)` if out of range
///
/// # Rules
///
/// - Valid input range is 1-3999
/// - Uses standard subtractive notation (IV, IX, XL, XC, CD, CM)
/// - Returns the shortest possible representation
///
/// # Example
///
/// ```
/// use the_algorithms_rust::conversions::int_to_roman;
///
/// assert_eq!(int_to_roman(3).unwrap(), "III");
/// assert_eq!(int_to_roman(154).unwrap(), "CLIV");
/// assert_eq!(int_to_roman(1009).unwrap(), "MIX");
/// assert_eq!(int_to_roman(3999).unwrap(), "MMMCMXCIX");
///
/// // Out of range returns error
/// assert!(int_to_roman(0).is_err());
/// assert!(int_to_roman(4000).is_err());
/// ```
pub fn int_to_roman(mut number: u32) -> Result<String, String> {
    if number == 0 || number > 3999 {
        return Err(format!(
            "Number {number} is out of valid range (1-3999) for Roman numerals"
        ));
    }

    let mut result = String::new();

    for (value, numeral) in ROMAN_NUMERALS.iter() {
        let count = number / value;
        if count > 0 {
            result.push_str(&numeral.repeat(count as usize));
            number %= value;
        }

        if number == 0 {
            break;
        }
    }

    Ok(result)
}

/// Helper function to convert a Roman numeral character to its integer value.
///
/// # Arguments
///
/// * `ch` - A Roman numeral character (I, V, X, L, C, D, M)
///
/// # Returns
///
/// The integer value of the character
///
/// # Panics
///
/// Panics if an invalid character is provided (this should be caught by validation)
fn char_to_value(ch: char) -> u32 {
    match ch {
        'I' => 1,
        'V' => 5,
        'X' => 10,
        'L' => 50,
        'C' => 100,
        'D' => 500,
        'M' => 1000,
        _ => panic!("Invalid Roman numeral character: {ch}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roman_to_int_basic() {
        assert_eq!(roman_to_int("I").unwrap(), 1);
        assert_eq!(roman_to_int("V").unwrap(), 5);
        assert_eq!(roman_to_int("X").unwrap(), 10);
        assert_eq!(roman_to_int("L").unwrap(), 50);
        assert_eq!(roman_to_int("C").unwrap(), 100);
        assert_eq!(roman_to_int("D").unwrap(), 500);
        assert_eq!(roman_to_int("M").unwrap(), 1000);
    }

    #[test]
    fn test_roman_to_int_additive() {
        assert_eq!(roman_to_int("II").unwrap(), 2);
        assert_eq!(roman_to_int("III").unwrap(), 3);
        assert_eq!(roman_to_int("VI").unwrap(), 6);
        assert_eq!(roman_to_int("VII").unwrap(), 7);
        assert_eq!(roman_to_int("VIII").unwrap(), 8);
        assert_eq!(roman_to_int("XI").unwrap(), 11);
        assert_eq!(roman_to_int("XV").unwrap(), 15);
        assert_eq!(roman_to_int("XX").unwrap(), 20);
        assert_eq!(roman_to_int("XXX").unwrap(), 30);
    }

    #[test]
    fn test_roman_to_int_subtractive() {
        assert_eq!(roman_to_int("IV").unwrap(), 4);
        assert_eq!(roman_to_int("IX").unwrap(), 9);
        assert_eq!(roman_to_int("XL").unwrap(), 40);
        assert_eq!(roman_to_int("XC").unwrap(), 90);
        assert_eq!(roman_to_int("CD").unwrap(), 400);
        assert_eq!(roman_to_int("CM").unwrap(), 900);
    }

    #[test]
    fn test_roman_to_int_complex() {
        assert_eq!(roman_to_int("CLIV").unwrap(), 154);
        assert_eq!(roman_to_int("MCMXC").unwrap(), 1990);
        assert_eq!(roman_to_int("MMXIV").unwrap(), 2014);
        assert_eq!(roman_to_int("MIX").unwrap(), 1009);
        assert_eq!(roman_to_int("MMD").unwrap(), 2500);
        assert_eq!(roman_to_int("MMMCMXCIX").unwrap(), 3999);
    }

    #[test]
    fn test_roman_to_int_case_insensitive() {
        assert_eq!(roman_to_int("iii").unwrap(), 3);
        assert_eq!(roman_to_int("Cliv").unwrap(), 154);
        assert_eq!(roman_to_int("mIx").unwrap(), 1009);
    }

    #[test]
    fn test_roman_to_int_invalid_character() {
        assert!(roman_to_int("INVALID").is_err());
        assert!(roman_to_int("XYZ").is_err());
        assert!(roman_to_int("123").is_err());
        assert!(roman_to_int("X5").is_err());
    }

    #[test]
    fn test_roman_to_int_empty() {
        assert!(roman_to_int("").is_err());
    }

    #[test]
    fn test_int_to_roman_basic() {
        assert_eq!(int_to_roman(1).unwrap(), "I");
        assert_eq!(int_to_roman(5).unwrap(), "V");
        assert_eq!(int_to_roman(10).unwrap(), "X");
        assert_eq!(int_to_roman(50).unwrap(), "L");
        assert_eq!(int_to_roman(100).unwrap(), "C");
        assert_eq!(int_to_roman(500).unwrap(), "D");
        assert_eq!(int_to_roman(1000).unwrap(), "M");
    }

    #[test]
    fn test_int_to_roman_additive() {
        assert_eq!(int_to_roman(2).unwrap(), "II");
        assert_eq!(int_to_roman(3).unwrap(), "III");
        assert_eq!(int_to_roman(6).unwrap(), "VI");
        assert_eq!(int_to_roman(7).unwrap(), "VII");
        assert_eq!(int_to_roman(8).unwrap(), "VIII");
        assert_eq!(int_to_roman(11).unwrap(), "XI");
        assert_eq!(int_to_roman(15).unwrap(), "XV");
        assert_eq!(int_to_roman(20).unwrap(), "XX");
        assert_eq!(int_to_roman(30).unwrap(), "XXX");
    }

    #[test]
    fn test_int_to_roman_subtractive() {
        assert_eq!(int_to_roman(4).unwrap(), "IV");
        assert_eq!(int_to_roman(9).unwrap(), "IX");
        assert_eq!(int_to_roman(40).unwrap(), "XL");
        assert_eq!(int_to_roman(90).unwrap(), "XC");
        assert_eq!(int_to_roman(400).unwrap(), "CD");
        assert_eq!(int_to_roman(900).unwrap(), "CM");
    }

    #[test]
    fn test_int_to_roman_complex() {
        assert_eq!(int_to_roman(154).unwrap(), "CLIV");
        assert_eq!(int_to_roman(1990).unwrap(), "MCMXC");
        assert_eq!(int_to_roman(2014).unwrap(), "MMXIV");
        assert_eq!(int_to_roman(1009).unwrap(), "MIX");
        assert_eq!(int_to_roman(2500).unwrap(), "MMD");
        assert_eq!(int_to_roman(3999).unwrap(), "MMMCMXCIX");
    }

    #[test]
    fn test_int_to_roman_out_of_range() {
        assert!(int_to_roman(0).is_err());
        assert!(int_to_roman(4000).is_err());
        assert!(int_to_roman(5000).is_err());
    }

    #[test]
    fn test_roundtrip_conversion() {
        // Test that converting to Roman and back gives the same number
        for i in 1..=3999 {
            let roman = int_to_roman(i).unwrap();
            let back = roman_to_int(&roman).unwrap();
            assert_eq!(i, back, "Roundtrip failed for {i}: {roman} -> {back}");
        }
    }

    #[test]
    fn test_all_examples_from_python() {
        // Test cases from the original Python implementation
        let tests = [
            ("III", 3),
            ("CLIV", 154),
            ("MIX", 1009),
            ("MMD", 2500),
            ("MMMCMXCIX", 3999),
        ];

        for (roman, expected) in tests.iter() {
            assert_eq!(roman_to_int(roman).unwrap(), *expected);
            assert_eq!(int_to_roman(*expected).unwrap(), *roman);
        }
    }

    #[test]
    fn test_edge_cases() {
        // Minimum value
        assert_eq!(int_to_roman(1).unwrap(), "I");
        assert_eq!(roman_to_int("I").unwrap(), 1);

        // Maximum value
        assert_eq!(int_to_roman(3999).unwrap(), "MMMCMXCIX");
        assert_eq!(roman_to_int("MMMCMXCIX").unwrap(), 3999);

        // Powers of 10
        assert_eq!(int_to_roman(10).unwrap(), "X");
        assert_eq!(int_to_roman(100).unwrap(), "C");
        assert_eq!(int_to_roman(1000).unwrap(), "M");
    }
}
