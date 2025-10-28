// Author: NithinU2802
// Hexadecimal to Octal Converter: Converts Hexadecimal to Octal
// Wikipedia References:
// 1. https://en.wikipedia.org/wiki/Hexadecimal
// 2. https://en.wikipedia.org/wiki/Octal

pub fn hexadecimal_to_octal(hex_str: &str) -> Result<String, &'static str> {
    let hex_str = hex_str.trim();

    if hex_str.is_empty() {
        return Err("Empty string");
    }

    // Validate hexadecimal string
    if !hex_str.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err("Invalid hexadecimal string");
    }

    // Convert hex to decimal first
    let decimal = u64::from_str_radix(hex_str, 16).map_err(|_| "Conversion error")?;

    // Then convert decimal to octal
    if decimal == 0 {
        return Ok("0".to_string());
    }

    let mut num = decimal;
    let mut octal = String::new();

    while num > 0 {
        let remainder = num % 8;
        octal.push_str(&remainder.to_string());
        num /= 8;
    }

    // Reverse the string to get the correct octal representation
    Ok(octal.chars().rev().collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hexadecimal_to_octal() {
        assert_eq!(hexadecimal_to_octal("A"), Ok("12".to_string()));
        assert_eq!(hexadecimal_to_octal("FF"), Ok("377".to_string()));
        assert_eq!(hexadecimal_to_octal("64"), Ok("144".to_string()));
        assert_eq!(hexadecimal_to_octal("0"), Ok("0".to_string()));
    }

    #[test]
    fn test_invalid_input() {
        assert_eq!(hexadecimal_to_octal(""), Err("Empty string"));
        assert_eq!(
            hexadecimal_to_octal("GG"),
            Err("Invalid hexadecimal string")
        );
        assert_eq!(
            hexadecimal_to_octal("XYZ"),
            Err("Invalid hexadecimal string")
        );
    }
}
