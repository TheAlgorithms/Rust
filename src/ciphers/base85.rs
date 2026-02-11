//! Base85 (Ascii85) encoding and decoding
//!
//! Ascii85 is a form of binary-to-text encoding developed by Adobe Systems.
//! It encodes 4 bytes into 5 ASCII characters from the range 33-117 ('!' to 'u').
//!
//! # References
//! - [Wikipedia: Ascii85](https://en.wikipedia.org/wiki/Ascii85)

/// Converts a base-10 number to base-85 representation
fn base10_to_85(mut d: u32) -> String {
    if d == 0 {
        return String::new();
    }

    let mut result = String::new();
    while d > 0 {
        result.push((d % 85 + 33) as u8 as char);
        d /= 85;
    }
    result
}

/// Converts base-85 digits to a base-10 number
fn base85_to_10(digits: &[u8]) -> u32 {
    digits
        .iter()
        .rev()
        .enumerate()
        .map(|(i, &ch)| (ch as u32) * 85_u32.pow(i as u32))
        .sum()
}

/// Encodes binary data using Base85 encoding
///
/// # Arguments
/// * `data` - The binary data to encode
///
/// # Returns
/// * `Vec<u8>` - The Base85 encoded data
///
/// # Examples
/// ```
/// use the_algorithms_rust::ciphers::base85_encode;
///
/// assert_eq!(base85_encode(b""), b"");
/// assert_eq!(base85_encode(b"12345"), b"0etOA2#");
/// assert_eq!(base85_encode(b"base 85"), b"@UX=h+?24");
/// ```
pub fn base85_encode(data: &[u8]) -> Vec<u8> {
    if data.is_empty() {
        return Vec::new();
    }

    // Convert input bytes to binary string
    let mut binary_data = String::new();
    for &byte in data {
        use std::fmt::Write;
        write!(&mut binary_data, "{byte:08b}").unwrap();
    }

    // Calculate padding needed to make length a multiple of 32
    let remainder = binary_data.len() % 32;
    let null_values = if remainder == 0 {
        0
    } else {
        (32 - remainder) / 8
    };

    // Pad binary data to multiple of 32 bits
    while !binary_data.len().is_multiple_of(32) {
        binary_data.push('0');
    }

    // Split into 32-bit chunks and convert to base-85
    let mut result = String::new();
    for chunk in binary_data.as_bytes().chunks(32) {
        let chunk_str = std::str::from_utf8(chunk).unwrap();
        let value = u32::from_str_radix(chunk_str, 2).unwrap();
        let mut encoded = base10_to_85(value);

        // Reverse the string (as per original Python logic)
        encoded = encoded.chars().rev().collect();
        result.push_str(&encoded);
    }

    // Remove padding characters if necessary
    if null_values % 4 != 0 {
        let trim_len = result.len() - null_values;
        result.truncate(trim_len);
    }

    result.into_bytes()
}

/// Decodes Base85 encoded data back to binary
///
/// # Arguments
/// * `data` - The Base85 encoded data to decode
///
/// # Returns
/// * `Vec<u8>` - The decoded binary data
///
/// # Examples
/// ```
/// use the_algorithms_rust::ciphers::base85_decode;
///
/// assert_eq!(base85_decode(b""), b"");
/// assert_eq!(base85_decode(b"0etOA2#"), b"12345");
/// assert_eq!(base85_decode(b"@UX=h+?24"), b"base 85");
/// ```
pub fn base85_decode(data: &[u8]) -> Vec<u8> {
    if data.is_empty() {
        return Vec::new();
    }

    // Calculate padding needed
    let remainder = data.len() % 5;
    let null_values = if remainder == 0 { 0 } else { 5 - remainder };

    // Create padded data
    let mut padded_data = data.to_vec();
    padded_data.extend(std::iter::repeat_n(b'u', null_values));

    // Process in 5-byte chunks
    let mut results = Vec::new();
    for chunk in padded_data.chunks(5) {
        // Convert ASCII characters to base-85 digits
        let b85_segment: Vec<u8> = chunk.iter().map(|&b| b - 33).collect();

        // Convert base-85 to base-10
        let value = base85_to_10(&b85_segment);

        // Convert to binary string (32 bits)
        let binary = format!("{value:032b}");
        results.push(binary);
    }

    // Convert binary strings to characters
    let mut char_chunks = Vec::new();
    for binary_str in results {
        for byte_str in binary_str.as_bytes().chunks(8) {
            let byte_string = std::str::from_utf8(byte_str).unwrap();
            let byte_value = u8::from_str_radix(byte_string, 2).unwrap();
            char_chunks.push(byte_value);
        }
    }

    // Calculate offset for trimming
    let offset = if null_values % 5 == 0 {
        0
    } else {
        -(null_values as isize)
    };
    let result_len = if offset < 0 {
        (char_chunks.len() as isize + offset) as usize
    } else {
        char_chunks.len()
    };

    char_chunks.truncate(result_len);
    char_chunks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_empty() {
        assert_eq!(base85_encode(b""), b"");
    }

    #[test]
    fn test_encode_12345() {
        assert_eq!(base85_encode(b"12345"), b"0etOA2#");
    }

    #[test]
    fn test_encode_base85() {
        assert_eq!(base85_encode(b"base 85"), b"@UX=h+?24");
    }

    #[test]
    fn test_decode_empty() {
        assert_eq!(base85_decode(b""), b"");
    }

    #[test]
    fn test_decode_12345() {
        assert_eq!(base85_decode(b"0etOA2#"), b"12345");
    }

    #[test]
    fn test_decode_base85() {
        assert_eq!(base85_decode(b"@UX=h+?24"), b"base 85");
    }

    #[test]
    fn test_encode_decode_roundtrip() {
        let test_cases = vec![
            b"Hello, World!".to_vec(),
            b"The quick brown fox".to_vec(),
            b"Rust".to_vec(),
            b"a".to_vec(),
        ];

        for test_case in test_cases {
            let encoded = base85_encode(&test_case);
            let decoded = base85_decode(&encoded);
            assert_eq!(decoded, test_case);
        }
    }
}
