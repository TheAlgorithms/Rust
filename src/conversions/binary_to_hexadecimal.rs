// Author : cyrixninja
// Binary to Hex Converter : Converts Binary to Hexadecimal
// Wikipedia References  : 1. https://en.wikipedia.org/wiki/Hexadecimal
//                         2. https://en.wikipedia.org/wiki/Binary_number

static BITS_TO_HEX: &[(u8, &str)] = &[
    (0b0000, "0"),
    (0b0001, "1"),
    (0b0010, "2"),
    (0b0011, "3"),
    (0b0100, "4"),
    (0b0101, "5"),
    (0b0110, "6"),
    (0b0111, "7"),
    (0b1000, "8"),
    (0b1001, "9"),
    (0b1010, "a"),
    (0b1011, "b"),
    (0b1100, "c"),
    (0b1101, "d"),
    (0b1110, "e"),
    (0b1111, "f"),
];

pub fn binary_to_hexadecimal(binary_str: &str) -> String {
    let binary_str = binary_str.trim();

    if binary_str.is_empty() {
        return String::from("Invalid Input");
    }

    let is_negative = binary_str.starts_with('-');
    let binary_str = if is_negative {
        &binary_str[1..]
    } else {
        binary_str
    };

    if !binary_str.chars().all(|c| c == '0' || c == '1') {
        return String::from("Invalid Input");
    }

    let padded_len = (4 - (binary_str.len() % 4)) % 4;
    let binary_str = format!(
        "{:0width$}",
        binary_str,
        width = binary_str.len() + padded_len
    );

    // Convert binary to hexadecimal
    let mut hexadecimal = String::with_capacity(binary_str.len() / 4 + 2);
    hexadecimal.push_str("0x");

    for chunk in binary_str.as_bytes().chunks(4) {
        let mut nibble = 0;
        for (i, &byte) in chunk.iter().enumerate() {
            nibble |= (byte - b'0') << (3 - i);
        }

        let hex_char = BITS_TO_HEX
            .iter()
            .find(|&&(bits, _)| bits == nibble)
            .map(|&(_, hex)| hex)
            .unwrap();
        hexadecimal.push_str(hex_char);
    }

    if is_negative {
        format!("-{hexadecimal}")
    } else {
        hexadecimal
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_string() {
        let input = "";
        let expected = "Invalid Input";
        assert_eq!(binary_to_hexadecimal(input), expected);
    }

    #[test]
    fn test_invalid_binary() {
        let input = "a";
        let expected = "Invalid Input";
        assert_eq!(binary_to_hexadecimal(input), expected);
    }

    #[test]
    fn test_binary() {
        let input = "00110110";
        let expected = "0x36";
        assert_eq!(binary_to_hexadecimal(input), expected);
    }

    #[test]
    fn test_padded_binary() {
        let input = " 1010   ";
        let expected = "0xa";
        assert_eq!(binary_to_hexadecimal(input), expected);
    }
}
