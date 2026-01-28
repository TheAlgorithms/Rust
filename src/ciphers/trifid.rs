//! The Trifid cipher uses a table to fractionate each plaintext letter into a trigram,
//! mixes the constituents of the trigrams, and then applies the table in reverse to turn
//! these mixed trigrams into ciphertext letters.
//!
//! [Wikipedia reference](https://en.wikipedia.org/wiki/Trifid_cipher)

use std::collections::HashMap;

type CharToNum = HashMap<char, String>;
type NumToChar = HashMap<String, char>;
type PrepareResult = Result<(String, String, CharToNum, NumToChar), String>;

const TRIGRAM_VALUES: [&str; 27] = [
    "111", "112", "113", "121", "122", "123", "131", "132", "133", "211", "212", "213", "221",
    "222", "223", "231", "232", "233", "311", "312", "313", "321", "322", "323", "331", "332",
    "333",
];

/// Encrypts a message using the Trifid cipher.
///
/// # Arguments
///
/// * `message` - The message to encrypt
/// * `alphabet` - The characters to be used for the cipher (must be 27 characters)
/// * `period` - The number of characters in a group whilst encrypting
pub fn trifid_encrypt(message: &str, alphabet: &str, period: usize) -> Result<String, String> {
    let (message, _alphabet, char_to_num, num_to_char) = prepare(message, alphabet)?;

    let mut encrypted_numeric = String::new();
    let chars: Vec<char> = message.chars().collect();

    for chunk in chars.chunks(period) {
        let chunk_str: String = chunk.iter().collect();
        encrypted_numeric.push_str(&encrypt_part(&chunk_str, &char_to_num));
    }

    let mut encrypted = String::new();
    let numeric_chars: Vec<char> = encrypted_numeric.chars().collect();

    for chunk in numeric_chars.chunks(3) {
        let trigram: String = chunk.iter().collect();
        if let Some(ch) = num_to_char.get(&trigram) {
            encrypted.push(*ch);
        }
    }

    Ok(encrypted)
}

/// Decrypts a Trifid cipher encrypted message.
///
/// # Arguments
///
/// * `message` - The message to decrypt
/// * `alphabet` - The characters used for the cipher (must be 27 characters)
/// * `period` - The number of characters used in grouping when it was encrypted
pub fn trifid_decrypt(message: &str, alphabet: &str, period: usize) -> Result<String, String> {
    let (message, _alphabet, char_to_num, num_to_char) = prepare(message, alphabet)?;

    let mut decrypted_numeric = Vec::new();
    let chars: Vec<char> = message.chars().collect();

    for chunk in chars.chunks(period) {
        let chunk_str: String = chunk.iter().collect();
        let (a, b, c) = decrypt_part(&chunk_str, &char_to_num);

        for i in 0..a.len() {
            let trigram = format!(
                "{}{}{}",
                a.chars().nth(i).unwrap(),
                b.chars().nth(i).unwrap(),
                c.chars().nth(i).unwrap()
            );
            decrypted_numeric.push(trigram);
        }
    }

    let mut decrypted = String::new();
    for trigram in decrypted_numeric {
        if let Some(ch) = num_to_char.get(&trigram) {
            decrypted.push(*ch);
        }
    }

    Ok(decrypted)
}

/// Arranges the trigram value of each letter of message_part vertically and joins
/// them horizontally.
fn encrypt_part(message_part: &str, char_to_num: &CharToNum) -> String {
    let mut one = String::new();
    let mut two = String::new();
    let mut three = String::new();

    for ch in message_part.chars() {
        if let Some(trigram) = char_to_num.get(&ch) {
            let chars: Vec<char> = trigram.chars().collect();
            one.push(chars[0]);
            two.push(chars[1]);
            three.push(chars[2]);
        }
    }

    format!("{one}{two}{three}")
}

/// Converts each letter of the input string into their respective trigram values,
/// joins them and splits them into three equal groups of strings which are returned.
fn decrypt_part(message_part: &str, char_to_num: &CharToNum) -> (String, String, String) {
    let mut this_part = String::new();

    for ch in message_part.chars() {
        if let Some(trigram) = char_to_num.get(&ch) {
            this_part.push_str(trigram);
        }
    }

    let part_len = message_part.len();

    if part_len == 0 {
        return (String::new(), String::new(), String::new());
    }

    let chars: Vec<char> = this_part.chars().collect();

    let mut result = Vec::new();
    for chunk in chars.chunks(part_len) {
        result.push(chunk.iter().collect::<String>());
    }

    // Ensure we have exactly 3 parts, pad with empty strings if necessary
    while result.len() < 3 {
        result.push(String::new());
    }

    (result[0].clone(), result[1].clone(), result[2].clone())
}

