const ERROR_MESSAGE: &str = "Rotation must be in the range [0, 25]";

/// Encrypts a given text using the Caesar cipher technique.
///
/// In cryptography, a Caesar cipher, also known as Caesar's cipher, the shift cipher, Caesar's code,
/// or Caesar shift, is one of the simplest and most widely known encryption techniques.
/// It is a type of substitution cipher in which each letter in the plaintext is replaced by a letter
/// some fixed number of positions down the alphabet.
///
/// # Arguments
///
/// * `text` - The text to be encrypted.
/// * `rotation` - The number of rotations (shift) to be applied. It should be within the range (0, 25).
///
/// # Returns
///
/// Returns a `Result` containing the encrypted string if successful, or an error message if the rotation
/// is out of the valid range.
///
/// # Errors
///
/// Returns an error if the rotation value is out of the valid range (0, 25)
pub fn caesar(text: &str, rotation: isize) -> Result<String, &'static str> {
    if !(0..=25).contains(&rotation) {
        return Err(ERROR_MESSAGE);
    }

    let result = text
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                shift_char(c, rotation)
            } else {
                c
            }
        })
        .collect();

    Ok(result)
}

/// Shifts a single ASCII alphabetic character by a specified number of positions in the alphabet.
///
/// # Arguments
///
/// * `c` - The ASCII alphabetic character to be shifted.
/// * `rotation` - The number of positions to shift the character. Should be within the range (0, 25).
///
/// # Returns
///
/// Returns the shifted ASCII alphabetic character.
fn shift_char(c: char, rotation: isize) -> char {
    let first = if c.is_ascii_lowercase() { b'a' } else { b'A' };
    let rotation = rotation as u8; // Safe cast as rotation is within (0, 25)

    (((c as u8 - first) + rotation) % 26 + first) as char
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_caesar_happy_path {
        ($($name:ident: $test_case:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (text, rotation, expected) = $test_case;

                    // Test forward rotation
                    match caesar(&text, rotation) {
                        Ok(result) => assert_eq!(result, expected),
                        Err(e) => panic!("Unexpected error: {}", e),
                    }

                    // Test backward rotation
                    let backward_rotation = if rotation == 0 { 0 } else { 26 - rotation };
                    match caesar(&expected, backward_rotation) {
                        Ok(result) => assert_eq!(result, text),
                        Err(e) => panic!("Unexpected error: {}", e),
                    }
                }
            )*
        };
    }

    macro_rules! test_caesar_error_cases {
        ($($name:ident: $test_case:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (text, rotation) = $test_case;

                    // If rotation is invalid, test that the function returns the specific error message
                    match caesar(&text, rotation) {
                        Ok(_) => panic!("Expected an error but got an Ok result"),
                        Err(e) => assert_eq!(e, ERROR_MESSAGE),
                    }
                }
            )*
        };
    }

    test_caesar_happy_path! {
        empty_text: ("", 13, ""),
        rot_13: ("rust", 13, "ehfg"),
        unicode: ("attack at dawn 攻", 5, "fyyfhp fy ifbs 攻"),
        rotation_within_alphabet_range: ("Hello, World!", 3, "Khoor, Zruog!"),
        no_rotation: ("Hello, World!", 0, "Hello, World!"),
        rotation_at_alphabet_end: ("Hello, World!", 25, "Gdkkn, Vnqkc!"),
        longer: ("The quick brown fox jumps over the lazy dog.", 5, "Ymj vznhp gwtbs ktc ozrux tajw ymj qfed itl."),
        non_alphabetic_characters: ("12345!@#$%", 3, "12345!@#$%"),
        uppercase_letters: ("ABCDEFGHIJKLMNOPQRSTUVWXYZ", 1, "BCDEFGHIJKLMNOPQRSTUVWXYZA"),
        mixed_case: ("HeLlO WoRlD", 7, "OlSsV DvYsK"),
        with_whitespace: ("Hello, World!", 13, "Uryyb, Jbeyq!"),
        with_special_characters: ("Hello!@#$%^&*()_+World", 4, "Lipps!@#$%^&*()_+Asvph"),
        with_numbers: ("Abcd1234XYZ", 10, "Klmn1234HIJ"),
    }

    test_caesar_error_cases! {
        negative_rotation: ("Hello, World!", -5),
        large_rotation: ("Large rotation", 139),
        large_rotation_with_big_input: ("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Curabitur ac ultrices ante, at gravida ante. Quisque luctus, ligula nec dictum facilisis, elit leo luctus arcu, ut auctor sapien turpis ut mauris. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Nulla vel orci sit amet sem efficitur sagittis a quis augue. Donec semper quam tincidunt hendrerit cursus. Duis placerat gravida diam, in interdum purus dapibus in.", 139),
        very_large_rotation_with_big_input: ("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Curabitur ac ultrices ante, at gravida ante. Quisque luctus, ligula nec dictum facilisis, elit leo luctus arcu, ut auctor sapien turpis ut mauris. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Nulla vel orci sit amet sem efficitur sagittis a quis augue. Donec semper quam tincidunt hendrerit cursus. Duis placerat gravida diam, in interdum purus dapibus in.", 345876),
    }
}
