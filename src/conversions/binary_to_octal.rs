// Author: NithinU2802
// Binary to Octal Converter: Converts Binary to Octal
// Wikipedia References:
// 1. https://en.wikipedia.org/wiki/Binary_number
// 2. https://en.wikipedia.org/wiki/Octal

pub fn binary_to_octal(binary_str: &str) -> Result<String, &'static str> {
    // Validate input
    let binary_str = binary_str.trim();
    if binary_str.is_empty() {
        return Err("Empty string");
    }

    if !binary_str.chars().all(|c| c == '0' || c == '1') {
        return Err("Invalid binary string");
    }

    // Pad the binary string with zeros to make its length a multiple of 3
    let padding_length = (3 - (binary_str.len() % 3)) % 3;
    let padded_binary = "0".repeat(padding_length) + binary_str;

    // Convert every 3 binary digits to one octal digit
    let mut octal = String::new();
    for chunk in padded_binary.chars().collect::<Vec<char>>().chunks(3) {
        let binary_group: String = chunk.iter().collect();
        let decimal = u8::from_str_radix(&binary_group, 2).map_err(|_| "Conversion error")?;
        octal.push_str(&decimal.to_string());
    }

    Ok(octal)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_to_octal() {
        assert_eq!(binary_to_octal("1010"), Ok("12".to_string()));
        assert_eq!(binary_to_octal("1111"), Ok("17".to_string()));
        assert_eq!(binary_to_octal("11111111"), Ok("377".to_string()));
        assert_eq!(binary_to_octal("1100100"), Ok("144".to_string()));
    }

    #[test]
    fn test_invalid_input() {
        assert_eq!(binary_to_octal(""), Err("Empty string"));
        assert_eq!(binary_to_octal("12"), Err("Invalid binary string"));
        assert_eq!(binary_to_octal("abc"), Err("Invalid binary string"));
    }
}
