//! Base32 encoding and decoding implementation.
//!
//! Base32 is a binary-to-text encoding scheme that represents binary data using 32 ASCII characters
//! (A-Z and 2-7). It's commonly used when case-insensitive encoding is needed or when avoiding
//! characters that might be confused (like 0/O or 1/l).
//!
//! This implementation follows the standard Base32 alphabet as defined in RFC 4648.

const B32_CHARSET: &[u8; 32] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";

/// Encodes the given bytes into base32.
///
/// The function converts binary data into base32 format using the standard alphabet.
/// Output is padded with '=' characters to make the length a multiple of 8.
///
/// # Arguments
///
/// * `data` - A byte slice to encode
///
/// # Returns
///
/// A `Vec<u8>` containing the base32-encoded data with padding.
///
/// # Examples
///
/// ```
/// use the_algorithms_rust::ciphers::base32_encode;
/// assert_eq!(base32_encode(b"Hello World!"), b"JBSWY3DPEBLW64TMMQQQ====");
/// assert_eq!(base32_encode(b"123456"), b"GEZDGNBVGY======");
/// assert_eq!(base32_encode(b"some long complex string"), b"ONXW2ZJANRXW4ZZAMNXW24DMMV4CA43UOJUW4ZY=");
/// ```
pub fn base32_encode(data: &[u8]) -> Vec<u8> {
    if data.is_empty() {
        return Vec::new();
    }

    // Convert bytes to binary string representation
    use std::fmt::Write;
    let mut binary_data = String::with_capacity(data.len() * 8);
    for byte in data {
        write!(binary_data, "{byte:08b}").unwrap();
    }

    // Pad binary data to be a multiple of 5 bits
    let padding_needed = (5 - (binary_data.len() % 5)) % 5;
    for _ in 0..padding_needed {
        binary_data.push('0');
    }

    // Convert 5-bit chunks to base32 characters
    let mut result = Vec::new();
    for chunk in binary_data.as_bytes().chunks(5) {
        let chunk_str = std::str::from_utf8(chunk).unwrap();
        let index = usize::from_str_radix(chunk_str, 2).unwrap();
        result.push(B32_CHARSET[index]);
    }

    // Pad result to be a multiple of 8 characters
    while !result.len().is_multiple_of(8) {
        result.push(b'=');
    }

    result
}

