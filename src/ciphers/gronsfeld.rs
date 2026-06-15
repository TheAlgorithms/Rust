//! Gronsfeld Cipher
//!
//! # Algorithm
//!
//! A variant of the Vigenere cipher where the key is a sequence of digits (0–9).
//! Each alphabetic character in the plaintext is shifted forward (encrypt) or
//! backward (decrypt) by the value of the corresponding key digit, cycling
//! through the key. Non-alphabetic characters are passed through unchanged.

const ALPHABET_LEN: u8 = 26;

const ERR_EMPTY_KEY: &str = "Key must not be empty";
const ERR_INVALID_KEY: &str = "Key must contain only digits (0-9)";

fn validate_key(key: &str) -> Result<Vec<u8>, &'static str> {
    if key.is_empty() {
        return Err(ERR_EMPTY_KEY);
    }

    key.bytes()
        .map(|b| match b {
            b'0'..=b'9' => Ok(b - b'0'),
            _ => Err(ERR_INVALID_KEY),
        })
        .collect()
}

fn shift_char(c: char, shift: u8, forward: bool) -> char {
    let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
    let pos = c as u8 - base;
    let shifted = if forward {
        (pos + shift) % ALPHABET_LEN
    } else {
        (pos + ALPHABET_LEN - shift % ALPHABET_LEN) % ALPHABET_LEN
    };
    (base + shifted) as char
}

fn process(text: &str, key: &[u8], forward: bool) -> String {
    let key_len = key.len();
    let mut key_index = 0;
    text.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let result = shift_char(c, key[key_index % key_len], forward);
                key_index += 1;
                result
            } else {
                c
            }
        })
        .collect()
}

/// Encrypts `text` using the Gronsfeld cipher with the given digit `key`.
pub fn gronsfeld_encrypt(text: &str, key: &str) -> Result<String, &'static str> {
    let digits = validate_key(key)?;
    Ok(process(text, &digits, true))
}

/// Decrypts `text` using the Gronsfeld cipher with the given digit `key`.
pub fn gronsfeld_decrypt(text: &str, key: &str) -> Result<String, &'static str> {
    let digits = validate_key(key)?;
    Ok(process(text, &digits, false))
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- validate_key ---

    #[test]
    fn empty_key_returns_error() {
        assert_eq!(gronsfeld_encrypt("hello", ""), Err(ERR_EMPTY_KEY));
        assert_eq!(gronsfeld_decrypt("hello", ""), Err(ERR_EMPTY_KEY));
    }

    #[test]
    fn non_digit_key_returns_error() {
        assert_eq!(gronsfeld_encrypt("hello", "12a3"), Err(ERR_INVALID_KEY));
        assert_eq!(gronsfeld_decrypt("hello", "abc"), Err(ERR_INVALID_KEY));
    }

    // --- encrypt ---

    #[test]
    fn encrypt_empty_text() {
        assert_eq!(gronsfeld_encrypt("", "123").unwrap(), "");
    }

    #[test]
    fn encrypt_basic() {
        assert_eq!(gronsfeld_encrypt("abc", "123").unwrap(), "bdf");
    }

    #[test]
    fn encrypt_preserves_case() {
        assert_eq!(gronsfeld_encrypt("ABC", "123").unwrap(), "BDF");
    }

    #[test]
    fn encrypt_mixed_case() {
        assert_eq!(gronsfeld_encrypt("aAbB", "12").unwrap(), "bCcD");
    }

    #[test]
    fn encrypt_passthrough_non_alpha() {
        assert_eq!(gronsfeld_encrypt("a b,c!", "123").unwrap(), "b d,f!");
    }

    #[test]
    fn encrypt_key_wraps() {
        assert_eq!(gronsfeld_encrypt("abcd", "12").unwrap(), "bddf");
    }

    #[test]
    fn encrypt_zero_shift() {
        assert_eq!(gronsfeld_encrypt("hello", "0").unwrap(), "hello");
    }

    #[test]
    fn encrypt_wraps_around_alphabet() {
        assert_eq!(gronsfeld_encrypt("z", "1").unwrap(), "a");
        assert_eq!(gronsfeld_encrypt("Z", "9").unwrap(), "I");
    }

    #[test]
    fn encrypt_single_digit_key() {
        assert_eq!(
            gronsfeld_encrypt("Hello, World!", "5").unwrap(),
            "Mjqqt, Btwqi!"
        );
    }

    // --- decrypt ---

    #[test]
    fn decrypt_empty_text() {
        assert_eq!(gronsfeld_decrypt("", "123").unwrap(), "");
    }

    #[test]
    fn decrypt_basic() {
        assert_eq!(gronsfeld_decrypt("bdf", "123").unwrap(), "abc");
    }

    #[test]
    fn decrypt_preserves_case() {
        assert_eq!(gronsfeld_decrypt("BDF", "123").unwrap(), "ABC");
    }

    #[test]
    fn decrypt_passthrough_non_alpha() {
        assert_eq!(gronsfeld_decrypt("b d,f!", "123").unwrap(), "a b,c!");
    }

    #[test]
    fn decrypt_zero_shift() {
        assert_eq!(gronsfeld_decrypt("hello", "0").unwrap(), "hello");
    }

    #[test]
    fn decrypt_wraps_around_alphabet() {
        // 'a' - 1 = 'z'
        assert_eq!(gronsfeld_decrypt("a", "1").unwrap(), "z");
    }

    // --- round-trip ---

    #[test]
    fn roundtrip_basic() {
        let plain = "Hello, World!";
        let key = "31415";
        let encrypted = gronsfeld_encrypt(plain, key).unwrap();
        assert_eq!(gronsfeld_decrypt(&encrypted, key).unwrap(), plain);
    }

    #[test]
    fn roundtrip_long_text() {
        let plain = "The quick brown fox jumps over the lazy dog.";
        let key = "9876543210";
        let encrypted = gronsfeld_encrypt(plain, key).unwrap();
        assert_eq!(gronsfeld_decrypt(&encrypted, key).unwrap(), plain);
    }

    #[test]
    fn roundtrip_with_unicode_passthrough() {
        let plain = "Rust 2024";
        let key = "42";
        let encrypted = gronsfeld_encrypt(plain, key).unwrap();
        assert_eq!(gronsfeld_decrypt(&encrypted, key).unwrap(), plain);
    }
}
