// Author: NithinU2802
// Octal to Hexadecimal Converter: Converts Octal to Hexadecimal
// Wikipedia References:
// 1. https://en.wikipedia.org/wiki/Octal
// 2. https://en.wikipedia.org/wiki/Hexadecimal

pub fn octal_to_hexadecimal(octal_str: &str) -> Result<String, &'static str> {
    let octal_str = octal_str.trim();

    if octal_str.is_empty() {
        return Err("Empty string");
    }

    // Validate octal string
    if !octal_str.chars().all(|c| ('0'..='7').contains(&c)) {
        return Err("Invalid octal string");
    }

    // Convert octal to decimal first
    let decimal = u64::from_str_radix(octal_str, 8).map_err(|_| "Conversion error")?;

    // Special case for zero
    if decimal == 0 {
        return Ok("0".to_string());
    }

    // Convert decimal to hexadecimal
    Ok(format!("{decimal:X}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_octal_to_hexadecimal() {
        assert_eq!(octal_to_hexadecimal("12"), Ok("A".to_string()));
        assert_eq!(octal_to_hexadecimal("377"), Ok("FF".to_string()));
        assert_eq!(octal_to_hexadecimal("144"), Ok("64".to_string()));
        assert_eq!(octal_to_hexadecimal("0"), Ok("0".to_string()));
    }

    #[test]
    fn test_invalid_input() {
        assert_eq!(octal_to_hexadecimal(""), Err("Empty string"));
        assert_eq!(octal_to_hexadecimal("8"), Err("Invalid octal string"));
        assert_eq!(octal_to_hexadecimal("9"), Err("Invalid octal string"));
        assert_eq!(octal_to_hexadecimal("ABC"), Err("Invalid octal string"));
    }
}
