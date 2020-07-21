//! XOR Cipher
//!
//! # Algorithm
//!
//! Compute each character as XOR of the original character with a given key.
//! This implementation does not rotate unicode characters.

/// XOR cipher by computing each character as XOR with a given key and return an owned String.
pub fn xor_cipher(plain_text: &str, key: u8) -> String {
    plain_text
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                (c as u8 ^ key) as char
            } else {
                c
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(xor_cipher("", 42), "");
    }
    
    #[test]
    fn ascii_encrypt() {
        assert_eq!(xor_cipher("abcd", 42), "KHIN");
    }
    
    #[test]
    fn ascii_decrypt() {
        assert_eq!(xor_cipher("KHIN", 42), "abcd");
    }
    
    #[test]
    fn unicode_encrypt() {
        assert_eq!(xor_cipher("😉ciao", 42), "😉ICKE");
    }

}
