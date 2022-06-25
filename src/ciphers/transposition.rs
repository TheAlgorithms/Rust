//! Transposition Cipher
//!
//! The Transposition Cipher is a method of encryption by which a message is shifted
//! according to a regular system, so that the ciphertext is a rearrangement of the
//! original message. The most commonly referred to Transposition Cipher is the
//! COLUMNAR TRANSPOSITION cipher, which is demonstrated below.

use std::ops::Range;

/// Encrypts or decrypts a message, using multiple keys. The
/// encryption is based on the columnar transposition method.
pub fn transposition(decrypt_mode: bool, msg: &str, key: &str) -> String {
    let key_uppercase: String = key.to_uppercase();
    let mut cipher_msg: String = msg.to_string();

    let keys: Vec<&str> = match decrypt_mode {
        false => key_uppercase.split_whitespace().collect(),
        true => key_uppercase.split_whitespace().rev().collect(),
    };

    for cipher_key in keys.iter() {
        let mut key_order: Vec<usize> = Vec::new();
        let mut counter: u8 = 0;

        // Removes any non-alphabet characters from 'msg'
        cipher_msg = cipher_msg
            .to_uppercase()
            .chars()
            .filter(|&c| c.is_ascii_alphabetic())
            .collect();

        // Determines the sequence of the columns, as dictated by the
        // alphabetical order of the keyword's letters
        let mut key_ascii: Vec<(usize, u8)> =
            cipher_key.bytes().enumerate().collect::<Vec<(usize, u8)>>();

        key_ascii.sort_by_key(|&(_, key)| key);

        key_ascii.iter_mut().for_each(|(_, key)| {
            *key = counter;
            counter += 1;
        });

        key_ascii.sort_by_key(|&(index, _)| index);

        key_ascii
            .into_iter()
            .for_each(|(_, key)| key_order.push(key.into()));

        // Determines whether to encrypt or decrypt the message,
        // and returns the result
        cipher_msg = match decrypt_mode {
            false => encrypt(cipher_msg, key_order),
            true => decrypt(cipher_msg, key_order),
        };
    }

    cipher_msg
}

/// Performs the columnar transposition encryption
fn encrypt(mut msg: String, key_order: Vec<usize>) -> String {
    let mut encrypted_msg: String = String::from("");
    let mut encrypted_vec: Vec<String> = Vec::new();

    let msg_len: usize = msg.len();
    let key_len: usize = key_order.len();

    let mut msg_index: usize = msg_len;
    let mut key_index: usize = key_len;

    // Loop each column, pushing it to a Vec<T>
    while !msg.is_empty() {
        let mut chars: String = String::from("");
        let mut index: usize = 0;
        key_index -= 1;

        // Loop every nth character, determined by key length, to create a column
        while index < msg_index {
            let ch: char = msg.remove(index);
            chars.push(ch);

            index += key_index;
            msg_index -= 1;
        }

        encrypted_vec.push(chars);
    }

    // Concatenate the columns into a string, determined by the
    // alphabetical order of the keyword's characters
    let mut indexed_vec: Vec<(usize, &String)> = Vec::new();
    let mut indexed_msg: String = String::from("");
    let mut counter: usize = 0;

    key_order.into_iter().for_each(|key_index| {
        indexed_vec.push((key_index, &encrypted_vec[counter]));
        counter += 1;
    });

    indexed_vec.sort();

    indexed_vec.into_iter().for_each(|(_, column)| {
        indexed_msg.push_str(column);
    });

    // Split the message by a space every nth character, determined by
    // 'message length divided by keyword length' to the next highest integer.
    let msg_div: usize = (msg_len as f32 / key_len as f32).ceil() as usize;
    let mut counter: usize = 0;

    indexed_msg.chars().for_each(|c| {
        encrypted_msg.push(c);
        counter += 1;
        if counter == msg_div {
            encrypted_msg.push(' ');
            counter = 0;
        }
    });

    encrypted_msg.trim_end().to_string()
}

