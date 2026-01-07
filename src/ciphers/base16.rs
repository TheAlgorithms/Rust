//! Base16 encoding and decoding implementation.
//!
//! Base16, also known as hexadecimal encoding, represents binary data using 16 ASCII characters
//! (0-9 and A-F). Each byte is represented by exactly two hexadecimal digits.
//!
//! This implementation follows RFC 3548 Section 6 specifications:
//! - Uses uppercase characters (A-F) for encoding
//! - Requires uppercase input for decoding
//! - Validates that encoded data has an even number of characters

/// Encodes the given bytes into base16 (hexadecimal) format.
///
/// Each byte is converted to two uppercase hexadecimal characters.
///
/// # Arguments
///
/// * `data` - A byte slice to encode
///
/// # Returns
///
/// A `String` containing the uppercase hexadecimal representation of the input data.
///
/// # Examples
///
/// ```
/// use the_algorithms_rust::ciphers::base16_encode;
/// assert_eq!(base16_encode(b"Hello World!"), "48656C6C6F20576F726C6421");
/// assert_eq!(base16_encode(b"HELLO WORLD!"), "48454C4C4F20574F524C4421");
/// assert_eq!(base16_encode(b""), "");
/// ```
pub fn base16_encode(data: &[u8]) -> String {
    use std::fmt::Write;
    data.iter().fold(String::new(), |mut output, byte| {
        write!(output, "{byte:02X}").unwrap();
        output
    })
}

/// Decodes base16 (hexadecimal) encoded data into bytes.
///
/// This function validates the input according to RFC 3548 Section 6:
/// - The data must have an even number of characters
/// - The data must only contain uppercase hexadecimal characters (0-9, A-F)
///
/// # Arguments
///
/// * `data` - A string slice containing uppercase hexadecimal characters
///
/// # Returns
///
/// * `Ok(Vec<u8>)` - Successfully decoded bytes
/// * `Err(String)` - Error message if the input is invalid
///
/// # Errors
///
/// Returns an error if:
/// - The input has an odd number of characters
/// - The input contains characters other than 0-9 and A-F
/// - The input contains lowercase hexadecimal characters (a-f)
///
/// # Examples
///
/// ```
/// use the_algorithms_rust::ciphers::base16_decode;
/// assert_eq!(base16_decode("48656C6C6F20576F726C6421").unwrap(), b"Hello World!");
/// assert_eq!(base16_decode("48454C4C4F20574F524C4421").unwrap(), b"HELLO WORLD!");
/// assert_eq!(base16_decode("").unwrap(), b"");
/// ```
///
/// Invalid inputs return errors:
///
/// ```
/// use the_algorithms_rust::ciphers::base16_decode;
/// assert!(base16_decode("486").is_err()); // Odd number of characters
/// assert!(base16_decode("48656c6c6f20576f726c6421").is_err()); // Lowercase hex
/// assert!(base16_decode("This is not base16 encoded data.").is_err()); // Invalid characters
/// ```
pub fn base16_decode(data: &str) -> Result<Vec<u8>, String> {
    // Check if data has an even number of characters
    if !data.len().is_multiple_of(2) {
        return Err("Base16 encoded data is invalid:\n\
             Data does not have an even number of hex digits."
            .to_string());
    }

    // Check if all characters are valid uppercase hexadecimal (0-9, A-F)
    // This follows RFC 3548 section 6 which specifies uppercase
    if !data
        .chars()
        .all(|c| c.is_ascii_hexdigit() && !c.is_lowercase())
    {
        return Err("Base16 encoded data is invalid:\n\
             Data is not uppercase hex or it contains invalid characters."
            .to_string());
    }

    // Decode pairs of hexadecimal characters into bytes
    let mut result = Vec::with_capacity(data.len() / 2);
    for i in (0..data.len()).step_by(2) {
        let hex_pair = &data[i..i + 2];
        match u8::from_str_radix(hex_pair, 16) {
            Ok(byte) => result.push(byte),
            Err(_) => {
                return Err("Base16 encoded data is invalid:\n\
                     Failed to decode hex pair."
                    .to_string())
            }
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_hello_world() {
        assert_eq!(base16_encode(b"Hello World!"), "48656C6C6F20576F726C6421");
    }

    #[test]
    fn test_encode_hello_world_uppercase() {
        assert_eq!(base16_encode(b"HELLO WORLD!"), "48454C4C4F20574F524C4421");
    }

    #[test]
    fn test_encode_empty() {
        assert_eq!(base16_encode(b""), "");
    }

    #[test]
    fn test_encode_special_characters() {
        assert_eq!(base16_encode(b"\x00\x01\xFF"), "0001FF");
    }

    #[test]
    fn test_encode_all_bytes() {
        let data: Vec<u8> = (0..=255).collect();
        let encoded = base16_encode(&data);
        assert_eq!(encoded.len(), 512); // 256 bytes * 2 hex chars each
    }

    #[test]
    fn test_decode_hello_world() {
        assert_eq!(
            base16_decode("48656C6C6F20576F726C6421").unwrap(),
            b"Hello World!"
        );
    }

    #[test]
    fn test_decode_hello_world_uppercase() {
        assert_eq!(
            base16_decode("48454C4C4F20574F524C4421").unwrap(),
            b"HELLO WORLD!"
        );
    }

    #[test]
    fn test_decode_empty() {
        assert_eq!(base16_decode("").unwrap(), b"");
    }

    #[test]
    fn test_decode_special_characters() {
        assert_eq!(base16_decode("0001FF").unwrap(), b"\x00\x01\xFF");
    }

    #[test]
    fn test_decode_odd_length() {
        let result = base16_decode("486");
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("does not have an even number of hex digits"));
    }

    #[test]
    fn test_decode_lowercase_hex() {
        let result = base16_decode("48656c6c6f20576f726c6421");
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("not uppercase hex or it contains invalid characters"));
    }

    #[test]
    fn test_decode_invalid_characters() {
        let result = base16_decode("This is not base16 encoded data.");
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("not uppercase hex or it contains invalid characters"));
    }

    #[test]
    fn test_decode_mixed_case() {
        let result = base16_decode("48656C6c6F"); // Mixed upper and lowercase
        assert!(result.is_err());
    }

    #[test]
    fn test_roundtrip() {
        let original = b"The quick brown fox jumps over the lazy dog";
        let encoded = base16_encode(original);
        let decoded = base16_decode(&encoded).unwrap();
        assert_eq!(decoded, original);
    }

    #[test]
    fn test_roundtrip_all_bytes() {
        let original: Vec<u8> = (0..=255).collect();
        let encoded = base16_encode(&original);
        let decoded = base16_decode(&encoded).unwrap();
        assert_eq!(decoded, original);
    }

    #[test]
    fn test_roundtrip_empty() {
        let original = b"";
        let encoded = base16_encode(original);
        let decoded = base16_decode(&encoded).unwrap();
        assert_eq!(decoded, original);
    }
}
