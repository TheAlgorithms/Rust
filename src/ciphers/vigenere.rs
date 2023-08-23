//! Vigenère Cipher
//!
//! # Algorithm
//!
//! Rotate each ascii character by the offset of the corresponding key character.
//! When we reach the last key character, we start over from the first one.
//! This implementation does not rotate unicode characters.

/// Vigenère cipher to rotate plain_text text by key and return an owned String.
pub fn vigenere(plain_text: &str, key: &str) -> String {
    // Remove all unicode and non-ascii characters from key
    let key: String = key.chars().filter(|&c| c.is_ascii_alphabetic()).collect();
    let key = key.to_ascii_lowercase();

    let key_len = key.len();
    if key_len == 0 {
        return String::from(plain_text);
    }

    let mut index = 0;

    plain_text
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let first = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                let shift = key.as_bytes()[index % key_len] - b'a';
                index += 1;
                // modulo the distance to keep character range
                (first + (c as u8 + shift - first) % 26) as char
            } else {
                c
            }
        })
        .collect()
}


mod vignere {
    pub fn encrypt(key: &str, plaintext: &str) -> String {
        // encrypt
        let key_it = key.bytes().cycle();
        String::from_utf8(
            std::iter::zip(plaintext.bytes(), key_it)
                .map(|(p, k)| {
                    let p = p - b'a';
                    let k = k - b'a';
                    let c = (p + k) % 26;
                    c + b'a'
                })
                .collect::<Vec<_>>(),
        )
        .unwrap()
    }
    pub fn decrypt(key: &str, ciphertext: &str) -> String {
        let key_it = key.bytes().cycle();
        String::from_utf8(
            std::iter::zip(ciphertext.bytes(), key_it)
                .map(|(p, k)| {
                    let p = p - b'a';
                    let k = k - b'a';
                    let c = (26 + p - k) % 26;
                    c + b'a'
                })
                .collect(),
        )
        .unwrap()
    }


}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(vigenere("", "test"), "");
    }

    #[test]
    fn vigenere_base() {
        assert_eq!(
            vigenere("LoremIpsumDolorSitAmet", "base"),
            "MojinIhwvmVsmojWjtSqft"
        );
    }

    #[test]
    fn vigenere_with_spaces() {
        assert_eq!(
            vigenere(
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
                "spaces"
            ),
            "Ddrgq ahhuo hgddr uml sbev, ggfheexwljr chahxsemfy tlkx."
        );
    }

    #[test]
    fn vigenere_unicode_and_numbers() {
        assert_eq!(
            vigenere("1 Lorem ⏳ ipsum dolor sit amet Ѡ", "unicode"),
            "1 Fbzga ⏳ ltmhu fcosl fqv opin Ѡ"
        );
    }

    #[test]
    fn vigenere_unicode_key() {
        assert_eq!(
            vigenere("Lorem ipsum dolor sit amet", "😉 key!"),
            "Vspoq gzwsw hmvsp cmr kqcd"
        );
    }

    #[test]
    fn vigenere_empty_key() {
        assert_eq!(vigenere("Lorem ipsum", ""), "Lorem ipsum");
    }

    #[test]
    fn test_vig() {
      let msg = "aoeuidhtnsqjkxbmwvzpyfgcrl";
      let key = "averygoodkey";
      
      let ciphertext = vignere::encrypt(key,msg);
      
      let plaintext = vignere::decrypt(key, &ciphertext);
      assert_eq!(msg, plaintext);
    
  }
}