/// Performs the columnar transposition decryption
fn decrypt(mut msg: String, key_order: Vec<usize>) -> String {
    let mut decrypted_msg: String = String::from("");
    let mut decrypted_vec: Vec<String> = Vec::new();
    let mut indexed_vec: Vec<(usize, String)> = Vec::new();

    let msg_len: usize = msg.len();
    let key_len: usize = key_order.len();

    // Split the message into columns, determined by 'message length divided by keyword length'.
    // Some columns are larger by '+1', where the prior calculation leaves a remainder.
    let split_size: usize = (msg_len as f64 / key_len as f64) as usize;
    let msg_mod: usize = msg_len % key_len;
    let mut counter: usize = msg_mod;

    let mut key_split: Vec<usize> = key_order.clone();
    let (split_large, split_small) = key_split.split_at_mut(msg_mod);

    split_large.sort_unstable();
    split_small.sort_unstable();

    split_large.iter_mut().rev().for_each(|key_index| {
        counter -= 1;
        let range: Range<usize> =
            ((*key_index * split_size) + counter)..(((*key_index + 1) * split_size) + counter + 1);

        let slice: String = msg[range.clone()].to_string();
        indexed_vec.push((*key_index, slice));

        msg.replace_range(range, "");
    });

    split_small.iter_mut().for_each(|key_index| {
        let (slice, rest_of_msg) = msg.split_at(split_size);
        indexed_vec.push((*key_index, (slice.to_string())));
        msg = rest_of_msg.to_string();
    });

    indexed_vec.sort();

    key_order.into_iter().for_each(|key| {
        if let Some((_, column)) = indexed_vec.iter().find(|(key_index, _)| key_index == &key) {
            decrypted_vec.push(column.to_string());
        }
    });

    // Concatenate the columns into a string, determined by the
    // alphabetical order of the keyword's characters
    for _ in 0..split_size {
        decrypted_vec.iter_mut().for_each(|column| {
            decrypted_msg.push(column.remove(0));
        })
    }

    if !decrypted_vec.is_empty() {
        decrypted_vec.into_iter().for_each(|chars| {
            decrypted_msg.push_str(&chars);
        })
    }

    decrypted_msg
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encryption() {
        assert_eq!(
            transposition(
                false,
                "The quick brown fox jumps over the lazy dog",
                "Archive",
            ),
            "TKOOL ERJEZ CFSEG QOURY UWMTD HBXVA INPHO"
        );

        assert_eq!(
            transposition(
                false,
                "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ.,/;'[]{}:|_+=-`~() ",
                "Tenacious"
            ),
            "DMVENW ENWFOX BKTCLU FOXGPY CLUDMV GPYHQZ IRAJSA JSBKTH QZIR"
        );

        assert_eq!(
            transposition(false, "WE ARE DISCOVERED. FLEE AT ONCE.", "ZEBRAS"),
            "EVLNA CDTES EAROF ODEEC WIREE"
        );
    }

    #[test]
    fn decryption() {
        assert_eq!(
            transposition(true, "TKOOL ERJEZ CFSEG QOURY UWMTD HBXVA INPHO", "Archive"),
            "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG"
        );

        assert_eq!(
            transposition(
                true,
                "DMVENW ENWFOX BKTCLU FOXGPY CLUDMV GPYHQZ IRAJSA JSBKTH QZIR",
                "Tenacious"
            ),
            "ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ"
        );

        assert_eq!(
            transposition(true, "EVLNA CDTES EAROF ODEEC WIREE", "ZEBRAS"),
            "WEAREDISCOVEREDFLEEATONCE"
        );
    }

    #[test]
    fn double_encryption() {
        assert_eq!(
            transposition(
                false,
                "The quick brown fox jumps over the lazy dog",
                "Archive Snow"
            ),
            "KEZEUWHAH ORCGRMBIO TLESOUDVP OJFQYTXN"
        );

        assert_eq!(
            transposition(
                false,
                "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ.,/;'[]{}:|_+=-`~() ",
                "Tenacious Drink"
            ),
            "DWOCXLGZSKI VNBUPDYRJHN FTOCVQJBZEW KFYMHASQMEX LGUPIATR"
        );

        assert_eq!(
            transposition(false, "WE ARE DISCOVERED. FLEE AT ONCE.", "ZEBRAS STRIPE"),
            "CAEEN SOIAE DRLEF WEDRE EVTOC"
        );
    }

    #[test]
    fn double_decryption() {
        assert_eq!(
            transposition(
                true,
                "KEZEUWHAH ORCGRMBIO TLESOUDVP OJFQYTXN",
                "Archive Snow"
            ),
            "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG"
        );

        assert_eq!(
            transposition(
                true,
                "DWOCXLGZSKI VNBUPDYRJHN FTOCVQJBZEW KFYMHASQMEX LGUPIATR",
                "Tenacious Drink",
            ),
            "ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ"
        );

        assert_eq!(
            transposition(true, "CAEEN SOIAE DRLEF WEDRE EVTOC", "ZEBRAS STRIPE"),
            "WEAREDISCOVEREDFLEEATONCE"
        );
    }
}
