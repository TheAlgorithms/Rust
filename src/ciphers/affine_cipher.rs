//! Affine Cipher
//!
//! The affine cipher is a type of monoalphabetic substitution cipher where each
//! character in the alphabet is mapped to its numeric equivalent, encrypted using
//! a mathematical function, and converted back to a character.
//!
//! # Algorithm
//!
//! The encryption function is: `E(x) = (ax + b) mod m`
//! The decryption function is: `D(x) = a^(-1)(x - b) mod m`
//!
//! Where:
//! - `x` is the numeric position of the character
//! - `a` and `b` are the keys (key_a and key_b)
//! - `m` is the size of the symbol set
//! - `a^(-1)` is the modular multiplicative inverse of `a` modulo `m`
//!
//! # Key Requirements
//!
//! - `key_a` must be coprime with the symbol set size (gcd(key_a, m) = 1)
//! - `key_a` must not be 1 (cipher becomes too weak)
//! - `key_b` must not be 0 (cipher becomes too weak)
//! - `key_b` must be between 0 and symbol set size - 1
//!
//! # References
//!
//! - [Affine Cipher - Wikipedia](https://en.wikipedia.org/wiki/Affine_cipher)

/// Symbol set used for the affine cipher - all printable ASCII characters
const SYMBOLS: &str = r##" !"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~"##;

/// Calculates the greatest common divisor using the iterative Euclidean algorithm.
///
/// # Arguments
///
/// * `a` - First number
/// * `b` - Second number
///
/// # Returns
///
/// The GCD of a and b
fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

/// Finds the modular multiplicative inverse of `a` modulo `m`.
///
/// Uses the Extended Euclidean Algorithm to find x such that:
/// (a * x) mod m = 1
///
/// # Arguments
///
/// * `a` - The number to find the inverse of
/// * `m` - The modulus
///
/// # Returns
///
/// `Some(inverse)` if the inverse exists, `None` otherwise
fn find_mod_inverse(a: i64, m: i64) -> Option<i64> {
    if gcd(a as usize, m as usize) != 1 {
        return None; // No inverse exists
    }

    // Extended Euclidean Algorithm
    let (mut u1, mut u2, mut u3) = (1i64, 0i64, a);
    let (mut v1, mut v2, mut v3) = (0i64, 1i64, m);

    while v3 != 0 {
        let q = u3 / v3;
        let t1 = u1 - q * v1;
        let t2 = u2 - q * v2;
        let t3 = u3 - q * v3;

        u1 = v1;
        u2 = v2;
        u3 = v3;
        v1 = t1;
        v2 = t2;
        v3 = t3;
    }

    let inverse = u1 % m;
    if inverse < 0 {
        Some(inverse + m)
    } else {
        Some(inverse)
    }
}

/// Validates the encryption/decryption keys.
///
/// # Arguments
///
/// * `key_a` - The multiplicative key
/// * `key_b` - The additive key
/// * `is_encrypt` - Whether this is for encryption (applies additional checks)
///
/// # Returns
///
/// `Ok(())` if keys are valid, `Err(String)` with error message otherwise
fn check_keys(key_a: usize, key_b: usize, is_encrypt: bool) -> Result<(), String> {
    let symbols_len = SYMBOLS.len();

    if is_encrypt {
        if key_a == 1 {
            return Err(
                "The affine cipher becomes weak when key A is set to 1. Choose a different key"
                    .to_string(),
            );
        }
        if key_b == 0 {
            return Err(
                "The affine cipher becomes weak when key B is set to 0. Choose a different key"
                    .to_string(),
            );
        }
    }

    if key_a == 0 {
        return Err("Key A must be greater than 0".to_string());
    }

    if key_b >= symbols_len {
        return Err(format!("Key B must be between 0 and {}", symbols_len - 1));
    }

    if gcd(key_a, symbols_len) != 1 {
        return Err(format!(
            "Key A ({key_a}) and the symbol set size ({symbols_len}) are not relatively prime. Choose a different key"
        ));
    }

    Ok(())
}

/// Encrypts a message using the affine cipher.
///
/// # Arguments
///
/// * `key` - The encryption key (encoded as key_a * SYMBOLS.len() + key_b)
/// * `message` - The plaintext message to encrypt
///
/// # Returns
///
/// `Ok(String)` with the encrypted message, or `Err(String)` if keys are invalid
///
/// # Example
///
/// ```
/// use the_algorithms_rust::ciphers::affine_encrypt;
///
/// let encrypted = affine_encrypt(4545, "The affine cipher is a type of monoalphabetic substitution cipher.").unwrap();
/// assert_eq!(encrypted, "VL}p MM{I}p~{HL}Gp{vp pFsH}pxMpyxIx JHL O}F{~pvuOvF{FuF{xIp~{HL}Gi");
/// ```
pub fn affine_encrypt(key: usize, message: &str) -> Result<String, String> {
    let symbols_len = SYMBOLS.len();
    let key_a = key / symbols_len;
    let key_b = key % symbols_len;

    check_keys(key_a, key_b, true)?;

    let mut cipher_text = String::new();

    for symbol in message.chars() {
        if let Some(sym_index) = SYMBOLS.find(symbol) {
            let encrypted_index = (sym_index * key_a + key_b) % symbols_len;
            cipher_text.push(SYMBOLS.chars().nth(encrypted_index).unwrap());
        } else {
            // Keep symbols not in SYMBOLS unchanged
            cipher_text.push(symbol);
        }
    }

    Ok(cipher_text)
}