/// Decodes base32-encoded data into bytes.
///
/// The function decodes base32 format back to binary data, removing padding characters.
///
/// # Arguments
///
/// * `data` - A byte slice containing base32-encoded data
///
/// # Returns
///
/// * `Ok(Vec<u8>)` - Successfully decoded bytes
/// * `Err(String)` - Error message if the input is invalid
///
/// # Errors
///
/// Returns an error if:
/// - The input contains invalid base32 characters
/// - The input cannot be properly decoded
///
/// # Examples
///
/// ```
/// use the_algorithms_rust::ciphers::base32_decode;
/// assert_eq!(base32_decode(b"JBSWY3DPEBLW64TMMQQQ====").unwrap(), b"Hello World!");
/// assert_eq!(base32_decode(b"GEZDGNBVGY======").unwrap(), b"123456");
/// assert_eq!(base32_decode(b"ONXW2ZJANRXW4ZZAMNXW24DMMV4CA43UOJUW4ZY=").unwrap(), b"some long complex string");
/// ```
pub fn base32_decode(data: &[u8]) -> Result<Vec<u8>, String> {
    if data.is_empty() {
        return Ok(Vec::new());
    }

    // Remove padding and convert to string
    let data_str =
        std::str::from_utf8(data).map_err(|_| "Invalid UTF-8 in base32 data".to_string())?;
    let data_stripped = data_str.trim_end_matches('=');

    // Convert base32 characters to binary string
    use std::fmt::Write;
    let mut binary_chunks = String::with_capacity(data_stripped.len() * 5);
    for ch in data_stripped.chars() {
        // Find the index of this character in the charset
        let index = B32_CHARSET
            .iter()
            .position(|&c| c == ch as u8)
            .ok_or_else(|| format!("Invalid base32 character: {ch}"))?;

        // Convert index to 5-bit binary string
        write!(binary_chunks, "{index:05b}").unwrap();
    }

    // Convert 8-bit chunks back to bytes
    let mut result = Vec::new();
    for chunk in binary_chunks.as_bytes().chunks(8) {
        if chunk.len() == 8 {
            let chunk_str = std::str::from_utf8(chunk).unwrap();
            let byte_value = u8::from_str_radix(chunk_str, 2)
                .map_err(|_| "Failed to parse binary chunk".to_string())?;
            result.push(byte_value);
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_hello_world() {
        assert_eq!(base32_encode(b"Hello World!"), b"JBSWY3DPEBLW64TMMQQQ====");
    }

    #[test]
    fn test_encode_numbers() {
        assert_eq!(base32_encode(b"123456"), b"GEZDGNBVGY======");
    }

    #[test]
    fn test_encode_long_string() {
        assert_eq!(
            base32_encode(b"some long complex string"),
            b"ONXW2ZJANRXW4ZZAMNXW24DMMV4CA43UOJUW4ZY="
        );
    }

    #[test]
    fn test_encode_empty() {
        assert_eq!(base32_encode(b""), b"");
    }

    #[test]
    fn test_encode_single_char() {
        assert_eq!(base32_encode(b"A"), b"IE======");
    }

    #[test]
    fn test_decode_hello_world() {
        assert_eq!(
            base32_decode(b"JBSWY3DPEBLW64TMMQQQ====").unwrap(),
            b"Hello World!"
        );
    }

    #[test]
    fn test_decode_numbers() {
        assert_eq!(base32_decode(b"GEZDGNBVGY======").unwrap(), b"123456");
    }

    #[test]
    fn test_decode_long_string() {
        assert_eq!(
            base32_decode(b"ONXW2ZJANRXW4ZZAMNXW24DMMV4CA43UOJUW4ZY=").unwrap(),
            b"some long complex string"
        );
    }

    #[test]
    fn test_decode_empty() {
        assert_eq!(base32_decode(b"").unwrap(), b"");
    }

    #[test]
    fn test_decode_single_char() {
        assert_eq!(base32_decode(b"IE======").unwrap(), b"A");
    }

    #[test]
    fn test_decode_without_padding() {
        assert_eq!(
            base32_decode(b"JBSWY3DPEBLW64TMMQQQ").unwrap(),
            b"Hello World!"
        );
    }

    #[test]
    fn test_decode_invalid_character() {
        let result = base32_decode(b"INVALID!@#$");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid base32 character"));
    }

    #[test]
    fn test_roundtrip_hello() {
        let original = b"Hello";
        let encoded = base32_encode(original);
        let decoded = base32_decode(&encoded).unwrap();
        assert_eq!(decoded, original);
    }

    #[test]
    fn test_roundtrip_various_strings() {
        let test_cases = vec![
            b"a" as &[u8],
            b"ab",
            b"abc",
            b"abcd",
            b"abcde",
            b"The quick brown fox jumps over the lazy dog",
            b"1234567890",
            b"!@#$%^&*()",
        ];

        for original in test_cases {
            let encoded = base32_encode(original);
            let decoded = base32_decode(&encoded).unwrap();
            assert_eq!(decoded, original, "Failed for: {:?}", original);
        }
    }

    #[test]
    fn test_all_charset_characters() {
        // Test that all characters in the charset can be encoded/decoded
        for i in 0..32 {
            let data = vec![i * 8]; // Arbitrary byte values
            let encoded = base32_encode(&data);
            let decoded = base32_decode(&encoded).unwrap();
            assert_eq!(decoded, data);
        }
    }

    #[test]
    fn test_binary_data() {
        let binary_data = vec![0x00, 0x01, 0x02, 0xFF, 0xFE, 0xFD];
        let encoded = base32_encode(&binary_data);
        let decoded = base32_decode(&encoded).unwrap();
        assert_eq!(decoded, binary_data);
    }

    #[test]
    fn test_padding_variations() {
        // Test different amounts of padding
        let test_cases: Vec<(&[u8], &[u8])> = vec![
            (b"f", b"MY======"),
            (b"fo", b"MZXQ===="),
            (b"foo", b"MZXW6==="),
            (b"foob", b"MZXW6YQ="),
            (b"fooba", b"MZXW6YTB"),
            (b"foobar", b"MZXW6YTBOI======"),
        ];

        for (input, expected) in test_cases {
            let encoded = base32_encode(input);
            assert_eq!(encoded, expected, "Encoding failed for: {:?}", input);
            let decoded = base32_decode(&encoded).unwrap();
            assert_eq!(decoded, input, "Roundtrip failed for: {:?}", input);
        }
    }
}