/// Prepares the message and alphabet for encryption/decryption.
/// Validates inputs and creates the character-to-number and number-to-character mappings.
fn prepare(message: &str, alphabet: &str) -> PrepareResult {
    // Remove spaces and convert to uppercase
    let alphabet: String = alphabet.chars().filter(|c| !c.is_whitespace()).collect();
    let alphabet = alphabet.to_uppercase();
    let message: String = message.chars().filter(|c| !c.is_whitespace()).collect();
    let message = message.to_uppercase();

    // Validate alphabet length
    if alphabet.len() != 27 {
        return Err("Length of alphabet has to be 27.".to_string());
    }

    // Validate that all message characters are in the alphabet
    for ch in message.chars() {
        if !alphabet.contains(ch) {
            return Err("Each message character has to be included in alphabet!".to_string());
        }
    }

    // Create character-to-number mapping
    let mut char_to_num = HashMap::new();
    let mut num_to_char = HashMap::new();

    for (i, ch) in alphabet.chars().enumerate() {
        let trigram = TRIGRAM_VALUES[i].to_string();
        char_to_num.insert(ch, trigram.clone());
        num_to_char.insert(trigram, ch);
    }

    Ok((message, alphabet, char_to_num, num_to_char))
}

#[cfg(test)]
mod tests {
    use super::*;

    const DEFAULT_ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ.";

    #[test]
    fn test_encrypt_basic() {
        let result = trifid_encrypt("I am a boy", DEFAULT_ALPHABET, 5);
        assert_eq!(result, Ok("BCDGBQY".to_string()));
    }

    #[test]
    fn test_encrypt_empty() {
        let result = trifid_encrypt(" ", DEFAULT_ALPHABET, 5);
        assert_eq!(result, Ok("".to_string()));
    }

    #[test]
    fn test_encrypt_custom_alphabet() {
        let result = trifid_encrypt(
            "aide toi le c iel ta id era",
            "FELIXMARDSTBCGHJKNOPQUVWYZ+",
            5,
        );
        assert_eq!(result, Ok("FMJFVOISSUFTFPUFEQQC".to_string()));
    }

    #[test]
    fn test_decrypt_basic() {
        let result = trifid_decrypt("BCDGBQY", DEFAULT_ALPHABET, 5);
        assert_eq!(result, Ok("IAMABOY".to_string()));
    }

    #[test]
    fn test_decrypt_custom_alphabet() {
        let result = trifid_decrypt("FMJFVOISSUFTFPUFEQQC", "FELIXMARDSTBCGHJKNOPQUVWYZ+", 5);
        assert_eq!(result, Ok("AIDETOILECIELTAIDERA".to_string()));
    }

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let msg = "DEFEND THE EAST WALL OF THE CASTLE.";
        let alphabet = "EPSDUCVWYM.ZLKXNBTFGORIJHAQ";
        let encrypted = trifid_encrypt(msg, alphabet, 5).unwrap();
        let decrypted = trifid_decrypt(&encrypted, alphabet, 5).unwrap();
        assert_eq!(decrypted, msg.replace(' ', ""));
    }

    #[test]
    fn test_invalid_alphabet_length() {
        let result = trifid_encrypt("test", "ABCDEFGHIJKLMNOPQRSTUVW", 5);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Length of alphabet has to be 27.");
    }

    #[test]
    fn test_invalid_character_in_message() {
        let result = trifid_encrypt("am i a boy?", "ABCDEFGHIJKLMNOPQRSTUVWXYZ+", 5);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Each message character has to be included in alphabet!"
        );
    }

    #[test]
    fn test_encrypt_part() {
        let mut char_to_num = HashMap::new();
        char_to_num.insert('A', "111".to_string());
        char_to_num.insert('S', "311".to_string());
        char_to_num.insert('K', "212".to_string());

        let result = encrypt_part("ASK", &char_to_num);
        assert_eq!(result, "132111112");
    }

    #[test]
    fn test_decrypt_part() {
        let mut char_to_num = HashMap::new();
        for (i, ch) in DEFAULT_ALPHABET.chars().enumerate() {
            char_to_num.insert(ch, TRIGRAM_VALUES[i].to_string());
        }

        let (a, b, c) = decrypt_part("ABCDE", &char_to_num);
        assert_eq!(a, "11111");
        assert_eq!(b, "21131");
        assert_eq!(c, "21122");
    }

    #[test]
    fn test_decrypt_part_single_char() {
        let mut char_to_num = HashMap::new();
        char_to_num.insert('A', "111".to_string());

        let (a, b, c) = decrypt_part("A", &char_to_num);
        assert_eq!(a, "1");
        assert_eq!(b, "1");
        assert_eq!(c, "1");
    }

    #[test]
    fn test_decrypt_part_empty() {
        let char_to_num = HashMap::new();
        let (a, b, c) = decrypt_part("", &char_to_num);
        assert_eq!(a, "");
        assert_eq!(b, "");
        assert_eq!(c, "");
    }

    #[test]
    fn test_decrypt_part_with_unmapped_chars() {
        let mut char_to_num = HashMap::new();
        char_to_num.insert('A', "111".to_string());
        // 'B' and 'C' are not in the mapping, so this_part will only contain A's trigram
        // With message_part length of 3, chunks will be size 3, giving us one chunk "111"
        // The padding logic will add two empty strings
        let (a, b, c) = decrypt_part("ABC", &char_to_num);
        assert_eq!(a, "111");
        assert_eq!(b, "");
        assert_eq!(c, "");
    }
}