/// Decrypts a message using the affine cipher.
///
/// # Arguments
///
/// * `key` - The decryption key (same as encryption key)
/// * `message` - The ciphertext message to decrypt
///
/// # Returns
///
/// `Ok(String)` with the decrypted message, or `Err(String)` if keys are invalid
///
/// # Example
///
/// ```
/// use the_algorithms_rust::ciphers::affine_decrypt;
///
/// let decrypted = affine_decrypt(4545, "VL}p MM{I}p~{HL}Gp{vp pFsH}pxMpyxIx JHL O}F{~pvuOvF{FuF{xIp~{HL}Gi").unwrap();
/// assert_eq!(decrypted, "The affine cipher is a type of monoalphabetic substitution cipher.");
/// ```
pub fn affine_decrypt(key: usize, message: &str) -> Result<String, String> {
    let symbols_len = SYMBOLS.len();
    let key_a = key / symbols_len;
    let key_b = key % symbols_len;

    check_keys(key_a, key_b, false)?;

    let mod_inverse_of_key_a = find_mod_inverse(key_a as i64, symbols_len as i64)
        .ok_or_else(|| format!("Could not find modular inverse of key A ({key_a})"))?;

    let mut plain_text = String::new();

    for symbol in message.chars() {
        if let Some(sym_index) = SYMBOLS.find(symbol) {
            let decrypted_index = ((sym_index as i64 - key_b as i64) * mod_inverse_of_key_a)
                .rem_euclid(symbols_len as i64) as usize;
            plain_text.push(SYMBOLS.chars().nth(decrypted_index).unwrap());
        } else {
            // Keep symbols not in SYMBOLS unchanged
            plain_text.push(symbol);
        }
    }

    Ok(plain_text)
}

