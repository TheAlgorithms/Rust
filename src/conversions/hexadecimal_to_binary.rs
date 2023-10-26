// Author : cyrixninja
// Hexadecimal to Binary Converter : Converts Hexadecimal to Binary
// Wikipedia References  : 1. https://en.wikipedia.org/wiki/Hexadecimal
//                         2. https://en.wikipedia.org/wiki/Binary_number
// Other References for Testing : https://www.rapidtables.com/convert/number/hex-to-binary.html

pub fn hexadecimal_to_binary(hex_str: &str) -> Result<String, String> {
    let hex_chars = hex_str.chars().collect::<Vec<char>>();
    let mut binary = String::new();

    for c in hex_chars {
        let bin_rep = match c {
            '0' => "0000",
            '1' => "0001",
            '2' => "0010",
            '3' => "0011",
            '4' => "0100",
            '5' => "0101",
            '6' => "0110",
            '7' => "0111",
            '8' => "1000",
            '9' => "1001",
            'a' | 'A' => "1010",
            'b' | 'B' => "1011",
            'c' | 'C' => "1100",
            'd' | 'D' => "1101",
            'e' | 'E' => "1110",
            'f' | 'F' => "1111",
            _ => return Err("Invalid".to_string()),
        };
        binary.push_str(bin_rep);
    }

    Ok(binary)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_string() {
        let input = "";
        let expected = Ok("".to_string());
        assert_eq!(hexadecimal_to_binary(input), expected);
    }

    #[test]
    fn test_hexadecimal() {
        let input = "1a2";
        let expected = Ok("000110100010".to_string());
        assert_eq!(hexadecimal_to_binary(input), expected);
    }
    #[test]
    fn test_hexadecimal2() {
        let input = "1b3";
        let expected = Ok("000110110011".to_string());
        assert_eq!(hexadecimal_to_binary(input), expected);
    }

    #[test]
    fn test_invalid_hexadecimal() {
        let input = "1g3";
        let expected = Err("Invalid".to_string());
        assert_eq!(hexadecimal_to_binary(input), expected);
    }
}