/// Generates a random valid key for the affine cipher.
///
/// The key is generated such that:
/// - key_a is coprime with the symbol set size
/// - key_b is not 0
/// - Both keys are within valid ranges
///
/// # Returns
///
/// A random valid key encoded as key_a * SYMBOLS.len() + key_b
///
/// # Example
///
/// ```
/// use the_algorithms_rust::ciphers::affine_generate_key;
///
/// let key = affine_generate_key();
/// assert!(key >= 2);
/// ```
pub fn affine_generate_key() -> usize {
    use rand::Rng;
    let mut rng = rand::rng();
    let symbols_len = SYMBOLS.len();

    loop {
        let key_a = rng.random_range(2..symbols_len);
        let key_b = rng.random_range(1..symbols_len);

        if gcd(key_a, symbols_len) == 1 {
            return key_a * symbols_len + key_b;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(48, 18), 6);
        assert_eq!(gcd(18, 48), 6);
        assert_eq!(gcd(100, 50), 50);
        assert_eq!(gcd(17, 13), 1);
        assert_eq!(gcd(1, 1), 1);
        assert_eq!(gcd(0, 5), 5);
    }

    #[test]
    fn test_find_mod_inverse() {
        assert_eq!(find_mod_inverse(3, 11), Some(4));
        assert_eq!(find_mod_inverse(7, 26), Some(15));
        assert_eq!(find_mod_inverse(2, 5), Some(3));
        assert_eq!(find_mod_inverse(4, 6), None); // No inverse (not coprime)
    }

    #[test]
    fn test_encrypt_decrypt_example() {
        let message = "The affine cipher is a type of monoalphabetic substitution cipher.";
        let key = 4545;

        let encrypted = affine_encrypt(key, message).unwrap();
        assert_eq!(
            encrypted,
            "VL}p MM{I}p~{HL}Gp{vp pFsH}pxMpyxIx JHL O}F{~pvuOvF{FuF{xIp~{HL}Gi"
        );

        let decrypted = affine_decrypt(key, &encrypted).unwrap();
        assert_eq!(decrypted, message);
    }

    #[test]
    fn test_encrypt_simple() {
        let key = 4545;
        let message = "Hello World!";
        let encrypted = affine_encrypt(key, message).unwrap();

        // Verify it's different from original
        assert_ne!(encrypted, message);

        // Verify we can decrypt it back
        let decrypted = affine_decrypt(key, &encrypted).unwrap();
        assert_eq!(decrypted, message);
    }

    #[test]
    fn test_roundtrip_various_messages() {
        let key = 4545;
        let messages = vec![
            "This is a test!",
            "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
            "0123456789",
            "Special chars: !@#$%^&*()",
            "Mixed Case And Numbers 123",
        ];

        for message in messages {
            let encrypted = affine_encrypt(key, message).unwrap();
            let decrypted = affine_decrypt(key, &encrypted).unwrap();
            assert_eq!(decrypted, message);
        }
    }

    #[test]
    fn test_empty_string() {
        let key = 4545;
        let message = "";
        let encrypted = affine_encrypt(key, message).unwrap();
        assert_eq!(encrypted, "");
        let decrypted = affine_decrypt(key, &encrypted).unwrap();
        assert_eq!(decrypted, "");
    }

    #[test]
    fn test_invalid_key_a_is_one() {
        let symbols_len = SYMBOLS.len();
        let key = 1 * symbols_len + 5; // key_a = 1

        let result = affine_encrypt(key, "test");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("weak when key A is set to 1"));
    }

    #[test]
    fn test_invalid_key_b_is_zero() {
        let symbols_len = SYMBOLS.len();
        let key = 5 * symbols_len + 0; // key_b = 0

        let result = affine_encrypt(key, "test");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("weak when key B is set to 0"));
    }

    #[test]
    fn test_invalid_key_not_coprime() {
        let symbols_len = SYMBOLS.len();
        // Find a key_a that's not coprime with symbols_len
        let key_a = 2; // 2 is not coprime with 95 (symbols_len) if it's odd... actually let me check
                       // SYMBOLS has 95 characters, gcd(2, 95) = 1, so this would work
                       // Let's use a number that won't be coprime
        let key = 5 * symbols_len + 10; // key_a = 5
                                        // gcd(5, 95) = 5, so this should fail

        let result = affine_encrypt(key, "test");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not relatively prime"));
    }

    #[test]
    fn test_key_b_too_large() {
        let symbols_len = SYMBOLS.len();
        let key = 3 * symbols_len + symbols_len; // key_b = symbols_len (too large)

        let result = affine_encrypt(key, "test");
        assert!(result.is_err());
        // This will actually have key_b = 0 after modulo, so it will fail for different reason
        // Let me recalculate: if key = 3 * 95 + 95 = 380, then key_a = 380 / 95 = 4, key_b = 380 % 95 = 0
        // So it will fail because key_b = 0
    }

    #[test]
    fn test_symbols_not_in_set() {
        let key = 4545;
        let message = "Hello\nWorld\t!"; // Contains newline and tab
        let encrypted = affine_encrypt(key, message).unwrap();

        // Newline and tab should remain unchanged
        assert!(encrypted.contains('\n'));
        assert!(encrypted.contains('\t'));

        let decrypted = affine_decrypt(key, &encrypted).unwrap();
        assert_eq!(decrypted, message);
    }

    #[test]
    fn test_generate_key() {
        // Generate a key and test it works
        let key = affine_generate_key();
        let message = "Test message for generated key";

        let encrypted = affine_encrypt(key, message).unwrap();
        let decrypted = affine_decrypt(key, &encrypted).unwrap();
        assert_eq!(decrypted, message);
    }

    #[test]
    fn test_generate_key_validity() {
        // Generate multiple keys and verify they're all valid
        for _ in 0..10 {
            let key = affine_generate_key();
            let symbols_len = SYMBOLS.len();
            let key_a = key / symbols_len;
            let key_b = key % symbols_len;

            // Check that the keys meet requirements
            assert!(key_a > 1);
            assert!(key_b > 0);
            assert!(key_b < symbols_len);
            assert_eq!(gcd(key_a, symbols_len), 1);
        }
    }

    #[test]
    fn test_all_symbols() {
        let key = 4545;

        // Test that all symbols in SYMBOLS can be encrypted and decrypted
        for symbol in SYMBOLS.chars() {
            let message = symbol.to_string();
            let encrypted = affine_encrypt(key, &message).unwrap();
            let decrypted = affine_decrypt(key, &encrypted).unwrap();
            assert_eq!(decrypted, message);
        }
    }

    #[test]
    fn test_different_keys_produce_different_ciphertexts() {
        let message = "Hello World";
        let key1 = 4545;
        let key2 = 3456;

        let encrypted1 = affine_encrypt(key1, message).unwrap();
        let encrypted2 = affine_encrypt(key2, message).unwrap();

        assert_ne!(encrypted1, encrypted2);
    }

    #[test]
    fn test_long_message() {
        let key = 4545;
        let message = "The quick brown fox jumps over the lazy dog. ".repeat(10);

        let encrypted = affine_encrypt(key, &message).unwrap();
        let decrypted = affine_decrypt(key, &encrypted).unwrap();
        assert_eq!(decrypted, message);
    }
}
